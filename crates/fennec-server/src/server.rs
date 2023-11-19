// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Context};
use fennec_common::{types, util, MODULE_MANIFEST_FILENAME, PROJECT_NAME};
use lsp_types::notification::{self, Notification as _};
use std::path::{Path, PathBuf};

use crate::handshake;

pub struct Server {
    conn: lsp_server::Connection,
    io_threads: lsp_server::IoThreads,
    request_id: i32,

    // From LSP InitializeParams:
    workspace_folders: Vec<PathBuf>,
    _utf8_pos: bool, // TODO: use
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

        let dyn_watch = handshake::fs_watch_dynamic(&init_params.capabilities);
        if !dyn_watch {
            return Err(anyhow!("Fennec LSP server requires client to support dynamic registration in DidChangeWatchedFilesClientCapabilities"));
        }

        let utf8_pos = handshake::utf8_positions(&init_params.capabilities);
        let init_result = lsp_types::InitializeResult {
            capabilities: handshake::server_capabilities(utf8_pos),
            server_info: Some(lsp_types::ServerInfo {
                name: PROJECT_NAME.to_owned(),
                version: Some(version.to_owned()),
            }),
        };
        let init_result =
            serde_json::to_value(init_result).context("failed to serialize InitializeResult")?;

        conn.initialize_finish(id, init_result)
            .context("failed to send InitializeResult")?;

        // We don't handle init_params.process_id in any way here.
        // Ideally, lsp_server should react to EOF from stdin and
        // initiate clean shutdown (disconnect the sending side of conn.receiver).
        // However, I have not tested that it actually works.

