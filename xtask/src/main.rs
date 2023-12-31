// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod flags;

use std::fmt::Write as _;
use std::fs;

use anyhow::Context as _;
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
        XtaskCmd::FuzzModParser(_) => run_fuzz_mod_parser(&sh),
        XtaskCmd::GenLex(_) => run_gen_lex(&sh),
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

fn run_fuzz_mod_parser(sh: &Shell) -> anyhow::Result<()> {
    sh.change_dir("crates/fennec-module");
    cmd!(sh, "cargo +nightly bolero test parser_fuzz").run()?;
    Ok(())
}

fn run_gen_lex(sh: &Shell) -> anyhow::Result<()> {
    let input = fs::read_to_string("crates/fennec-module/src/lexer_def.rs")
        .context("failed to read lexer definition")?;
    let output = lex_codegen(&input).context("failed to generate lexer")?;
    let out_path = "crates/fennec-module/src/lexer_gen.rs";
    fs::write(out_path, output).context("failed to write generated lexer")?;
    cmd!(sh, "rustfmt {out_path}")
        .run()
        .context("failed to format generated lexer")?;
    Ok(())
}

fn run_lint(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo clippy -- -D warnings").run()?;
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

fn lex_codegen(input: &str) -> anyhow::Result<String> {
    let input_tokens: proc_macro2::TokenStream = input
        .parse()
        .map_err(|err: proc_macro2::LexError| anyhow::Error::msg(err.to_string()))
        .context("failed to parse input as rust code")?;
    let mut output = String::new();
    writeln!(
        output,
        "// Generated code; run `cargo x gen-lex` to re-generate."
    )?;
    writeln!(output, "#![allow(unused_imports)]")?;
    writeln!(output, "#![allow(clippy::all)]")?;
    writeln!(output, "#![allow(clippy::pedantic)]")?;
    write!(
        output,
        "{}",
        logos_codegen::strip_attributes(input_tokens.clone())
    )?;
    write!(output, "{}", logos_codegen::generate(input_tokens))?;
    Ok(output)
}
