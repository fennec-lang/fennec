// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use fennec_common::types;
use std::time::Duration;

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(991);

pub struct Vfs {
    poll_interval: Duration,
    module_roots: Vec<types::AbsolutePath>,
}

impl Vfs {
    #[must_use]
    pub fn new() -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
            module_roots: vec![],
        }
    }

    pub fn run(&mut self, state: &types::State) {
        loop {
            if self.scan() {
                state.signal_vfs_updates();
            }

            let changes = state.wait_vfs(self.poll_interval);
            if changes.exit {
                return;
            }
            for root in changes.module_roots {
                self.add_root(root);
            }
        }
    }

    fn add_root(&mut self, root: types::AbsolutePath) {
        self.module_roots.push(root);
        todo!()
    }

    fn scan(&self) -> bool {
        todo!()
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
