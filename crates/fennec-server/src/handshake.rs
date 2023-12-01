// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context as _;
use fennec_common::MODULE_MANIFEST_FILENAME;
use lsp_types::{
    notification::{self, Notification as _},
    request::{self, Request as _},
    ClientCapabilities, DidChangeWatchedFilesRegistrationOptions, FileSystemWatcher, GlobPattern,
    InitializeParams, OneOf, PositionEncodingKind, Registration, RegistrationParams,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, WatchKind, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};
use std::path::PathBuf;

pub const FILE_SCHEME: &str = "file";

pub fn server_capabilities(utf8_pos: bool) -> ServerCapabilities {
    // We simplistically assume that:
    // - client-side watching for module roots
    // - VFS picking up real changes from the file system within modules as scan roots
    // - open/change/close notifications from the client
    // is enough to maintain an accurate module-level view of the state.
    //
    // It might be a good idea to trigger VFS re-scans after getting some
    // client-side notifications.
    ServerCapabilities {
        position_encoding: if utf8_pos {
            Some(lsp_types::PositionEncodingKind::UTF8)
        } else {
            None
        },
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true), // textDocument/didOpen, textDocument/didClose
                change: Some(TextDocumentSyncKind::INCREMENTAL), // textDocument/didChange
                will_save: Some(false), // textDocument/willSave
                will_save_wait_until: Some(false), // textDocument/willSaveWaitUntil
                save: Some(TextDocumentSyncSaveOptions::Supported(false)), // textDocument/didSave
            },
        )),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(false)), // workspace/didChangeWorkspaceFolders
            }),
            file_operations: None,
        }),
        ..Default::default()
    }
}

pub fn fs_watch_dynamic(client_caps: &ClientCapabilities) -> bool {
    if let Some(ref workspace_caps) = client_caps.workspace {
        if let Some(ref change_watched) = workspace_caps.did_change_watched_files {
            return change_watched.dynamic_registration.unwrap_or(false);
        }
    }
    false
}

pub fn utf8_positions(client_caps: &ClientCapabilities) -> bool {
    if let Some(ref general_caps) = client_caps.general {
        if let Some(ref encodings) = general_caps.position_encodings {
            return encodings.contains(&PositionEncodingKind::UTF8);
        }
    }
    false
}

pub fn workspace_roots(init_params: &InitializeParams) -> Vec<PathBuf> {
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

pub fn register_module_manifest_watchers(
    conn: &lsp_server::Connection,
    id: lsp_server::RequestId,
) -> Result<(), anyhow::Error> {
    let opts = DidChangeWatchedFilesRegistrationOptions {
        watchers: vec![FileSystemWatcher {
            glob_pattern: GlobPattern::String(format!("**/{MODULE_MANIFEST_FILENAME}")),
            kind: Some(WatchKind::Create),
        }],
    };
    let opts = serde_json::to_value(opts)
        .context("failed to serialize DidChangeWatchedFilesRegistrationOptions")?;

    let params = RegistrationParams {
        registrations: vec![Registration {
            id: MODULE_MANIFEST_FILENAME.to_owned(),
            method: notification::DidChangeWatchedFiles::METHOD.to_owned(),
            register_options: Some(opts),
        }],
    };
    let params = serde_json::to_value(params).context("failed to serialize RegistrationParams")?;

    let req = lsp_server::Request {
        id,
        method: request::RegisterCapability::METHOD.to_owned(),
        params,
    };
    conn.sender
        .send(req.into())
        .context("failed to send client/registerCapability request")?;

    Ok(())
}
