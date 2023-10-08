// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context;
use fennec_common::{types, util, MODULE_MANIFEST_FILENAME};
use std::{
    cmp::Ordering,
    io::Read,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
    vec,
};

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
                let disp = self.path.display(); // TODO: we duplicate display from below
                log::warn!(r#"failed to read content of "{disp}", ignoring: {err}"#);
                None
            }
        };
        self.content.is_some()
    }

    fn read_utf8_lossy(path: &Path) -> Result<Arc<str>, anyhow::Error> {
        let mut file = std::fs::File::open(path).with_context(|| {
            let disp = path.display();
            format!(r#"failed to open source file "{disp}""#)
        })?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).with_context(|| {
            let disp = path.display();
            format!(r#"failed to read source file "{disp}""#)
        })?;
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
        File {
            path: take(&mut file.path),
            deleted: true,
            content_changed: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct Directory {
    name: String,
    depth: usize,
    deleted: bool,
    _content_changed: Option<bool>,
    subdirectories: Vec<usize>, // sorted by directory name
    source_files: Vec<File>,    // sorted by file name
}

impl Directory {
    fn new(name: String, depth: usize) -> Directory {
        Directory {
            name,
            depth,
            ..Default::default()
        }
    }

    fn take_existing(dir: &mut Directory, modified: bool) -> Directory {
        assert!(!dir.deleted);
        let mut subdirs = take(&mut dir.subdirectories);
        subdirs.truncate(0);
        Directory {
            name: take(&mut dir.name),
            depth: dir.depth,
            deleted: false,
            _content_changed: Some(modified),
            subdirectories: subdirs,
            source_files: take(&mut dir.source_files),
        }
    }

    fn take_deleted(dir: &mut Directory) -> Directory {
        assert!(!dir.deleted);
        Directory {
            name: take(&mut dir.name),
            depth: dir.depth,
            deleted: true,
            _content_changed: Some(true),
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct TreeBuildState {
    tree: Vec<Directory>,
    cur_depth: usize,
    cur_dir_ix: usize,
    parents: Vec<Option<usize>>,   // parent index of i'th directory
    subdirectories_at: Vec<usize>, // next index to visit in subdirectories of i'th directory
}

impl TreeBuildState {
    fn reset(&mut self) {
        self.tree = vec![Directory::new(String::new(), 0)]; // root entry
        self.cur_depth = 0;
        self.cur_dir_ix = 0;
        self.parents.resize(1, None);
        self.subdirectories_at.resize(1, 0);
    }

    fn navigate(&mut self, depth: usize) {
        if depth != self.cur_depth {
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
        }
        // Next entry will be an immediate child of the current directory.
    }

    fn add_dir(&mut self, dir: Directory) {
        let dir_ix = self.tree.len();
        self.parents.push(Some(self.cur_dir_ix));
        self.tree.push(dir);
        self.tree[self.cur_dir_ix].subdirectories.push(dir_ix);
    }

    fn add_file(&mut self, file: File) {
        self.tree[self.cur_dir_ix].source_files.push(file);
    }

    fn take_tree(&mut self) -> Vec<Directory> {
        take(&mut self.tree)
    }
}

pub struct Vfs {
    poll_interval: Duration,
    scan_roots: Vec<PathBuf>, // sorted; no element is a prefix of another element
    scan_trees: Vec<Vec<Directory>>, // same order as scan_roots
    // TODO: unify above vectors in one?
    scan_state: TreeBuildState,
}

impl Vfs {
    #[must_use]
    pub fn new() -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
            scan_roots: Vec::default(),
            scan_trees: Vec::default(),
            scan_state: TreeBuildState::default(),
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
                self.scan();
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
                // TODO: self.scan_trees.insert(ix, Directory::default());
                true
            }
        }
    }

    fn scan(&mut self) {
        for (root, prev_tree) in self
            .scan_roots
            .iter()
            .zip(take(&mut self.scan_trees).into_iter())
        {
            let tree = Self::scan_root(root, &mut self.scan_state);
            let _ = Self::merge_hydrate_sorted_preorder_dirs(tree, prev_tree, &mut self.scan_state);
            // TODO
        }
    }

    fn scan_root(root: &PathBuf, state: &mut TreeBuildState) -> Vec<Directory> {
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
                    let name = entry
                        .file_name()
                        .to_str()
                        .expect("is_valid_utf8_visible() must ensure UTF-8");
                    if typ.is_dir() {
                        let dir = Directory::new(name.to_owned(), state.cur_depth);
                        state.add_dir(dir);
                    } else if typ.is_file()
                        && (name == MODULE_MANIFEST_FILENAME || util::valid_source_file_name(name))
                    {
                        let modified = entry.metadata().ok().and_then(|meta| meta.modified().ok());
                        let path = entry.into_path();
                        let file = File::new(path, modified);
                        state.add_file(file);
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
        state: &mut TreeBuildState,
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
                        match (dir.depth, &dir.name).cmp(&(prev_dir.depth, &prev_dir.name)) {
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
                    let diff = dir
                        .source_files
                        .iter()
                        .any(|f| f.content_changed.expect("change marker must be set"));
                    state.add_dir(Directory::take_existing(dir, diff));
                    dir_iter.next();
                    prev_dir_iter.next();
                }
                (Some(dir), None) => {
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

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
