// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
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
static HISTORY_FILENAME: &str = ".fennec-history.txt";

pub struct Env {
    rl: Editor<EnvHelper>,
}

impl Env {
    pub fn new() -> anyhow::Result<Self> {
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
                warn!("Failed to load history: {}", err);
            }
        }

        Ok(Self { rl })
    }

    pub fn close(&mut self) -> anyhow::Result<()> {
        self.rl
            .save_history(HISTORY_FILENAME)
            .context("Failed to save history")?;
        Ok(())
    }

    pub fn readline(&mut self) -> anyhow::Result<String> {
        let line = self.rl.readline(PROMPT)?;
        Ok(line)
    }

    pub fn remember(&mut self, line: String) {
        self.rl.add_history_entry(line);
    }
}

pub fn eof(err: &anyhow::Error) -> bool {
    matches!(
        err.downcast_ref::<ReadlineError>(),
        Some(ReadlineError::Eof | ReadlineError::Interrupted)
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
        _ = default;
        Owned(Style::new().bold().paint(prompt).to_string())
    }
}

impl Validator for EnvHelper {}

impl Helper for EnvHelper {}
