// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context;
use fennec_server::Server;

#[derive(clap::Args)]
pub struct Args {}

pub fn cmd(_args: &Args, _verbose: bool) -> anyhow::Result<()> {
    let mut srv = Server::new_stdio().context("failed to initialize LSP server")?;
    srv.serve().context("failed to serve LSP")?;
    todo!()
}
