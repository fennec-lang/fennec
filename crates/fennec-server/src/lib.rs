// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use anyhow::Context;
use lsp_server::{Connection, ExtractError, IoThreads, Message, Request, RequestId, Response};
use lsp_types::{request::GotoDefinition, GotoDefinitionResponse, InitializeParams};

pub struct Server {
    conn: Connection,
    io_threads: IoThreads,
}

impl Server {
    #[must_use]
    pub fn new_stdio() -> Server {
        let (conn, io_threads) = Connection::stdio();
        Server { conn, io_threads }
    }

    pub fn join(self) -> Result<(), anyhow::Error> {
        self.io_threads.join()?;
        Ok(())
    }

    // TODO: write real code
    pub fn serve(&mut self, params: serde_json::Value) -> Result<(), anyhow::Error> {
        let _params: InitializeParams =
            serde_json::from_value(params).context("failed to unmarshal InitializeParams")?;
        for msg in &self.conn.receiver {
            log::info!("got msg: {msg:?}");
            match msg {
                Message::Request(req) => {
                    use lsp_types::request::Request;
                    log::info!("got request: {req:?}");
                    if self.conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    match req.method.as_str() {
                        GotoDefinition::METHOD => match extract::<GotoDefinition>(req) {
                            Ok((id, params)) => {
                                log::info!("got gotoDefinition request #{id}: {params:?}");
                                let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                                let result = serde_json::to_value(&result)
                                    .context("failed to marshal GotoDefinitionResponse")?;
                                let resp = Response {
                                    id,
                                    result: Some(result),
                                    error: None,
                                };
                                self.conn.sender.send(Message::Response(resp))?;
                                continue;
                            }
                            Err(err) => {
                                log::warn!("failed to extract request: {err}");
                            }
                        },
                        m => {
                            log::warn!("unexpected method {m}");
                        }
                    }
                }
                Message::Response(resp) => {
                    log::info!("got response: {resp:?}");
                }
                Message::Notification(not) => {
                    log::info!("got notification: {not:?}");
                }
            }
        }
        Ok(())
    }
}

fn extract<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
