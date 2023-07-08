// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

mod flags;

use env_logger::Env;

fn main() {
    let flags = flags::Xtask::from_env_or_exit();

    let default_level = if flags.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level)).init();

    match flags.subcommand {
        flags::XtaskCmd::HelloWorld(_) => {
            println!("Hello, World!");
        }
    }
}
