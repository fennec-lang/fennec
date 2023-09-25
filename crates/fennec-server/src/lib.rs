// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use anyhow::{anyhow, Context};
use fennec_common::{types, MODULE_ROOT_FILENAME, PROJECT_NAME};
use lsp_types::{notification::Notification, request::Request};
use std::path::PathBuf;

const FILE_SCHEME: &str = "file";

pub struct Server {
    conn: lsp_server::Connection,
    io_threads: lsp_server::IoThreads,
    request_id: i32,

    // from LSP InitializeParams
    workspace_folders: Vec<PathBuf>,
    pub utf8_pos: bool,                 // TODO: remove pub, use
    pub parent_process_id: Option<u32>, // TODO: remove pub, exit when parent is dead
}

impl Server {
    pub fn new_stdio(version: &str) -> Result<Server, anyhow::Error> {
        let (conn, io_threads) = lsp_server::Connection::stdio();

        let (id, init_params) = conn
            .initialize_start()
            .context("failed to wait for InitializeParams")?;
        let init_params: lsp_types::InitializeParams = serde_json::from_value(init_params)
            .context("failed to deserialize InitializeParams")?;
        if log::log_enabled!(log::Level::Debug) {
            let init_pretty =
                serde_json::to_string_pretty(&init_params).unwrap_or_else(|e| e.to_string());
            log::debug!("InitializeParams: {init_pretty}");
        }

        let dyn_watch = cap_fs_watch_dynamic(&init_params);
        if !dyn_watch {
            return Err(anyhow!("Fennec LSP server requires client to support dynamic registration in DidChangeWatchedFilesClientCapabilities"));
        }

        let utf8_pos = cap_utf8_positions(&init_params);
        let folders = workspace_roots(&init_params);

        let init_result = lsp_types::InitializeResult {
            capabilities: lsp_types::ServerCapabilities {
                position_encoding: if utf8_pos {
                    Some(lsp_types::PositionEncodingKind::UTF8)
                } else {
                    None
                },
                ..Default::default()
            },
            server_info: Some(lsp_types::ServerInfo {
                name: PROJECT_NAME.to_owned(),
                version: Some(version.to_owned()),
            }),
        };
        let init_result =
            serde_json::to_value(init_result).context("failed to serialize InitializeResult")?;

        conn.initialize_finish(id, init_result)
            .context("failed to send InitializeResult")?;

        Ok(Server {
            conn,
            io_threads,
            request_id: 0,
            utf8_pos,
            workspace_folders: folders,
            parent_process_id: init_params.process_id,
        })
    }

    pub fn join(self) -> Result<(), anyhow::Error> {
        self.io_threads.join()?;
        Ok(())
    }

    fn next_id(&mut self) -> lsp_server::RequestId {
        let id = self.request_id;
        self.request_id += 1;
        lsp_server::RequestId::from(id)
    }

