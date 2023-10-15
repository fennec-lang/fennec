// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::{types, util, workspace, MODULE_MANIFEST_FILENAME};
use std::{
    cmp::Ordering,
    io::Read,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
    vec,
};

use crate::manifest;

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(991);

#[derive(Default)]
struct File {
    path: PathBuf,
    modified: Option<SystemTime>,
    deleted: bool,
    content_changed: Option<bool>,
    content: Option<Arc<str>>, // None initially or in case of read error
}

impl File {
    fn new(path: PathBuf, modified: Option<SystemTime>) -> File {
        File {
            path,
            modified,
            ..Default::default()
        }
    }

    fn file_name(&self) -> &str {
        self.path
            .file_name()
            .expect("file must have a valid file name")
            .to_str()
            .expect("file name must be valid UTF-8")
    }

    fn read_content(&mut self) -> bool {
        let res = Self::read_utf8_lossy(&self.path);
        self.content = match res {
            Ok(content) => Some(content),
            Err(err) => {
                let disp = self.path.display();
                log::warn!(r#"failed to read content of "{disp}", ignoring: {err}"#);
                None
            }
        };
        self.content.is_some()
    }

    fn read_utf8_lossy(path: &Path) -> Result<Arc<str>, anyhow::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let s = String::from_utf8_lossy(&buf);
        Ok(Arc::from(s))
        // Vec -> Cow -> Arc is quite a lot of copies, but hopefully guaranteed UTF-8 simplifies things downstream.
    }

    fn take_existing(file: &mut File, modified: bool) -> File {
        assert!(!file.deleted && file.content.is_some());
        let mut f = take(file);
        f.content_changed = Some(modified);
        f
    }

    fn take_deleted(file: &mut File) -> File {
        assert!(!file.deleted && file.content.is_some());
        let mut f = File::new(take(&mut file.path), None);
        f.content_changed = Some(true);
        f.deleted = true;
        f
    }
}

#[derive(Default)]
enum DirManifestState {
    #[default]
    None, // used in case of deleted manifest as well
    Same, // used in case of parse error as well
    Changed(workspace::ModuleManifest),
}

impl DirManifestState {
    fn is_some(&self) -> bool {
        match self {
            DirManifestState::Changed(_) | DirManifestState::Same => true,
            DirManifestState::None => false,
        }
    }
}

#[derive(Default)]
struct Directory {
    path: PathBuf,
    depth: usize,
    deleted: bool,
    content_changed: Option<bool>,
    subdirectories: Vec<usize>, // sorted by directory name
    source_files: Vec<File>,    // sorted by file name
    manifest: DirManifestState,
}

impl Directory {
    fn new(path: PathBuf, depth: usize) -> Directory {
        Directory {
            path,
            depth,
            ..Default::default()
        }
    }

    fn name(&self) -> &str {
        self.path
            .file_name()
            .expect("directory must have a valid file name")
            .to_str()
            .expect("directory name must be valid UTF-8")
    }

    fn fill_manifest(&mut self) {
        self.manifest = self.get_manifest();
    }

    fn get_manifest(&self) -> DirManifestState {
        let ins = self
            .source_files
            .binary_search_by_key(&MODULE_MANIFEST_FILENAME, |f| f.file_name());
        match ins {
            Ok(ix) => {
                let m = &self.source_files[ix];
                let changed = m.content_changed.expect("change indicator must be set");
                if !changed {
                    return DirManifestState::Same;
                }
                if m.deleted {
                    return DirManifestState::None;
                }
                let content = m
                    .content
                    .as_ref()
                    .expect("changed file must contain content")
                    .as_ref();
                let res = manifest::parse(content);
                match res {
                    Ok(manifest) => DirManifestState::Changed(manifest),
                    Err(err) => {
                        let disp = m.path.display();
                        log::warn!(r#"failed to parse module manifest "{disp}", ignoring: {err}"#);
                        DirManifestState::Same
                    }
                }
            }
            Err(_) => DirManifestState::None,
        }
    }

    fn take_existing(dir: &mut Directory, modified: bool) -> Directory {
        assert!(!dir.deleted);
        let mut d = take(dir);
        d.content_changed = Some(modified);
        d.subdirectories.truncate(0);
        d
    }

    fn take_deleted(dir: &mut Directory) -> Directory {
        assert!(!dir.deleted);
        let mut d = Directory::new(take(&mut dir.path), dir.depth);
        d.content_changed = Some(true);
        d.deleted = true;
        d
    }
}

#[derive(Default)]
struct DirTreeBuildState {
    tree: Vec<Directory>,
    cur_depth: usize,
    cur_dir_ix: usize,
    parents: Vec<Option<usize>>,   // parent index of i'th directory
    subdirectories_at: Vec<usize>, // next index to visit in subdirectories of i'th directory
}

impl DirTreeBuildState {
    fn reset(&mut self) {
        self.tree = vec![Directory::new(PathBuf::new(), 0)]; // root entry
        self.cur_depth = 0;
        self.cur_dir_ix = 0;
        self.parents.resize(1, None);
        self.subdirectories_at.resize(1, 0);
    }

