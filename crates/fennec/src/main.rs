// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap_verbosity_flag::{InfoLevel, Verbosity};

mod new;
mod server;
mod version;

#[derive(clap::Parser)]
#[command(author, about, long_about=None, disable_version_flag(true))]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Create new Fennec module
    New(new::Args),

    /// Launch Fennec language server
    Server(server::Args),

    /// Print Fennec version
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match cli.command {
        Commands::New(args) => new::cmd(&args),
        Commands::Server(args) => server::cmd(&args),
        Commands::Version => {
            let level = cli.verbose.log_level().unwrap_or(log::Level::Info);
            let verbose = level > log::Level::Info;
            version::cmd(verbose);
            Ok(())
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
