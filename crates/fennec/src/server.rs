// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::version::vcs_version;
use anyhow::{anyhow, Context};
use fennec_common::types;
use fennec_core::Core;
use fennec_server::Server;
use fennec_vfs::Vfs;
use std::thread;

#[derive(clap::Args)]
pub struct Args {
    /// Use stdio LSP connection
    #[arg(long)]
    stdio: bool,
}

pub fn cmd(args: &Args) -> anyhow::Result<()> {
    if !args.stdio {
        return Err(anyhow!("--stdio is the only supported LSP mode"));
    }

    let state = types::SyncState::new();
    let mut vfs = Vfs::new();
    let mut core = Core::new();
    let mut srv = Server::new_stdio(vcs_version()).context("failed to initialize LSP server")?;
    let mut srv_err: Option<anyhow::Error> = None;

    thread::scope(|s| {
        // Fill change buffer: server loop
        s.spawn(|| {
            srv_err = srv.run(&state).err();
            state.signal_exit();
        });

        // Fill change buffer: VFS loop
        s.spawn(|| {
            vfs.run(&state);
        });

        // Consume change buffer: core loop
        s.spawn(|| {
            core.run(&state);
        });
    });

    srv_err.context("failed to serve LSP")?;
    srv.join().context("failed to join IO threads")?;

    Ok(())
}