    fn navigate(&mut self, depth: usize) {
        if depth == self.cur_depth {
            return;
        }

        // Traverse up if necessary.
        while self.cur_depth >= depth {
            self.cur_dir_ix =
                self.parents[self.cur_dir_ix].expect("must not traverse outside the root");
            self.cur_depth -= 1;
        }
        assert!(depth == self.cur_depth + 1);

        // Navigate to the next subdirectory.
        let parent = &mut self.tree[self.cur_dir_ix];
        let subdir_at = self.subdirectories_at[self.cur_dir_ix];
        self.subdirectories_at[self.cur_dir_ix] += 1;
        self.cur_dir_ix = parent.subdirectories[subdir_at];
        self.cur_depth = depth;
        // Next entry will be an immediate child of the current directory.
    }

    fn add_dir(&mut self, dir: Directory) {
        let dir_ix = self.tree.len();
        self.tree.push(dir);
        self.tree[self.cur_dir_ix].subdirectories.push(dir_ix);
        self.parents.push(Some(self.cur_dir_ix));
    }

    fn add_file(&mut self, file: File) {
        self.tree[self.cur_dir_ix].source_files.push(file);
    }

    fn take_tree(&mut self) -> Vec<Directory> {
        take(&mut self.tree)
    }
}

struct ScanState {
    root: PathBuf,
    tree: Vec<Directory>,
}

impl ScanState {
    fn new(root: PathBuf) -> ScanState {
        ScanState {
            root,
            tree: Vec::default(),
        }
    }
}

pub struct Vfs {
    poll_interval: Duration,
    cleanup_stale_roots: bool,
    scan_state: Vec<ScanState>, // sorted; no element is a prefix of another element
    scan_aux: DirTreeBuildState,
}

impl Vfs {
    #[must_use]
    pub fn new(cleanup_stale_roots: bool) -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
            cleanup_stale_roots,
            scan_state: Vec::default(),
            scan_aux: DirTreeBuildState::default(),
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

