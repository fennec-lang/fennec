// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use fennec_common::types;
use parking_lot::{Condvar, Mutex};

pub struct Vfs {}

impl Vfs {
    #[must_use]
    pub fn new() -> Vfs {
        Vfs {}
    }

    pub fn run(&self, _state: &Mutex<types::ChangeBuffer>, _condvar: &Condvar) {
        loop {
            // TODO: scan
            // TODO: sleep
            todo!()
        }
    }

    pub fn watch_module_root(&self, _path: &types::AbsolutePath) {
        todo!()
    }

    pub fn join(self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Vfs {
    fn drop(&mut self) {}
}
