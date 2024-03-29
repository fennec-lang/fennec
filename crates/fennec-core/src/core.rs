// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::types;

pub struct Core {}

impl Core {
    #[must_use]
    pub fn new() -> Core {
        Core {}
    }

    pub fn run(&mut self, state: &types::SyncState) {
        loop {
            let mut changes = state.wait_core();
            if changes.exit {
                return;
            }

            // In the future, we could consider simplifying the changes before processing
            // (e.g., ignoring any update that was later followed by removal).

            self.apply(&mut changes, state);
        }
    }

    #[allow(clippy::unused_self)]
    fn apply(&mut self, _changes: &mut types::CoreChangeBuffer, _state: &types::SyncState) {
        // TODO: take changes by value
        log::warn!("TODO");
    }

    // TODO: sync + async diagnostics + results
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
