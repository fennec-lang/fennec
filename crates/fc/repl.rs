// Copyright 2022 Gregory Petrosyan <gregory.petrosyan@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow::{self, Owned};
use std::io::ErrorKind;

use ansi_term::Style;
use anyhow::Context;
use log::warn;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Config, Editor, Helper};

static PROMPT: &str = ">> ";
static HISTORY_FILENAME: &str = ".fc-history.txt";

pub struct Env {
    rl: Editor<EnvHelper>,
}

impl Env {
    pub fn new() -> Result<Self, anyhow::Error> {
        let cfg = Config::default();
        let mut rl = Editor::<EnvHelper>::with_config(cfg).context("Failed to init terminal")?;
        rl.set_helper(Some(EnvHelper {}));

        let history_err = rl.load_history(HISTORY_FILENAME);
        if let Err(err) = history_err {
            let should_warn = if let ReadlineError::Io(ref err) = err {
                err.kind() != ErrorKind::NotFound
            } else {
                true
            };
            if should_warn {
                warn!("Failed to load history: {}", err)
            }
        }

        Ok(Self { rl })
    }

    pub fn close(&mut self) -> Result<(), anyhow::Error> {
        self.rl
            .save_history(HISTORY_FILENAME)
            .context("Failed to save history")?;
        Ok(())
    }

    pub fn readline(&mut self) -> Result<String, anyhow::Error> {
        let res = self.rl.readline(PROMPT);
        match res {
            Ok(line) => {
                self.rl.add_history_entry(line.as_str());
                Ok(line)
            }
            Err(err) => Err(err.into()),
        }
    }
}

pub fn eof(err: &anyhow::Error) -> bool {
    matches!(
        err.downcast_ref::<ReadlineError>(),
        Some(ReadlineError::Eof) | Some(ReadlineError::Interrupted)
    )
}

struct EnvHelper {}

impl Completer for EnvHelper {
    type Candidate = String;
}

impl Hinter for EnvHelper {
    type Hint = String;
}

impl Highlighter for EnvHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        let _ = default;
        Owned(Style::new().bold().paint(prompt).to_string())
    }
}

impl Validator for EnvHelper {}

impl Helper for EnvHelper {}
