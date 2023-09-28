// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

use fennec_common::{types, util, SOURCE_EXTENSION};
use std::{path::PathBuf, time::Duration};

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(991);

pub struct Vfs {
    poll_interval: Duration,
    module_roots: types::HashSet<PathBuf>,
}

// TODO: react on "saved" notifications from the client to update the in-memory content more efficiently

impl Vfs {
    #[must_use]
    pub fn new() -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
            module_roots: types::HashSet::default(),
        }
    }

    pub fn run(&mut self, state: &types::SyncState) {
        loop {
            let (changes, timed_out) = state.wait_vfs(self.poll_interval);
            if changes.exit {
                return;
            }

            let mut should_scan = timed_out;
            for root in changes.module_roots {
                // We don't want to re-scan if we got notified about a root we are already watching.
                should_scan |= self.module_roots.insert(root);
            }

            if should_scan && self.scan() {
                state.signal_vfs_updates();
            }
        }
    }

    fn scan(&mut self) -> bool {
        for root in &self.module_roots {
            let walker = walkdir::WalkDir::new(root).sort_by_file_name().into_iter();
            for entry in walker.filter_entry(|e| util::is_valid_utf8_visible(e.file_name())) {
                match entry {
                    // TODO: ensure root still has `fennec.toml`
                    // TODO: stop descending if we hit a root (DFS?)
                    Ok(entry) => {
                        if entry.file_type().is_file()
                            && entry.path().extension() == Some(SOURCE_EXTENSION.as_ref())
                        {
                            log::info!("got {entry:?} !!!");
                        }
                    }
                    Err(err) => {
                        log::warn!("error while scanning VFS, ignoring: {err}");
                    }
                }
            }
        }
        // TODO: remove roots from the set if required

        true // TODO: return if we found something new
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
