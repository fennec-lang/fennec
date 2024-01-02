// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use parking_lot::{Condvar, Mutex};
use std::{
    mem::take,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::{types, workspace};

#[derive(Debug)]
pub enum OverlayUpdate {
    AddOverlay(PathBuf, types::Text, i32),
    ChangeOverlay(PathBuf, Vec<OverlayChange>, i32),
    RemoveOverlay(PathBuf),
}

impl OverlayUpdate {
    #[must_use]
    pub fn path(&self) -> &Path {
        match self {
            types::OverlayUpdate::AddOverlay(path, _, _)
            | types::OverlayUpdate::ChangeOverlay(path, _, _)
            | types::OverlayUpdate::RemoveOverlay(path) => path,
        }
    }
}

#[derive(Debug)]
pub struct OverlayChange {
    pub range: Option<(types::LineCol, types::LineCol)>,
    pub content: String,
    pub utf8_pos: bool,
}

#[derive(Default, Debug)]
pub struct VfsChangeBuffer {
    pub exit: bool,
    // Scan roots usually correspond to directories with a manifest file, but they don't have to.
    pub scan_roots: Vec<PathBuf>,
    pub force_scan_id: Option<u64>,
    pub overlay_updates: Vec<OverlayUpdate>,
}

impl VfsChangeBuffer {
    fn is_empty(&self) -> bool {
        !self.exit
            && self.scan_roots.is_empty()
            && self.force_scan_id.is_none()
            && self.overlay_updates.is_empty()
    }
}

#[derive(Default, Debug)]
pub struct CoreChangeBuffer {
    pub exit: bool,
    pub module_updates: Vec<workspace::ModuleUpdate>,
    pub last_force_scan_id: Option<u64>,
}

impl CoreChangeBuffer {
    fn is_empty(&self) -> bool {
        !self.exit && self.module_updates.is_empty() && self.last_force_scan_id.is_none()
    }
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

    pub fn signal_vfs_new_roots(&self, roots: Vec<PathBuf>) {
        if !roots.is_empty() {
            let mut vfs = self.vfs_changes.lock();
            vfs.scan_roots.extend(roots);
            self.notify_vfs();
        }
    }

    pub fn signal_vfs_force_scan(&self, id: u64) {
        let mut vfs = self.vfs_changes.lock();
        vfs.force_scan_id = Some(id);
        self.notify_vfs();
    }

    pub fn signal_vfs_overlay_updates(&self, updates: Vec<OverlayUpdate>) {
        if !updates.is_empty() {
            let mut vfs = self.vfs_changes.lock();
            vfs.overlay_updates.extend(updates);
            self.notify_vfs();
        }
    }

    pub fn signal_core_module_updates(
        &self,
        updates: Vec<workspace::ModuleUpdate>,
        force_scan_id: Option<u64>,
    ) {
        if !updates.is_empty() || force_scan_id.is_some() {
            let mut core = self.core_changes.lock();
            core.module_updates.extend(updates);
            if force_scan_id.is_some() {
                core.last_force_scan_id = force_scan_id;
            }
            self.notify_core();
        }
    }

    pub fn wait_vfs(&self, timeout: Duration) -> (VfsChangeBuffer, bool) {
        let mut vfs = self.vfs_changes.lock();
        let timed_out = if vfs.is_empty() {
            let res = self.vfs_condvar.wait_for(&mut vfs, timeout);
            res.timed_out()
        } else {
            false
        };
        (take(&mut vfs), timed_out)
    }

    pub fn wait_core(&self) -> CoreChangeBuffer {
        let mut core = self.core_changes.lock();
        if core.is_empty() {
            self.core_condvar.wait(&mut core);
        }
        take(&mut core)
    }
}

impl Default for SyncState {
    fn default() -> Self {
        Self::new()
    }
}
