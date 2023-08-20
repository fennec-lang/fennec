// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use anyhow::{anyhow, Context};
use fennec_common::{types::RootPath, MODULE_ROOT_FILENAME, PROJECT_NAME};
use lsp_types::{notification::Notification, request::Request};

const FILE_SCHEME: &str = "file";

pub struct Server {
    conn: lsp_server::Connection,
    io_threads: lsp_server::IoThreads,

    // TODO: use
    pub folders: Vec<RootPath>,
    pub utf8_pos: bool,
    pub parent_process_id: Option<u32>, // TODO: exit when parent is dead
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

        register_module_root_watchers(&conn).context(format!(
            "failed to register {MODULE_ROOT_FILENAME} watchers"
        ))?;
        // TODO: we need to find initial set of roots by hand

        Ok(Server {
            conn,
            io_threads,
            utf8_pos,
            folders,
            parent_process_id: init_params.process_id,
        })
    }

    pub fn join(self) -> Result<(), anyhow::Error> {
        self.io_threads.join()?;
        Ok(())
    }

    // TODO: write real code
    pub fn serve(&mut self) -> Result<(), anyhow::Error> {
        for msg in &self.conn.receiver {
            log::trace!("got msg: {msg:?}");
            match msg {
                lsp_server::Message::Request(req) => {
                    log::trace!("got request: {req:?}");
                    if self.conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                }
                lsp_server::Message::Response(resp) => {
                    log::trace!("got response: {resp:?}");
                }
                lsp_server::Message::Notification(not) => {
                    log::trace!("got notification: {not:?}");
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

fn workspace_roots(init_params: &lsp_types::InitializeParams) -> Vec<RootPath> {
    if let Some(ref wf) = init_params.workspace_folders {
        return wf
            .iter()
            .filter(|f| f.uri.scheme() == FILE_SCHEME)
            .filter_map(|f| f.uri.to_file_path().ok())
            .filter_map(|pb| RootPath::from_path(&pb))
            .collect();
    }
    init_params
        .root_uri
        .iter()
        .filter(|uri| uri.scheme() == FILE_SCHEME)
        .filter_map(|uri| uri.to_file_path().ok())
        .filter_map(|pb| RootPath::from_path(&pb))
        .collect()
}

fn register_module_root_watchers(conn: &lsp_server::Connection) -> Result<(), anyhow::Error> {
    let opts = lsp_types::DidChangeWatchedFilesRegistrationOptions {
        watchers: vec![lsp_types::FileSystemWatcher {
            glob_pattern: lsp_types::GlobPattern::String(
                format!("**/{MODULE_ROOT_FILENAME}").to_owned(),
            ),
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
        id: lsp_server::RequestId::from(0),
        method: lsp_types::request::RegisterCapability::METHOD.to_owned(),
        params,
    };
    conn.sender
        .send(lsp_server::Message::Request(req))
        .context("failed to send client/registerCapability request")?;

    // TODO: should we wait for response or no?
    Ok(())
}