    pub fn run(&mut self, state: &types::State) -> Result<(), anyhow::Error> {
        let reg_id = self.next_id();
        let mut registered_root_watchers = false;
        register_module_root_watchers(&self.conn, reg_id.clone()).context(format!(
            "failed to register {MODULE_ROOT_FILENAME} watchers"
        ))?;

        for msg in &self.conn.receiver {
            match msg {
                lsp_server::Message::Request(req) => {
                    if self.conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    if !registered_root_watchers {
                        let lsp_server::Request { id, method, .. } = req;
                        let msg = format!(
                            r#"got "{method}" (id {id}) request before root watchers were registered, ignoring"#
                        );
                        log::warn!("{msg}");
                        let _ = self.conn.sender.send(
                            lsp_server::Response::new_err(
                                id,
                                lsp_server::ErrorCode::ContentModified as i32,
                                msg,
                            )
                            .into(),
                        );
                    }
                }
                lsp_server::Message::Response(resp) => {
                    if resp.id == reg_id {
                        if let Some(lsp_server::ResponseError { code, message, .. }) = resp.error {
                            return Err(anyhow!("failed to register {MODULE_ROOT_FILENAME} watchers: [{code}] {message}"));
                        }
                        registered_root_watchers = true;

                        // We find the roots only after watchers are registered to avoid possible races
                        // where we would miss new roots that appeared after the walk is complete but
                        // before the watch is set up.
                        let roots = find_module_roots(&self.workspace_folders);
                        state.signal_new_roots(roots);
                    }
                }
                lsp_server::Message::Notification(lsp_server::Notification { method, .. }) => {
                    if !registered_root_watchers {
                        log::warn!(
                            r#"got "{method}" notification before root watchers were registered, ignoring"#
                        );
                    }

                    // TODO: react on new module roots
                    log::info!(r#"got "{method}" notification"#);
                }
            }
        }
        Ok(())
    }
}

fn cap_fs_watch_dynamic(init_params: &lsp_types::InitializeParams) -> bool {
    if let Some(ref workspace_caps) = init_params.capabilities.workspace {
        if let Some(ref change_watched) = workspace_caps.did_change_watched_files {
            return change_watched.dynamic_registration.unwrap_or(false);
        }
    }
    false
}

fn cap_utf8_positions(init_params: &lsp_types::InitializeParams) -> bool {
    if let Some(ref general_caps) = init_params.capabilities.general {
        if let Some(ref encodings) = general_caps.position_encodings {
            return encodings.contains(&lsp_types::PositionEncodingKind::UTF8);
        }
    }
    false
}

fn workspace_roots(init_params: &lsp_types::InitializeParams) -> Vec<PathBuf> {
    if let Some(ref wf) = init_params.workspace_folders {
        return wf
            .iter()
            .filter(|f| f.uri.scheme() == FILE_SCHEME)
            .filter_map(|f| f.uri.to_file_path().ok())
            .collect();
    }
    init_params
        .root_uri
        .iter()
        .filter(|uri| uri.scheme() == FILE_SCHEME)
        .filter_map(|uri| uri.to_file_path().ok())
        .collect()
}

fn register_module_root_watchers(
    conn: &lsp_server::Connection,
    id: lsp_server::RequestId,
) -> Result<(), anyhow::Error> {
    let opts = lsp_types::DidChangeWatchedFilesRegistrationOptions {
        watchers: vec![lsp_types::FileSystemWatcher {
            glob_pattern: lsp_types::GlobPattern::String(format!("**/{MODULE_ROOT_FILENAME}")),
            kind: None, // create + change + delete
        }],
    };
    let opts = serde_json::to_value(opts)
        .context("failed to serialize DidChangeWatchedFilesRegistrationOptions")?;

    let params = lsp_types::RegistrationParams {
        registrations: vec![lsp_types::Registration {
            id: MODULE_ROOT_FILENAME.to_owned(),
            method: lsp_types::notification::DidChangeWatchedFiles::METHOD.to_owned(),
            register_options: Some(opts),
        }],
    };
    let params = serde_json::to_value(params).context("failed to serialize RegistrationParams")?;

    let req = lsp_server::Request {
        id,
        method: lsp_types::request::RegisterCapability::METHOD.to_owned(),
        params,
    };
    conn.sender
        .send(req.into())
        .context("failed to send client/registerCapability request")?;

    Ok(())
}

fn find_module_roots(workspace_folders: &Vec<PathBuf>) -> Vec<types::AbsolutePath> {
    let mut roots: Vec<types::AbsolutePath> = Vec::with_capacity(workspace_folders.len());
    for folder in workspace_folders {
        let walker = walkdir::WalkDir::new(folder).into_iter();
        for entry in walker.filter_entry(is_valid_utf8_visible) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file()
                        && entry.file_name().to_str() == Some(MODULE_ROOT_FILENAME)
                    {
                        let root = entry.into_path();
                        let path = types::AbsolutePath::from_path(&root);
                        if let Some(path) = path {
                            roots.push(path);
                        } else {
                            let root = root.display();
                            log::warn!(r#"ignoring invalid module root path "{root}""#);
                        }
                    }
                }
                Err(err) => {
                    log::warn!("error while finding module roots, ignoring: {err}");
                }
            }
        }
    }
    roots
}

fn is_valid_utf8_visible(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name() // windows: WTF-8, unix: byte slice, usually UTF-8
        .to_str() // maybe UTF-8
        .map_or(false, |s| !s.starts_with('.'))
}
