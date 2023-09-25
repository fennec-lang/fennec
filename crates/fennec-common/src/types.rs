// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use parking_lot::{Condvar, Mutex};
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub struct AbsolutePath(String); // UTF-8 + forward slash
pub struct RelativePath(String); // UTF-8 + forward slash

impl AbsolutePath {
    #[must_use]
    pub fn from_path(path: &Path) -> Option<AbsolutePath> {
        debug_assert!(path.is_absolute());
        let path = path.to_str()?;
        let path = if cfg!(windows) {
            path.replace('\\', "/")
        } else {
            path.to_string()
        };
        Some(AbsolutePath(path))
    }
}

#[derive(Default)]
pub struct VfsChangeBuffer {
    pub exit: bool,
    pub module_roots: Vec<AbsolutePath>,
}

#[derive(Default)]
pub struct CoreChangeBuffer {
    pub exit: bool,
    pub vfs_version: i32, // TODO: store the real thing
}

pub struct State {
    vfs_changes: Mutex<VfsChangeBuffer>,
    vfs_condvar: Condvar,
    core_changes: Mutex<CoreChangeBuffer>,
    core_changed: AtomicBool,
    core_condvar: Condvar,
}

impl State {
    #[must_use]
    pub fn new() -> State {
        State {
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

    pub fn signal_new_roots(&self, roots: Vec<AbsolutePath>) {
        let mut vfs = self.vfs_changes.lock();
        vfs.module_roots.extend(roots);
        self.notify_vfs();
    }

    pub fn signal_vfs_updates(&self) {
        let mut core = self.core_changes.lock();
        core.vfs_version += 1;
        self.notify_core();
    }

    pub fn wait_vfs(&self, timeout: Duration) -> VfsChangeBuffer {
        let mut vfs = self.vfs_changes.lock();
        self.vfs_condvar.wait_for(&mut vfs, timeout);
        std::mem::take(&mut vfs)
    }

    pub fn wait_core(&self) -> CoreChangeBuffer {
        let mut core = self.core_changes.lock();
        self.core_condvar.wait(&mut core);
        std::mem::take(&mut core)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
