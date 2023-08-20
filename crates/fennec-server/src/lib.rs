// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use anyhow::Context;
use fennec_common::{
    types::{Root, RootPath},
    PROJECT_NAME,
};

pub struct Server {
    conn: lsp_server::Connection,
    io_threads: lsp_server::IoThreads,

    // TODO: use
    pub folders: Vec<Root>,
    pub utf8_pos: bool,
    pub parent_process_id: Option<u32>, // TODO: exit when parent is dead
}

impl Server {
    #[allow(deprecated)]
    pub fn new_stdio(version: &str) -> Result<Server, anyhow::Error> {
        let (conn, io_threads) = lsp_server::Connection::stdio();

        let (id, init_params) = conn
            .initialize_start()
            .context("failed to wait for InitializeParams")?;
        let init_params: lsp_types::InitializeParams =
            serde_json::from_value(init_params).context("failed to unmarshal InitializeParams")?;
        if log::log_enabled!(log::Level::Debug) {
            let init_pretty =
                serde_json::to_string_pretty(&init_params).unwrap_or_else(|e| e.to_string());
            log::debug!("InitializeParams: {init_pretty}");
        }

        let mut utf8_pos = false;
        if let Some(general_caps) = init_params.capabilities.general {
            if let Some(encodings) = general_caps.position_encodings {
                utf8_pos = encodings.contains(&lsp_types::PositionEncodingKind::UTF8);
            }
        }

        let mut folders: Vec<Root> = vec![];
        if let Some(path) = init_params.root_path {
            folders = vec![Root {
                path: path_to_root_path(&path),
                name: path_to_basename(&path),
            }];
        }
        if let Some(uri) = init_params.root_uri {
            let mut name = "";
            if let Some(seg) = uri.path_segments() {
                name = seg.last().unwrap_or("");
            }
            folders = vec![Root {
                path: uri_to_root_path(&uri),
                name: name.to_owned(),
            }];
        }
        if let Some(wf) = init_params.workspace_folders {
            folders = wf
                .iter()
                .map(|f| Root {
                    path: uri_to_root_path(&f.uri),
                    name: f.name.clone(),
                })
                .collect();
        }

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
            serde_json::to_value(init_result).context("failed to marshal InitializeResult")?;

        conn.initialize_finish(id, init_result)
            .context("failed to send InitializeResult")?;

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

fn path_to_basename(_path: &str) -> String {
    todo!()
}

fn path_to_root_path(_path: &str) -> RootPath {
    todo!()
}

fn uri_to_root_path(_uri: &lsp_types::Url) -> RootPath {
    todo!()
}
