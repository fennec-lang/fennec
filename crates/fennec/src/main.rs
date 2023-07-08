// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

mod flags;

use crate::flags::FennecCmd::Version;

fn main() {
    env_logger::init();

    let flags = flags::Fennec::from_env_or_exit();

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
