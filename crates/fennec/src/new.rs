// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[derive(clap::Args)]
pub struct Args {
    /// Module path
    module_path: String,

    /// Directory to use. Defaults to the last element of the module path
    #[arg(long)]
    dir: Option<String>,
}

pub fn cmd(_args: &Args, _verbose: bool) -> anyhow::Result<()> {
    panic!("TODO");
}
