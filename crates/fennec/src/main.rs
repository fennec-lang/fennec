// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

mod flags;

use crate::flags::FennecCmd::Version;
use env_logger::Env;

fn main() {
    let flags = flags::Fennec::from_env_or_exit();

    let default_level = if flags.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level)).init();

    match flags.subcommand {
        Version(_) => {
            println!(
                "Fennec {}, built by {} at {}",
                env!("BUILD_GIT_DESCRIBE"),
                env!("BUILD_RUSTC_VERSION"),
                env!("BUILD_DATE"),
            );
        }
    }
}
