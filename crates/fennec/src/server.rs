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
use parking_lot::{Condvar, Mutex};
use std::thread;

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
    let state = Mutex::new(types::ChangeBuffer::new());
    let condvar = Condvar::new();

    let mut core = Core::new();
    let vfs = Vfs::new();
    let mut srv = Server::new_stdio(version).context("failed to initialize LSP server")?;
    let mut serve_err: Option<anyhow::Error> = None;

    thread::scope(|s| {
        // fill change buffer: server loop
        s.spawn(|| {
            let root_cb = |path: &types::AbsolutePath| vfs.watch_module_root(path);
            serve_err = srv.serve(&root_cb, &state, &condvar).err();
        });

        // fill change buffer: VFS loop
        s.spawn(|| {
            vfs.run(&state, &condvar);
        });

        // consume change buffer: core loop
        s.spawn(|| loop {
            let mut change = {
                let mut state = state.lock();
                condvar.wait(&mut state);
                core.extract(&mut state)
            };
            core.apply(&mut change);
        });
    });

    serve_err.context("failed to serve LSP")?;

    vfs.join().context("failed to join VFS threads")?;
    srv.join().context("failed to join IO threads")?;
    Ok(())
}
