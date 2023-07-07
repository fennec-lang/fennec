// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xshell::{cmd, Shell};

const RELEASE_VERSION: &str = "0.1.3";

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    let date = cmd!(sh, "date --rfc-email").output()?;
    let date = String::from_utf8(date.stdout)?;

    let git_describe = cmd!(sh, "git describe --tags --dirty").output()?;
    let git_describe = String::from_utf8(git_describe.stdout)?;
    let git_describe = if git_describe.is_empty() {
        format!("v{RELEASE_VERSION}")
    } else {
        git_describe
    };

    let git_branch = cmd!(sh, "git branch --show-current").output()?;
    let git_branch = String::from_utf8(git_branch.stdout)?;

    let rustc_version = cmd!(sh, "rustc --version").output()?;
    let rustc_version = String::from_utf8(rustc_version.stdout)?;

    println!("cargo:rustc-env=BUILD_DATE={date}");
    println!("cargo:rustc-env=BUILD_GIT_DESCRIBE={git_describe}");
    println!("cargo:rustc-env=BUILD_RUSTC_VERSION={rustc_version}");
    println!("cargo:rerun-if-changed=../../crates/");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads/{git_branch}");

    Ok(())
}
