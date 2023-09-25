// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use fennec_common::types;

pub mod import;

pub struct Core {}

impl Core {
    #[must_use]
    pub fn new() -> Core {
        Core {}
    }

    pub fn run(&mut self, state: &types::State) {
        loop {
            let mut changes = state.wait_core();
            if changes.exit {
                return;
            }

            self.apply(&mut changes, state);
        }
    }

    fn apply(&mut self, _changes: &mut types::CoreChangeBuffer, _state: &types::State) {
        // TODO: take changes by value
        todo!()
    }

    // TODO: sync + async diagnostics + results
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