        Ok(Server {
            conn,
            io_threads,
            request_id: 0,
            _utf8_pos: utf8_pos,
            workspace_folders: handshake::workspace_roots(&init_params),
        })
    }

    #[must_use]
    pub fn watch_for_roots(&self) -> bool {
        true // ¯\_(ツ)_/¯
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

    pub fn run(&mut self, state: &types::SyncState) -> Result<(), anyhow::Error> {
        let reg_id = self.next_id();
        let mut registered_manifest_watchers = false;
        handshake::register_module_manifest_watchers(&self.conn, reg_id.clone()).context(
            format!("failed to register {MODULE_MANIFEST_FILENAME} watchers"),
        )?;

        // TODO: server must wait for responses from the core (and be able to cancel them)

        for msg in &self.conn.receiver {
            match msg {
                lsp_server::Message::Request(req) => {
                    if self.conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    if !registered_manifest_watchers {
                        let lsp_server::Request { id, method, .. } = req;
                        let msg = format!(
                            r#"got "{method}" (id {id}) request before module manifest watchers were registered, ignoring"#
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
                    // TODO: process request
                }
                lsp_server::Message::Response(resp) => {
                    if resp.id == reg_id {
                        if let Some(lsp_server::ResponseError { code, message, .. }) = resp.error {
                            return Err(anyhow!("failed to register {MODULE_MANIFEST_FILENAME} watchers: [{code}] {message}"));
                        }
                        registered_manifest_watchers = true;

                        // We find the roots only after watchers are registered to avoid possible races
                        // where we would miss new roots that appeared after the walk is complete but
                        // before the watch is set up.
                        let roots = find_module_roots(&self.workspace_folders);
                        state.signal_vfs_new_roots(roots);
                    }
                    // TODO: process response
                }
                lsp_server::Message::Notification(note) => {
                    if !registered_manifest_watchers {
                        let method = note.method;
                        log::warn!(
                            r#"got "{method}" notification before module manifest watchers were registered, ignoring"#
                        );
                        continue;
                    }

                    match note.method.as_str() {
                        notification::SetTrace::METHOD => {
                            let params = extract_note_params::<notification::SetTrace>(note);
                            if let Some(params) = params {
                                self.handle_set_trace(params);
                            }
                        }
                        notification::DidChangeWatchedFiles::METHOD => {
                            let params =
                                extract_note_params::<notification::DidChangeWatchedFiles>(note);
                            if let Some(params) = params {
                                Self::handle_did_change_watched_files(params, state);
                            }
                        }
                        notification::DidOpenTextDocument::METHOD => {
                            let params =
                                extract_note_params::<notification::DidOpenTextDocument>(note);
                            if let Some(params) = params {
                                self.handle_did_open_text_document(params, state);
                            }
                        }
                        notification::DidChangeTextDocument::METHOD => {
                            let params =
                                extract_note_params::<notification::DidChangeTextDocument>(note);
                            if let Some(params) = params {
                                self.handle_did_change_text_document(params, state);
                            }
                        }
                        notification::DidCloseTextDocument::METHOD => {
                            let params =
                                extract_note_params::<notification::DidCloseTextDocument>(note);
                            if let Some(params) = params {
                                self.handle_did_close_text_document(params, state);
                            }
                        }
                        other => {
                            log::warn!(r#"got an unexpected "{other}" notification, ignoring"#);
                            continue;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    fn handle_set_trace(&self, _params: lsp_types::SetTraceParams) {
        // We'll figure something useful later.
    }

    fn handle_did_change_watched_files(
        params: lsp_types::DidChangeWatchedFilesParams,
        state: &types::SyncState,
    ) {
        let mut roots: Vec<PathBuf> = vec![];
        for change in params.changes {
            if change.typ != lsp_types::FileChangeType::CREATED {
                // We react to create events only because we expect to get change/delete events from our VFS.
                // Note that VSCode seems to miss e.g. delete events for module manifests
                // when module manifest parent folder is deleted.
                continue;
            }
            let uri = change.uri;
            if uri.scheme() != handshake::FILE_SCHEME {
                log::warn!(r#"ignoring non-file-scheme change event for "{uri}""#);
                continue;
            }
            if let Ok(manifest) = uri.to_file_path() {
                roots.extend(module_manifest_parent(&manifest));
            } else {
                log::warn!(r#"ignoring change event with invalid file path "{uri}""#);
            }
        }
        state.signal_vfs_new_roots(roots);
    }

    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    fn handle_did_open_text_document(
        &self,
        _params: lsp_types::DidOpenTextDocumentParams,
        _state: &types::SyncState,
    ) {
    }

    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    fn handle_did_change_text_document(
        &self,
        _params: lsp_types::DidChangeTextDocumentParams,
        _state: &types::SyncState,
    ) {
    }

    #[allow(clippy::unused_self, clippy::needless_pass_by_value)]
    fn handle_did_close_text_document(
        &self,
        _params: lsp_types::DidCloseTextDocumentParams,
        _state: &types::SyncState,
    ) {
    }
}

fn extract_note_params<N>(note: lsp_server::Notification) -> Option<N::Params>
where
    N: notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    match note.extract(N::METHOD) {
        Ok(params) => Some(params),
        Err(err) => {
            let method = N::METHOD;
            log::warn!(r#"failed to extract "{method}" notification params, ignoring: {err}"#);
            None
        }
    }
}

fn find_module_roots(workspace_folders: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = Vec::with_capacity(workspace_folders.len());
    for folder in workspace_folders {
        let walker = walkdir::WalkDir::new(folder).into_iter();
        for entry in walker.filter_entry(|e| util::is_valid_utf8_visible(e.file_name())) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() && entry.file_name() == MODULE_MANIFEST_FILENAME
                    {
                        roots.extend(module_manifest_parent(&entry.into_path()));
                    }
                }
                Err(err) => {
                    log::warn!("error while scanning for module roots, ignoring: {err}");
                }
            }
        }
    }
    roots
}

fn module_manifest_parent(manifest: &Path) -> Option<PathBuf> {
    assert!(manifest.file_name() == Some(MODULE_MANIFEST_FILENAME.as_ref()));
    Some(util::normalize_path(manifest).parent()?.to_path_buf())
}
