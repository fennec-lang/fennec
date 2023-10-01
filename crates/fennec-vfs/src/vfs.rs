// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::{types, util, SOURCE_EXTENSION};
use std::{path::PathBuf, time::Duration};

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(991);

pub struct Vfs {
    poll_interval: Duration,
    scan_roots: Vec<PathBuf>, // sorted; no element is a prefix of another element
}

impl Vfs {
    #[must_use]
    pub fn new() -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
            scan_roots: Vec::default(),
        }
    }

    pub fn run(&mut self, state: &types::SyncState) {
        loop {
            let (changes, timed_out) = state.wait_vfs(self.poll_interval);
            if changes.exit {
                return;
            }

            // In the future, we could consider reacting to the client-side watch notifications
            // in addition to periodic scanning here.

            let mut should_scan = timed_out;
            for root in changes.scan_roots {
                // We don't want to re-scan if we got notified about a root we are already watching.
                should_scan |= self.add_root(root);
            }

            if should_scan && self.scan() {
                // TODO: update the local workspace state
                state.signal_vfs_updates(Vec::default());
            }
        }
    }

    fn add_root(&mut self, root: PathBuf) -> bool {
        let ins = self.scan_roots.binary_search(&root);
        match ins {
            Ok(_) => false,
            Err(ix) => {
                if let Some(prev_root) = self.scan_roots.get(ix - 1) {
                    if root.strip_prefix(prev_root).is_ok() {
                        // Trying to add subdirectory of an existing root; do nothing.
                        return false;
                    }
                }
                self.scan_roots.insert(ix, root);
                true
            }
        }
    }

    fn scan(&mut self) -> bool {
        for root in &self.scan_roots {
            let walker = walkdir::WalkDir::new(root).sort_by_file_name().into_iter();
            for entry in walker.filter_entry(|e| util::is_valid_utf8_visible(e.file_name())) {
                match entry {
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

        // TODO: build a fs tree; from it produce a module tree; diff it to the prev. module tree and push the changes

        true // TODO: return if we found something new
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
