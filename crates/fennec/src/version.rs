// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::PROJECT_NAME;
use regex::Regex;

pub fn cmd(verbose: bool) -> anyhow::Result<()> {
    let desc = env!("BUILD_GIT_DESCRIBE");
    let version_re = Regex::new(r"^v\d+\.\d+\.\d+")?;
    let version = if version_re.is_match(desc) {
        desc.split_at(1).1
    } else {
        desc
    };

    if verbose {
        println!(
            "{PROJECT_NAME} {version}, built by {} at {}",
            env!("BUILD_RUSTC_VERSION"),
            env!("BUILD_DATE"),
        );
    } else {
        println!("{PROJECT_NAME} {version}");
    }

    Ok(())
}
