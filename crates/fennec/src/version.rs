// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::PROJECT_NAME;
use once_cell::sync::Lazy;
use regex::Regex;

static VERSION_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^v\d+\.\d+\.\d+").expect("invalid regex literal"));

pub fn cmd(verbose: bool) {
    let version = vcs_version();
    if verbose {
        println!(
            "{PROJECT_NAME} {version}, built by {} at {}",
            env!("BUILD_RUSTC_VERSION"),
            env!("BUILD_DATE"),
        );
    } else {
        println!("{PROJECT_NAME} {version}");
    }
}

pub(crate) fn vcs_version() -> &'static str {
    let desc = env!("BUILD_GIT_DESCRIBE");
    let version = if VERSION_RE.is_match(desc) {
        desc.split_at(1).1
    } else {
        desc
    };
    version
}
