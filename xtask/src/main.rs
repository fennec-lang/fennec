// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

mod flags;

use env_logger::Env;
use flags::{ReleaseCrate, ReleaseExt, XtaskCmd};
use log::info;
use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    let default_level = if flags.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level)).init();

    let sh = Shell::new()?;
    match flags.subcommand {
        XtaskCmd::CheckDeps(_) => {
            cmd!(sh, "cargo deny check").run()?;
            cmd!(sh, "cargo +nightly udeps --all-targets").run()?;
            Ok(())
        }
        XtaskCmd::ReleaseCrate(ReleaseCrate { version, execute }) => {
            let exec = if execute {
                ["--execute", "--no-confirm"].as_slice()
            } else {
                [].as_slice()
            };
            cmd!(sh, "cargo release {version} {exec...}").run()?;
            Ok(())
        }
        XtaskCmd::ReleaseExt(ReleaseExt { execute }) => {
            sh.change_dir("ide/vscode-fennec");
            if execute {
                cmd!(sh, "vsce publish").run()?;
            } else {
                info!(r#"pass --execute to run "vsce publish" for real"#);
            }
            Ok(())
        }
        XtaskCmd::Spellcheck(_) => {
            cmd!(sh, "cspell .").run()?;
            Ok(())
        }
    }
}
