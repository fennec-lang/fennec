// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod flags;

use env_logger::Env;
use flags::{Ci, ReleaseCrate, ReleaseExt, XtaskCmd};
use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    let default_level = if flags.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level)).init();

    let sh = Shell::new()?;
    match flags.subcommand {
        XtaskCmd::Ci(Ci { all }) => {
            cmd!(sh, "cargo fmt --check").run()?;
            run_lint(&sh)?;
            cmd!(sh, "cargo test -- --quiet").run()?;
            if all {
                cmd!(sh, "cargo update -p fennec --locked").run()?;
                run_spellcheck(&sh)?;
                run_check_deps(&sh)?;

                sh.change_dir("ide/vscode-fennec");
                cmd!(sh, "npm run compile-tests").run()?;
                cmd!(sh, "npm run lint").run()?;
                cmd!(sh, "npm run test").run()?;
            }
            Ok(())
        }
        XtaskCmd::Lint(_) => run_lint(&sh),
        XtaskCmd::Spellcheck(_) => run_spellcheck(&sh),
        XtaskCmd::CheckDeps(_) => run_check_deps(&sh),
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
                log::info!(r#"pass --execute to run "vsce publish" for real"#);
            }
            Ok(())
        }
    }
}

fn run_lint(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo clippy").run()?;
    Ok(())
}

fn run_spellcheck(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cspell .").run()?;
    Ok(())
}

fn run_check_deps(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo deny check").run()?;
    cmd!(sh, "cargo +nightly udeps --all-targets").run()?;
    Ok(())
}
