// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Context as _};
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
    utf8_pos: bool,
}

impl Server {
    pub fn new_stdio(version: &str) -> Result<Server, anyhow::Error> {
        let (conn, io_threads) = lsp_server::Connection::stdio();

        let (id, init_params) = conn
            .initialize_start()
            .context("failed to wait for InitializeParams")?;
        let init_params: lsp_types::InitializeParams = serde_json::from_value(init_params)
            .context("failed to deserialize InitializeParams")?;

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
            utf8_pos,
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
                            if let Some((_, params)) =
                                extract_note_params::<notification::SetTrace>(note)
                            {
                                self.handle_set_trace(params);
                            }
                        }
                        notification::DidChangeWatchedFiles::METHOD => {
                            if let Some((method, params)) =
                                extract_note_params::<notification::DidChangeWatchedFiles>(note)
                            {
                                Self::handle_did_change_watched_files(method, params, state);
                            }
                        }
                        notification::DidOpenTextDocument::METHOD => {
                            if let Some((method, params)) =
                                extract_note_params::<notification::DidOpenTextDocument>(note)
                            {
                                Self::handle_did_open_text_document(method, params, state);
                            }
                        }
                        notification::DidChangeTextDocument::METHOD => {
                            if let Some((method, params)) =
                                extract_note_params::<notification::DidChangeTextDocument>(note)
                            {
                                self.handle_did_change_text_document(method, params, state);
                            }
                        }
                        notification::DidCloseTextDocument::METHOD => {
                            if let Some((method, params)) =
                                extract_note_params::<notification::DidCloseTextDocument>(note)
                            {
                                Self::handle_did_close_text_document(method, &params, state);
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
        method: &str,
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
            if let Some(manifest) = uri_to_pathbuf(&change.uri, method) {
                roots.extend(module_manifest_parent(&manifest));
            }
        }
        state.signal_vfs_new_roots(roots);
    }

    fn handle_did_open_text_document(
        method: &str,
        params: lsp_types::DidOpenTextDocumentParams,
        state: &types::SyncState,
    ) {
        if let Some(path) = uri_to_pathbuf(&params.text_document.uri, method) {
            let text = types::Text::from(params.text_document.text);
            let update = types::OverlayUpdate::AddOverlay(path, text, params.text_document.version);
            state.signal_vfs_overlay_updates(vec![update]);
        }
    }

    fn handle_did_change_text_document(
        &self,
        method: &str,
        params: lsp_types::DidChangeTextDocumentParams,
        state: &types::SyncState,
    ) {
        if let Some(path) = uri_to_pathbuf(&params.text_document.uri, method) {
            let changes = params.content_changes.into_iter().map(|c| {
                if c.range.is_none() && c.range_length.is_none() {
                    Some(types::OverlayChange {
                        range: None,
                        content: c.text,
                        utf8_pos: self.utf8_pos,
                    })
                } else if let Some(range) = c.range {
                    Some(types::OverlayChange {
                        range: Some((
                            types::LineCol {
                                line: range.start.line,
                                col: range.start.character,
                            },
                            types::LineCol {
                                line: range.end.line,
                                col: range.end.character,
                            },
                        )),
                        content: c.text,
                        utf8_pos: self.utf8_pos,
                    })
                } else {
                    let uri = &params.text_document.uri;
                    log::warn!(r#"{method} for "{uri}" contains only deprecated rangeLength field, ignoring"#);
                    None
                }
            });
            let update = types::OverlayUpdate::ChangeOverlay(
                path,
                changes.flatten().collect(),
                params.text_document.version,
            );
            state.signal_vfs_overlay_updates(vec![update]);
        }
    }

    fn handle_did_close_text_document(
        method: &str,
        params: &lsp_types::DidCloseTextDocumentParams,
        state: &types::SyncState,
    ) {
        if let Some(path) = uri_to_pathbuf(&params.text_document.uri, method) {
            let update = types::OverlayUpdate::RemoveOverlay(path);
            state.signal_vfs_overlay_updates(vec![update]);
        }
    }
}

fn uri_to_pathbuf(uri: &lsp_types::Url, method: &str) -> Option<PathBuf> {
    if uri.scheme() != handshake::FILE_SCHEME {
        log::warn!(r#"ignoring non-file-scheme {method} for "{uri}""#);
        return None;
    }
    if let Ok(path) = uri.to_file_path() {
        Some(util::normalize_path(&path))
    } else {
        log::warn!(r#"ignoring {method} with invalid file path "{uri}""#);
        None
    }
}

fn extract_note_params<N>(note: lsp_server::Notification) -> Option<(&'static str, N::Params)>
where
    N: notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    match note.extract(N::METHOD) {
        Ok(params) => Some((N::METHOD, params)),
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
    Some(manifest.parent()?.to_path_buf())
}
