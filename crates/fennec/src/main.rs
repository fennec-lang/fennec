// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

mod new;
mod server;
mod version;

#[derive(clap::Parser)]
#[command(author, about, long_about=None, disable_version_flag(true))]
struct Cli {
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Create new module
    New(new::Args),

    /// Launch language server
    Server(server::Args),

    /// Print version
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();

    let default_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_level))
        .init();

    match cli.command {
        Commands::New(args) => new::cmd(&args, cli.verbose),
        Commands::Server(args) => server::cmd(&args, cli.verbose),
        Commands::Version => {
            version::cmd(cli.verbose);
            Ok(())
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
