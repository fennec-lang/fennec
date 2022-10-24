// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

mod commands;
mod flags;
mod repl;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let flags = flags::Fennec::from_env_or_exit();
    let _ = commands::run(&flags, true)?;
    Ok(())
}
