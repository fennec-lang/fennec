// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::version::vcs_version;
use anyhow::{anyhow, Context};
use fennec_server::Server;

#[derive(clap::Args)]
pub struct Args {
    /// Use stdio LSP connection
    #[arg(long)]
    stdio: bool,
}

pub fn cmd(args: &Args, _verbose: bool) -> anyhow::Result<()> {
    if !args.stdio {
        return Err(anyhow!("--stdio is the only supported LSP mode"));
    }
    let version = vcs_version();
    let mut srv = Server::new_stdio(version).context("failed to initialize LSP server")?;
    srv.serve().context("failed to serve LSP")?;
    srv.join().context("failed to join IO threads")?;
    Ok(())
}
