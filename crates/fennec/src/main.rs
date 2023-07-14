// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use env_logger::Env;
use regex::Regex;

#[derive(Parser)]
#[command(author, about, long_about=None, disable_version_flag(true))]
struct Cli {
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print version
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let default_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level)).init();

    match cli.command {
        Commands::Version => print_version(cli.verbose),
    }
}

fn print_version(verbose: bool) -> anyhow::Result<()> {
    let desc = env!("BUILD_GIT_DESCRIBE");
    let version_re = Regex::new(r"^v\d+\.\d+\.\d+")?;
    let version = if version_re.is_match(desc) {
        desc.split_at(1).1
    } else {
        desc
    };

    if verbose {
        println!(
            "fennec {version}, built by {} at {}",
            env!("BUILD_RUSTC_VERSION"),
            env!("BUILD_DATE"),
        );
    } else {
        println!("fennec {version}");
    }

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
