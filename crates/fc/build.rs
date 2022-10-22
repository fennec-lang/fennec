// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Command;

fn main() -> anyhow::Result<()> {
    let date_out = Command::new("date").args(&["--rfc-email"]).output()?;
    let date = String::from_utf8(date_out.stdout)?;

    let git_describe_out = Command::new("git")
        .args(&["describe", "--tags", "--dirty"])
        .output()?;
    let git_describe = String::from_utf8(git_describe_out.stdout)?;

    let rustc_version_out = Command::new("rustc").args(&["--version"]).output()?;
    let rustc_version = String::from_utf8(rustc_version_out.stdout)?;

    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=BUILD_GIT_DESCRIBE={}", git_describe);
    println!("cargo:rustc-env=BUILD_RUSTC_VERSION={}", rustc_version);

    Ok(())
}