            if should_scan {
                let updates = self.scan();
                state.signal_vfs_updates(updates);
            }
        }
    }

    fn add_root(&mut self, root: PathBuf) -> bool {
        let ins = self.scan_state.binary_search_by_key(&&root, |s| &s.root);
        match ins {
            Ok(_) => false,
            Err(ix) => {
                if let Some(prev_state) = self.scan_state.get(ix - 1) {
                    if root.strip_prefix(&prev_state.root).is_ok() {
                        // Trying to add subdirectory of an existing root; do nothing.
                        return false;
                    }
                }
                self.scan_state.insert(ix, ScanState::new(root));
                true
            }
        }
    }

    fn scan(&mut self) -> Vec<workspace::ModuleUpdate> {
        let mut ret: Vec<workspace::ModuleUpdate> = Vec::new();
        for state in &mut self.scan_state {
            let scan_tree = Self::scan_root(&state.root, &mut self.scan_aux);
            state.tree = Self::merge_hydrate_sorted_preorder_dirs(
                scan_tree,
                take(&mut state.tree),
                &mut self.scan_aux,
            );
            let updates = Self::build_module_updates(&state.tree);
            ret.extend(updates);
        }
        if self.cleanup_stale_roots {
            self.scan_state
                .retain_mut(|s| s.tree.iter().any(|dir| dir.manifest.is_some()));
        }
        // Ensure binary search can be used on the updates.
        // Doing this here and not outside feels better because when sorted invariant
        // is encapsulated, we are free to use a different implementation in VFS if necessary.
        ret.sort_unstable_by_key(|_u| (0));
        ret
    }

    fn build_module_updates(_tree: &[Directory]) -> Vec<workspace::ModuleUpdate> {
        todo!()
    }

    // TODO: use valid_package_name

    fn scan_root(root: &PathBuf, state: &mut DirTreeBuildState) -> Vec<Directory> {
        state.reset();
        let walker = walkdir::WalkDir::new(root).sort_by_file_name().into_iter();
        for entry in walker.filter_entry(|e| util::is_valid_utf8_visible(e.file_name())) {
            match entry {
                Ok(entry) => {
                    let depth = entry.depth();
                    state.navigate(depth);
                    if depth == 0 {
                        continue;
                    }
                    let typ = entry.file_type();
                    if typ.is_dir() {
                        let dir = Directory::new(entry.into_path(), state.cur_depth);
                        state.add_dir(dir);
                    } else if typ.is_file() {
                        let name = entry
                            .file_name()
                            .to_str()
                            .expect("is_valid_utf8_visible() must ensure UTF-8");
                        if name == MODULE_MANIFEST_FILENAME || util::valid_source_file_name(name) {
                            let modified =
                                entry.metadata().ok().and_then(|meta| meta.modified().ok());
                            let file = File::new(entry.into_path(), modified);
                            state.add_file(file);
                        }
                    }
                }
                Err(err) => {
                    log::warn!("error while scanning VFS, ignoring: {err}");
                }
            }
        }
        state.take_tree()
    }

    fn merge_hydrate_sorted_preorder_dirs(
        dirs: Vec<Directory>,
        prev_dirs: Vec<Directory>,
        state: &mut DirTreeBuildState,
    ) -> Vec<Directory> {
        state.reset();
        let mut dir_iter = dirs.into_iter().peekable();
        let mut prev_dir_iter = prev_dirs.into_iter().filter(|d| !d.deleted).peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching (by depth and name) directories.
            // Loop invariant: parents of both cur and prev have already been visited (preorder traversal).
            let (cur, prev) = match (dir_iter.peek_mut(), prev_dir_iter.peek_mut()) {
                (None, None) => break,
                (Some(dir), Some(prev_dir)) => match dir.depth.cmp(&prev_dir.depth) {
                    Ordering::Equal => {
                        match (dir.depth, dir.name()).cmp(&(prev_dir.depth, prev_dir.name())) {
                            Ordering::Equal => (Some(dir), Some(prev_dir)),
                            Ordering::Less => (Some(dir), None),
                            Ordering::Greater => (None, Some(prev_dir)),
                        }
                    }
                    Ordering::Less => (None, Some(prev_dir)),
                    Ordering::Greater => (Some(dir), None),
                },
                only_one => only_one,
            };
            let depth = match (&cur, &prev) {
                (Some(dir), _) => dir.depth,
                (_, Some(prev_dir)) => prev_dir.depth,
                _ => unreachable!("at least one directory must be set"),
            };
            state.navigate(depth);
            if depth == 0 {
                continue;
            }
            match (cur, prev) {
                (Some(dir), Some(prev_dir)) => {
                    dir.source_files = Self::merge_hydrate_sorted_files(
                        take(&mut dir.source_files),
                        take(&mut prev_dir.source_files),
                    );
                    dir.fill_manifest();
                    let diff = dir
                        .source_files
                        .iter()
                        .any(|f| f.content_changed.expect("change marker must be set"));
                    state.add_dir(Directory::take_existing(dir, diff));
                    dir_iter.next();
                    prev_dir_iter.next();
                }
                (Some(dir), None) => {
                    dir.source_files =
                        Self::merge_hydrate_sorted_files(take(&mut dir.source_files), Vec::new());
                    dir.fill_manifest();
                    state.add_dir(Directory::take_existing(dir, true));
                    dir_iter.next();
                }
                (None, Some(prev_dir)) => {
                    state.add_dir(Directory::take_deleted(prev_dir));
                    prev_dir_iter.next();
                }
                _ => unreachable!("at least one directory must be set"),
            }
        }
        state.take_tree()
    }

    fn merge_hydrate_sorted_files(files: Vec<File>, prev_files: Vec<File>) -> Vec<File> {
        let mut merged_files: Vec<File> = Vec::with_capacity(files.len());
        let mut file_iter = files.into_iter().peekable();
        let mut prev_file_iter = prev_files.into_iter().filter(|f| !f.deleted).peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching files.
            let (cur, prev) = match (file_iter.peek_mut(), prev_file_iter.peek_mut()) {
                (None, None) => break,
                (Some(file), Some(prev_file)) => {
                    match file.file_name().cmp(prev_file.file_name()) {
                        Ordering::Equal => (Some(file), Some(prev_file)),
                        Ordering::Less => (Some(file), None),
                        Ordering::Greater => (None, Some(prev_file)),
                    }
                }
                only_one => only_one,
            };
            match (cur, prev) {
                (Some(file), Some(prev_file)) => {
                    let modified = match (file.modified, prev_file.modified) {
                        (Some(t), Some(prev_t)) => t != prev_t,
                        _ => true, // conservative
                    };
                    if modified && file.read_content() {
                        let diff = file.content != prev_file.content;
                        merged_files.push(File::take_existing(file, diff));
                    } else {
                        merged_files.push(File::take_existing(prev_file, false));
                    }
                    file_iter.next();
                    prev_file_iter.next();
                }
                (Some(file), None) => {
                    if file.read_content() {
                        merged_files.push(File::take_existing(file, true));
                    }
                    file_iter.next();
                }
                (None, Some(prev_file)) => {
                    merged_files.push(File::take_deleted(prev_file));
                    prev_file_iter.next();
                }
                _ => unreachable!("at least one file must be set"),
            }
        }
        merged_files // sorted by file name
    }
}

// TODO:
// - transform N fs trees into M module trees (after parsing the manifests), sorted by import path (preserving change indicators for files)
// - diff the module trees, producing the workspace update events
// - save new trees as the current trees
