// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

xflags::xflags! {
    src "crates/fennec/flags.rs"

    cmd fennec {
        optional -v, --verbose
        default cmd repl {}
        cmd version {}
        cmd quit {}
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Fennec {
    pub verbose: bool,
    pub subcommand: FennecCmd,
}

#[derive(Debug)]
pub enum FennecCmd {
    Repl(Repl),
    Version(Version),
    Quit(Quit),
}

#[derive(Debug)]
pub struct Repl;

#[derive(Debug)]
pub struct Version;

#[derive(Debug)]
pub struct Quit;

impl Fennec {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end