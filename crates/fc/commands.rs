// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{flags, repl};
use flags::FennecCmd::{Quit, Repl, Version};
use std::ffi::OsString;

static CMD_PREFIX: &str = ":";

pub enum Action {
    Continue,
    Exit,
}

pub fn run(flags: flags::Fennec, toplevel: bool) -> anyhow::Result<Action> {
    match flags.subcommand {
        Repl(_) => {
            if toplevel {
                run_repl(flags.verbose)?;
            }
            Ok(Action::Continue)
        }
        Version(_) => {
            println!(
                "fennec {}, built by {} at {}",
                env!("BUILD_GIT_DESCRIBE"),
                env!("BUILD_RUSTC_VERSION"),
                env!("BUILD_DATE"),
            );
            Ok(Action::Continue)
        }
        Quit(_) => {
            if flags.verbose {
                println!("Bye!");
            }
            Ok(Action::Exit)
        }
    }
}

enum LineParse {
    Whitespace,
    NotCommand,
    Command(flags::Fennec),
}

fn parse_repl_cmd(line: &str) -> anyhow::Result<LineParse> {
    let mut iter = line.split_whitespace();
    if let Some(s) = iter.next() {
        if !s.starts_with(CMD_PREFIX) {
            return Ok(LineParse::NotCommand);
        }

        let mut args = iter.collect::<Vec<&str>>();
        args.insert(0, s.split_at(CMD_PREFIX.len()).1);
        let os_args = args
            .iter()
            .map(|s| s.to_string().into())
            .collect::<Vec<OsString>>();
        let flags = flags::Fennec::from_vec(os_args)?;
        Ok(LineParse::Command(flags))
    } else {
        Ok(LineParse::Whitespace)
    }
}

fn run_repl(_verbose: bool) -> anyhow::Result<()> {
    let mut env = repl::Env::new()?;

    loop {
        match env.readline() {
            Ok(line) => {
                let parse = parse_repl_cmd(&line);
                match parse {
                    Ok(LineParse::Whitespace) => {}
                    Ok(LineParse::NotCommand) => {
                        println!("ok {}", line)
                    }
                    Ok(LineParse::Command(flags)) => {
                        let act = run(flags, false)?;
                        match act {
                            Action::Continue => {}
                            Action::Exit => break,
                        }
                    }
                    Err(err) => {
                        println!("error parsing command: {}", err);
                    }
                }
            }
            Err(err) => {
                if repl::eof(&err) {
                    break;
                }
                println!("error {}", err)
            }
        }
    }

    env.close()
}
