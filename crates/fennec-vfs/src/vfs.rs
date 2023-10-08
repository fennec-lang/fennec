// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context;
use fennec_common::{types, util, MODULE_MANIFEST_FILENAME};
use std::{
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
    vec,
};

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(991);

#[derive(Default)]
struct File {
    path: PathBuf,
    content: Option<Arc<str>>,
    modified: Option<SystemTime>,
    content_changed: Option<bool>,
}

impl File {
    fn new(path: PathBuf, modified: Option<SystemTime>) -> File {
        File {
            path,
            content: None,
            modified,
            content_changed: None,
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
        let res = read_file_content_utf8_lossy(&self.path);
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

    fn take_existing(file: &mut File) -> File {
        assert!(file.content.is_some());
        let mut f = std::mem::take(file);
        f.content_changed = Some(false);
        f
    }

    fn take_deleted(file: &mut File) -> File {
        assert!(file.content.is_some());
        File {
            path: std::mem::take(&mut file.path),
            content: None,
            modified: None,
            content_changed: Some(true),
        }
    }
}

struct DirectoryRef {
    _name: String,
    index: Option<usize>,
}

impl DirectoryRef {
    fn new(name: String, index: usize) -> DirectoryRef {
        DirectoryRef {
            _name: name,
            index: Some(index),
        }
    }
}

struct Directory {
    parent: Option<usize>,
    subdirectories: Vec<DirectoryRef>,
    subdirectories_at: usize, // next index to visit in subdirectories
    source_files: Vec<File>,
}

impl Directory {
    fn new(parent: Option<usize>) -> Directory {
        Directory {
            parent,
            subdirectories: Vec::default(),
            subdirectories_at: 0,
            source_files: Vec::default(),
        }
    }
}

pub struct Vfs {
    poll_interval: Duration,
    scan_roots: Vec<PathBuf>, // sorted; no element is a prefix of another element
    scan_trees: Vec<Vec<Directory>>, // same order as scan_roots
                              // TODO: unify vectors in one
}

impl Vfs {
    #[must_use]
    pub fn new() -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
            scan_roots: Vec::default(),
            scan_trees: Vec::default(),
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
        for (root, prev_tree) in self.scan_roots.iter().zip(self.scan_trees.iter_mut()) {
            let mut tree = Self::scan_root(root);
            Self::hydrate_root(&mut tree, prev_tree);
            // TODO
        }
    }

    fn scan_root(root: &PathBuf) -> Vec<Directory> {
        let walker = walkdir::WalkDir::new(root).sort_by_file_name().into_iter();
        let mut tree = vec![Directory::new(None)];
        let mut cur_depth = 0;
        let mut cur_dir_ix = 0;
        for entry in walker.filter_entry(|e| util::is_valid_utf8_visible(e.file_name())) {
            match entry {
                Ok(entry) => {
                    // Adjust the current directory and depth.
                    let depth = entry.depth();
                    if depth != cur_depth {
                        // Traverse up if necessary.
                        while cur_depth >= depth {
                            cur_dir_ix = tree[cur_dir_ix]
                                .parent
                                .expect("must not traverse outside the root");
                            cur_depth -= 1;
                        }
                        assert!(depth == cur_depth + 1);

                        // Navigate to the next subdirectory.
                        let parent = &mut tree[cur_dir_ix];
                        let subdir_at = parent.subdirectories_at;
                        parent.subdirectories_at += 1;
                        cur_dir_ix = parent.subdirectories[subdir_at]
                            .index
                            .expect("index must be set for all existing directories");
                        cur_depth = depth;
                    }

                    // Entry is an immediate child of the current directory; add it.
                    let typ = entry.file_type();
                    let name = entry
                        .file_name()
                        .to_str()
                        .expect("is_valid_utf8_visible() must ensure UTF-8");
                    if typ.is_dir() {
                        let dir = Directory::new(Some(cur_dir_ix));
                        let dir_ref = DirectoryRef::new(name.to_owned(), tree.len());
                        tree.push(dir);
                        tree[cur_dir_ix].subdirectories.push(dir_ref);
                    } else if typ.is_file()
                        && depth > 0
                        && (name == MODULE_MANIFEST_FILENAME || util::valid_source_file_name(name))
                    {
                        let modified = entry.metadata().ok().and_then(|meta| meta.modified().ok());
                        let path = entry.into_path();
                        let file = File::new(path, modified);
                        tree[cur_dir_ix].source_files.push(file);
                    }
                }
                Err(err) => {
                    log::warn!("error while scanning VFS, ignoring: {err}");
                }
            }
        }
        // If the scan root was a directory, tree[0] will contain it as a single subdirectory, tree[1].
        tree
    }

    fn hydrate_root(tree: &mut [Directory], prev_tree: &mut [Directory]) {
        let cur_dir_ix = 0;
        loop {
            // Loop start invariant: cur_dir_ix points to matching directories in tree and prev_tree.
            let dir = &mut tree[cur_dir_ix];
            let prev_dir = &mut prev_tree[cur_dir_ix];

            // For matching directories, merge files.
            let mut file_ix = 0;
            let mut prev_file_ix = 0;
            let mut merged_files: Vec<File> = Vec::with_capacity(dir.source_files.len());
            loop {
                match (
                    dir.source_files.get_mut(file_ix),
                    prev_dir.source_files.get_mut(prev_file_ix),
                ) {
                    (Some(file), Some(prev_file)) => {
                        match file.file_name().cmp(prev_file.file_name()) {
                            std::cmp::Ordering::Equal => {
                                if is_modified(file.modified, prev_file.modified)
                                    && file.read_content()
                                {
                                    merged_files.push(File::take_existing(file));
                                } else {
                                    merged_files.push(File::take_existing(prev_file));
                                }
                                file_ix += 1;
                                prev_file_ix += 1;
                            }
                            std::cmp::Ordering::Less => {
                                if file.read_content() {
                                    merged_files.push(File::take_existing(file));
                                }
                                file_ix += 1;
                            }
                            std::cmp::Ordering::Greater => {
                                merged_files.push(File::take_deleted(prev_file));
                                prev_file_ix += 1;
                            }
                        };
                    }
                    (Some(file), None) => {
                        if file.read_content() {
                            merged_files.push(File::take_existing(file));
                        }
                        file_ix += 1;
                    }
                    (None, Some(prev_file)) => {
                        merged_files.push(File::take_deleted(prev_file));
                        prev_file_ix += 1;
                    }
                    (None, None) => break,
                }
            }
        }
    }
}

fn is_modified(modified: Option<SystemTime>, prev_modified: Option<SystemTime>) -> bool {
    match (modified, prev_modified) {
        (Some(t), Some(prev_t)) => t != prev_t,
        _ => true, // conservative
    }
}

fn read_file_content_utf8_lossy(path: &Path) -> Result<Arc<str>, anyhow::Error> {
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

// TODO:
// - transform N fs trees into M module trees (after parsing the manifests), sorted by import path (preserving change indicators for files)
// - diff the module trees, producing the workspace update events
// - save new trees as the current trees

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
