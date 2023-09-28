// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use parking_lot::{Condvar, Mutex};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;
pub type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;

#[derive(Default)]
pub struct VfsChangeBuffer {
    pub exit: bool,
    // Another possibility would be to watch workspace roots and not module roots.
    // While more general, that would duplicate more of the client-side watching
    // and be less efficient, so we expect the client to feed us with module roots.
    pub module_roots: Vec<PathBuf>,
}

#[derive(Default)]
pub struct CoreChangeBuffer {
    pub exit: bool,
    pub vfs_version: i32, // TODO: store the real thing
}

pub struct SyncState {
    vfs_changes: Mutex<VfsChangeBuffer>,
    vfs_condvar: Condvar,
    core_changes: Mutex<CoreChangeBuffer>,
    core_changed: AtomicBool,
    core_condvar: Condvar,
}

impl SyncState {
    #[must_use]
    pub fn new() -> SyncState {
        SyncState {
            vfs_changes: Mutex::new(VfsChangeBuffer::default()),
            vfs_condvar: Condvar::new(),
            core_changes: Mutex::new(CoreChangeBuffer::default()),
            core_changed: AtomicBool::new(false),
            core_condvar: Condvar::new(),
        }
    }

    fn notify_vfs(&self) {
        self.vfs_condvar.notify_one();
    }

    fn notify_core(&self) {
        self.core_changed.store(true, Ordering::Release);
        self.core_condvar.notify_one();
    }

    pub fn is_core_changed(&self) -> bool {
        self.core_changed.load(Ordering::Acquire)
    }

    pub fn signal_exit(&self) {
        {
            let mut vfs = self.vfs_changes.lock();
            vfs.exit = true;
            self.notify_vfs();
        }
        {
            let mut core = self.core_changes.lock();
            core.exit = true;
            self.notify_core();
        }
    }

    pub fn signal_new_roots(&self, roots: Vec<PathBuf>) {
        let mut vfs = self.vfs_changes.lock();
        vfs.module_roots.extend(roots);
        self.notify_vfs();
    }

    pub fn signal_vfs_updates(&self) {
        let mut core = self.core_changes.lock();
        core.vfs_version += 1;
        self.notify_core();
    }

    pub fn wait_vfs(&self, timeout: Duration) -> (VfsChangeBuffer, bool) {
        let mut vfs = self.vfs_changes.lock();
        let res = self.vfs_condvar.wait_for(&mut vfs, timeout);
        (std::mem::take(&mut vfs), res.timed_out())
    }

    pub fn wait_core(&self) -> CoreChangeBuffer {
        let mut core = self.core_changes.lock();
        self.core_condvar.wait(&mut core);
        std::mem::take(&mut core)
    }
}

impl Default for SyncState {
    fn default() -> Self {
        Self::new()
    }
}
