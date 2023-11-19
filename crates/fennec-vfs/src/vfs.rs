// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use debug_ignore::DebugIgnore;
use fennec_common::{
    types, util,
    workspace::{self, FileUpdate},
    MODULE_MANIFEST_FILENAME,
};
use std::{
    cmp::Ordering,
    io::Read,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
    vec,
};

use crate::manifest;

pub const DEFAULT_VFS_POLL_INTERVAL: Duration = Duration::from_millis(991);

#[derive(Default, Clone, Debug)]
struct File {
    path: PathBuf,
    meta: Option<DebugIgnore<std::fs::Metadata>>,
    deleted: bool,
    content_changed: Option<bool>,
    content: Option<Arc<str>>, // None initially or in case of read error, and also for deleted files
}

impl File {
    fn new(path: PathBuf, meta: Option<std::fs::Metadata>) -> File {
        File {
            path,
            meta: meta.map(std::convert::Into::into),
            ..Default::default()
        }
    }

    fn new_from_existing(mut file: File, content_changed: bool) -> File {
        assert!(!file.deleted && file.content.is_some());
        file.content_changed = Some(content_changed);
        file
    }

    fn new_deleted(path: PathBuf) -> File {
        File {
            path,
            deleted: true,
            content_changed: Some(true),
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

    #[must_use]
    fn is_manifest(&self) -> bool {
        self.file_name() == MODULE_MANIFEST_FILENAME
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
}

#[derive(Clone, Default, Debug)]
struct ManifestInfo {
    manifest: Option<workspace::ModuleManifest>, // empty if valid manifest does not currently exist
    manifest_changed: bool,
}

#[derive(Clone, Default, Debug)]
struct Directory {
    path: PathBuf,
    depth: isize,
    deleted: bool,
    subdirectories: Vec<usize>, // sorted by directory name
    source_files: Vec<File>,    // sorted by file name; manifest file is included
    content_changed: Option<bool>,
    manifest_info: ManifestInfo,
}

impl Directory {
    fn new(path: PathBuf, depth: isize) -> Directory {
        Directory {
            path,
            depth,
            ..Default::default()
        }
    }

    fn new_from_existing(mut dir: Directory) -> Directory {
        assert!(!dir.deleted);
        dir.subdirectories.truncate(0);
        dir.content_changed = None;
        dir
    }

    fn new_deleted(path: PathBuf, depth: isize) -> Directory {
        Directory {
            path,
            depth,
            deleted: true,
            ..Default::default()
        }
    }

    fn name(&self) -> &str {
        self.path.file_name().map_or("" /* root entry */, |s| {
            s.to_str().expect("directory name must be valid UTF-8")
        })
    }

    fn module(&self) -> Option<&types::ImportPath> {
        self.manifest_info.manifest.as_ref().map(|m| &m.module)
    }

    fn set_files(&mut self, files: Vec<File>, prev_manifest: Option<workspace::ModuleManifest>) {
        self.source_files = files;
        self.content_changed = Some(
            self.source_files
                .iter()
                .any(|f| f.content_changed.expect("change marker must be set")),
        );
        self.manifest_info = self.get_manifest(prev_manifest);
    }

    fn manifest_pos(&self) -> Option<usize> {
        self.source_files
            .binary_search_by_key(&MODULE_MANIFEST_FILENAME, |f| f.file_name())
            .ok()
    }

    fn get_manifest(&self, prev_manifest: Option<workspace::ModuleManifest>) -> ManifestInfo {
        let ix = self.manifest_pos();
        if let Some(ix) = ix {
            let m = &self.source_files[ix];
            if m.deleted {
                return ManifestInfo {
                    manifest: None,
                    manifest_changed: prev_manifest.is_some(),
                };
            }
            let changed = m.content_changed.expect("change indicator must be set");
            if !changed {
                return ManifestInfo {
                    manifest: prev_manifest,
                    manifest_changed: false,
                };
            }
            let content = m
                .content
                .as_ref()
                .expect("changed file must contain content")
                .as_ref();
            let res = manifest::parse(content);
            match res {
                Ok(manifest) => ManifestInfo {
                    manifest_changed: prev_manifest.map_or(true, |prev| manifest != prev),
                    manifest: Some(manifest),
                },
                Err(err) => {
                    let disp = m.path.display();
                    log::warn!(r#"failed to parse module manifest "{disp}": {err}"#);
                    // We might go back to pretending that the manifest has not changed instead,
                    // as this avoids the module removal. However, that breaks the invariant that
                    // the VFS state is equal to the fresh re-scan, so let's not do that.
                    ManifestInfo {
                        manifest: None,
                        manifest_changed: prev_manifest.is_some(),
                    }
                }
            }
        } else {
            ManifestInfo {
                manifest: None,
                manifest_changed: false,
            }
        }
    }
}

struct DirTreeBuildState {
    tree: Vec<Directory>,
    cur_depth: isize,
    cur_parent_ix: usize,
    parents: Vec<Option<usize>>,   // parent index of i'th directory
    subdirectories_at: Vec<usize>, // next index to visit in subdirectories of i'th directory
}

impl DirTreeBuildState {
    // We could play tricks with reuse of the state between scans/traversals,
    // but let's avoid premature optimization for now.
    fn new() -> DirTreeBuildState {
        DirTreeBuildState {
            tree: vec![Directory::new(PathBuf::new(), -1)], // root-of-root entry
            cur_depth: -1,
            cur_parent_ix: 0,
            parents: vec![None],
            subdirectories_at: vec![0],
        }
    }

    fn navigate(&mut self, depth: isize, parent: Option<&str>) {
        match depth.cmp(&self.cur_depth) {
            Ordering::Equal => {
                // Traversing the current layer, no need to adjust parent.
            }
            Ordering::Greater => {
                let cur_parent = &self.tree[self.cur_parent_ix];
                if depth == cur_parent.depth + 2 {
                    // Need to move parent one layer down, to the next matching subdirectory.
                    let next_parent_ix = loop {
                        let candidate_subdir_ix = self.subdirectories_at[self.cur_parent_ix];
                        self.subdirectories_at[self.cur_parent_ix] += 1;
                        let parent_candidate_ix = cur_parent.subdirectories[candidate_subdir_ix];
                        let candidate = &self.tree[parent_candidate_ix];
                        if Some(candidate.name()) == parent {
                            break parent_candidate_ix;
                        }
                    };
                    self.cur_parent_ix = next_parent_ix;
                }
            }
            Ordering::Less => {
                // Need to navigate up.
                while self.tree[self.cur_parent_ix].depth >= depth {
                    self.cur_parent_ix = self.parents[self.cur_parent_ix]
                        .expect("must not traverse outside the root");
                }
            }
        }
        self.cur_depth = depth;
    }

    fn add_dir(&mut self, dir: Directory) {
        let dir_ix = self.tree.len();
        self.tree.push(dir);
        self.tree[self.cur_parent_ix].subdirectories.push(dir_ix);
        self.parents.push(Some(self.cur_parent_ix));
        self.subdirectories_at.push(0);
    }

    fn add_file(&mut self, file: File) {
        self.tree[self.cur_parent_ix].source_files.push(file);
    }

    fn take_tree(&mut self) -> Vec<Directory> {
        take(&mut self.tree)
    }
}

struct ScanState {
    root: PathBuf,
    tree: Vec<Directory>,
    modules: Vec<ModTraverse>,
}

impl ScanState {
    fn new(root: PathBuf) -> ScanState {
        ScanState {
            root,
            tree: Vec::default(),
            modules: Vec::default(),
        }
    }
}

#[derive(Default, Debug)]
struct ModTraverse {
    root: usize,
    packages: Vec<usize>,
    depth: isize,
    cur_ignore_pkgs_depth: Option<isize>,
}

impl ModTraverse {
    fn new(root: usize, depth: isize) -> ModTraverse {
        ModTraverse {
            root,
            depth,
            ..Default::default()
        }
    }

    fn index_root<'a>(&self, tree: &'a [Directory]) -> &'a Directory {
        &tree[self.root]
    }
}

pub struct Vfs {
    poll_interval: Duration,
    cleanup_stale_roots: bool,
    scan_state: Vec<ScanState>, // sorted; no element is a prefix of another element
}

impl Vfs {
    #[must_use]
    pub fn new(cleanup_stale_roots: bool, poll_interval: Duration) -> Vfs {
        Vfs {
            poll_interval,
            cleanup_stale_roots,
            scan_state: Vec::default(),
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

            let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
            let mut should_scan = changes.force_scan_id.is_some() || timed_out;
            for root in changes.scan_roots {
                // We don't want to re-scan if we got notified about a root we are already watching.
                should_scan |= self.add_root(root, &mut updates);
            }

            if should_scan {
                self.scan(&mut updates);
                state.signal_core_module_updates(updates, changes.force_scan_id);
            }
        }
    }

    pub fn __test_add_root(&mut self, root: PathBuf) -> Vec<workspace::ModuleUpdate> {
        let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
        self.add_root(root, &mut updates);
        updates
    }

    pub fn __test_scan(&mut self) -> Vec<workspace::ModuleUpdate> {
        let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
        self.scan(&mut updates);
        updates
    }

    fn add_root(&mut self, root: PathBuf, updates: &mut Vec<workspace::ModuleUpdate>) -> bool {
        let ins = self.scan_state.binary_search_by_key(&&root, |s| &s.root);
        match ins {
            Ok(_) => false,
            Err(ix) => {
                if ix > 0 {
                    if let Some(prev_state) = self.scan_state.get(ix - 1) {
                        if util::has_prefix(&root, &prev_state.root) {
                            // Trying to add subdirectory of an existing root; do nothing.
                            return false;
                        }
                    }
                }
                while let Some(next_state) = self.scan_state.get(ix) {
                    if util::has_prefix(&next_state.root, &root) {
                        // Trying to add parent of an existing root; remove old one instead.
                        let state = self.scan_state.remove(ix);
                        for module in state.modules {
                            let dir = module.index_root(&state.tree);
                            updates.push(workspace::ModuleUpdate {
                                source: dir.path.clone(),
                                module: dir.module().cloned(),
                                manifest: workspace::ModuleManifestUpdate::Unknown,
                                packages: Vec::new(),
                                update: workspace::ModuleUpdateKind::ModuleRemoved,
                            });
                        }
                    } else {
                        break;
                    }
                }
                self.scan_state.insert(ix, ScanState::new(root));
                true
            }
        }
    }

    fn scan(&mut self, updates: &mut Vec<workspace::ModuleUpdate>) {
        for state in &mut self.scan_state {
            let scan_tree = Self::scan_root(&state.root);
            log::trace!("scan tree {scan_tree:#?}");
            let tree = Self::merge_hydrate_sorted_preorder_dirs(scan_tree, &state.tree);
            log::trace!("tree {tree:#?}");
            let tree_modules = Self::build_modules(&tree);
            log::trace!("modules {tree_modules:#?}");
            let tree_updates =
                Self::build_module_updates(&tree, &tree_modules, &state.tree, &state.modules);
            log::trace!("updates {tree_updates:#?}");
            updates.extend(tree_updates);
            state.tree = tree;
            state.modules = tree_modules;
        }
        if self.cleanup_stale_roots {
            // Since we only clean up after all manifest entries are removed from the tree,
            // by that time we have already reported that the modules are deleted;
            // no need to special-case scan root removal in module diff calculation.
            self.scan_state.retain(|s| {
                let any_manifest = s.tree.iter().any(|dir| {
                    if let Some(ix) = dir.manifest_pos() {
                        !dir.source_files[ix].deleted
                    } else {
                        false
                    }
                });
                assert!(any_manifest || s.modules.is_empty());
                any_manifest
            });
        }
    }

    fn build_modules(tree: &[Directory]) -> Vec<ModTraverse> {
        let mut modules: Vec<ModTraverse> = Vec::new();
        let mut stack: Vec<ModTraverse> = Vec::new();
        for (ix, dir) in tree.iter().enumerate() {
            // Ignore all deleted directories.
            if dir.deleted {
                continue;
            }
            // Pop finished modules off the stack until the depth is increasing
            // (equal depth means we are processing a potential new module at the same depth, so old one is done).
            while !stack.is_empty() && dir.depth <= stack.last().expect("stack is not empty").depth
            {
                modules.push(stack.pop().expect("stack is not empty"));
            }
            if let Some(m_ix) = dir.manifest_pos() {
                if !dir.source_files[m_ix].deleted {
                    // Push new module on the stack, if we are visiting an existing module root.
                    let m = ModTraverse::new(ix, dir.depth);
                    stack.push(m);
                }
            }
            // If we are not inside a module, do nothing.
            if stack.is_empty() {
                continue;
            }
            // Inside a module, process the potential package.
            let m = stack.last_mut().expect("stack is not empty");
            let root_pkg = dir.depth == m.depth;
            // Ensure that we are either processing a root package
            // (where we don't care about the directory name
            // as we use the import path from the module manifest instead)
            // or the directory name is a valid package name.
            if !root_pkg {
                // Maybe we have finished a subtree with an invalid path element?
                let ignore_finished = match m.cur_ignore_pkgs_depth {
                    Some(depth) => dir.depth <= depth,
                    None => true,
                };
                if ignore_finished {
                    m.cur_ignore_pkgs_depth = None;
                } else {
                    // We are still processing an ignored subtree where we are only interested in new module roots.
                    continue;
                }
                // Maybe we have to start a new ignore subtree?
                let valid = util::valid_package_name(dir.name());
                if !valid {
                    m.cur_ignore_pkgs_depth = Some(dir.depth);
                    continue;
                }
            }
            m.packages.push(ix);
        }
        modules.extend(stack.into_iter().rev());
        // Sort modules so that we can later merge them.
        modules.sort_unstable_by(|a, b| {
            let a_dir = a.index_root(tree);
            let b_dir = b.index_root(tree);
            (a_dir.module(), &a_dir.path).cmp(&(b_dir.module(), &b_dir.path))
        });
        modules
    }

    fn build_module_updates(
        tree: &[Directory],
        modules: &[ModTraverse],
        prev_tree: &[Directory],
        prev_modules: &[ModTraverse],
    ) -> Vec<workspace::ModuleUpdate> {
        let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
        let mut mod_iter = modules.iter().peekable();
        let mut prev_mod_iter = prev_modules.iter().peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching modules.
            let (cur, prev) = match (mod_iter.peek(), prev_mod_iter.peek()) {
                (None, None) => break,
                (Some(module), Some(prev_module)) => {
                    let mod_dir = module.index_root(tree);
                    let prev_mod_dir = prev_module.index_root(prev_tree);
                    match (mod_dir.module(), &mod_dir.path)
                        .cmp(&(prev_mod_dir.module(), &prev_mod_dir.path))
                    {
                        Ordering::Equal => (Some(module), Some(prev_module)),
                        Ordering::Less => (Some(module), None),
                        Ordering::Greater => (None, Some(prev_module)),
                    }
                }
                only_one => only_one,
            };
            match (cur, prev) {
                (Some(module), Some(prev_module)) => {
                    let dir = module.index_root(tree);
                    let packages = Self::build_package_updates(
                        dir.module(),
                        &dir.path,
                        &module.packages,
                        tree,
                        &prev_module.packages,
                        prev_tree,
                    );
                    if dir.manifest_info.manifest_changed || !packages.is_empty() {
                        let manifest = if dir.manifest_info.manifest_changed {
                            workspace::ModuleManifestUpdate::Updated(
                                dir.manifest_info.manifest.clone(),
                            )
                        } else {
                            workspace::ModuleManifestUpdate::Unknown
                        };
                        updates.push(workspace::ModuleUpdate {
                            source: dir.path.clone(),
                            module: dir.module().cloned(),
                            manifest,
                            packages, // may be empty
                            update: workspace::ModuleUpdateKind::ModuleUpdated,
                        });
                    }
                    mod_iter.next();
                    prev_mod_iter.next();
                }
                (Some(module), None) => {
                    let dir = module.index_root(tree);
                    let packages = Self::build_package_updates(
                        dir.module(),
                        &dir.path,
                        &module.packages,
                        tree,
                        &[],
                        &[],
                    );
                    updates.push(workspace::ModuleUpdate {
                        source: dir.path.clone(),
                        module: dir.module().cloned(),
                        manifest: workspace::ModuleManifestUpdate::Updated(
                            dir.manifest_info.manifest.clone(),
                        ),
                        packages,
                        update: workspace::ModuleUpdateKind::ModuleAdded,
                    });
                    mod_iter.next();
                }
                (None, Some(prev_module)) => {
                    let dir = prev_module.index_root(prev_tree);
                    updates.push(workspace::ModuleUpdate {
                        source: dir.path.clone(),
                        module: dir.module().cloned(),
                        manifest: workspace::ModuleManifestUpdate::Unknown,
                        packages: Vec::new(),
                        update: workspace::ModuleUpdateKind::ModuleRemoved,
                    });
                    prev_mod_iter.next();
                }
                _ => unreachable!("at least one module must be set"),
            };
        }
        updates
    }

    fn build_package_updates(
        mod_path: Option<&types::ImportPath>,
        mod_dir: &Path,
        packages: &[usize],
        tree: &[Directory],
        prev_packages: &[usize],
        prev_tree: &[Directory],
    ) -> Vec<workspace::PackageUpdate> {
        let mut updates: Vec<workspace::PackageUpdate> = Vec::new();
        let mut package_dir_iter = packages.iter().map(|ix| &tree[*ix]).peekable();
        let mut prev_package_dir_iter = prev_packages.iter().map(|ix| &prev_tree[*ix]).peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching package directories.
            let (cur, prev) = match (package_dir_iter.peek(), prev_package_dir_iter.peek()) {
                (None, None) => break,
                (Some(dir), Some(prev_dir)) => match dir.path.cmp(&prev_dir.path) {
                    Ordering::Equal => (Some(dir), Some(prev_dir)),
                    Ordering::Less => (Some(dir), None),
                    Ordering::Greater => (None, Some(prev_dir)),
                },
                only_one => only_one,
            };
            match (cur, prev) {
                (Some(dir), Some(_)) => {
                    let files: Vec<workspace::FileUpdate> = dir
                        .source_files
                        .iter()
                        .filter(|f| {
                            f.content_changed.expect("change marker must be set")
                                && !f.is_manifest()
                        })
                        .map(|f| workspace::FileUpdate {
                            source: f.path.clone(),
                            content: f.content.clone(),
                        })
                        .collect();
                    if !files.is_empty() {
                        updates.push(workspace::PackageUpdate {
                            source: dir.path.clone(),
                            path: Self::pkg_import_path(mod_path, mod_dir, &dir.path),
                            files,
                            update: workspace::PackageUpdateKind::PackageUpdated,
                        });
                    }
                    package_dir_iter.next();
                    prev_package_dir_iter.next();
                }
                (Some(dir), None) => {
                    let files = dir
                        .source_files
                        .iter()
                        .filter(|f| !f.deleted && !f.is_manifest())
                        .map(|f| workspace::File {
                            source: f.path.clone(),
                            content: f
                                .content
                                .as_ref()
                                .expect("content must be set on all files in a package directory")
                                .clone(),
                        });
                    updates.push(workspace::PackageUpdate {
                        source: dir.path.clone(),
                        path: Self::pkg_import_path(mod_path, mod_dir, &dir.path),
                        files: files
                            .map(|f| FileUpdate {
                                source: f.source,
                                content: Some(f.content),
                            })
                            .collect(), // may be empty
                        update: workspace::PackageUpdateKind::PackageAdded,
                    });
                    package_dir_iter.next();
                }
                (None, Some(prev_dir)) => {
                    updates.push(workspace::PackageUpdate {
                        source: prev_dir.path.clone(),
                        path: Self::pkg_import_path(mod_path, mod_dir, &prev_dir.path),
                        files: Vec::new(),
                        update: workspace::PackageUpdateKind::PackageRemoved,
                    });
                    prev_package_dir_iter.next();
                }
                _ => unreachable!("at least one package must be set"),
            };
        }
        updates
    }

    fn pkg_import_path(
        path: Option<&types::ImportPath>,
        mod_dir: &Path,
        pkg_dir: &Path,
    ) -> Option<types::ImportPath> {
        path.map(|path| {
            let rel = pkg_dir
                .strip_prefix(mod_dir)
                .expect("package must be a module subdirectory")
                .to_str()
                .expect("package relative path must be valid UTF-8");
            if rel.is_empty() {
                path.clone() // root package of the module
            } else {
                path.join(rel)
                    .expect("join of package relative path to module import path must succeed")
            }
        })
    }

    fn scan_root(root: &PathBuf) -> Vec<Directory> {
        let mut state = DirTreeBuildState::new();
        let walker = walkdir::WalkDir::new(root).sort_by_file_name().into_iter(); // depth-first traversal
        for entry in
            walker.filter_entry(|e| e.depth() == 0 || util::is_valid_utf8_visible(e.file_name()))
        {
            match entry {
                Ok(entry) => {
                    let depth: isize = entry.depth().try_into().expect("depth must be valid isize");
                    let parent = path_parent_filename(entry.path());
                    state.navigate(depth, parent);
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
                            let meta = entry.metadata().ok();
                            let file = File::new(entry.into_path(), meta);
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
        prev_dirs: &[Directory],
    ) -> Vec<Directory> {
        let mut state = DirTreeBuildState::new();
        let mut dir_iter = dirs.into_iter().skip(1).peekable();
        let mut prev_dir_iter = prev_dirs.iter().skip(1).filter(|d| !d.deleted).peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching (by depth and name) directories.
            // Loop invariant: parents of both cur and prev have already been visited (preorder traversal).
            let (cur, prev) = match (dir_iter.peek_mut(), prev_dir_iter.peek()) {
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
            let (depth, parent) = match (&cur, &prev) {
                (Some(dir), _) => (dir.depth, path_parent_filename(&dir.path)),
                (_, Some(prev_dir)) => (prev_dir.depth, path_parent_filename(&prev_dir.path)),
                _ => unreachable!("at least one directory must be set"),
            };
            state.navigate(depth, parent);
            match (cur, prev) {
                (Some(dir), Some(prev_dir)) => {
                    let mut d = Directory::new_from_existing(take(dir));
                    let files = take(&mut d.source_files);
                    d.set_files(
                        Self::merge_hydrate_sorted_files(files, &prev_dir.source_files),
                        prev_dir.manifest_info.manifest.clone(),
                    );
                    state.add_dir(d);
                    dir_iter.next();
                    prev_dir_iter.next();
                }
                (Some(dir), None) => {
                    let mut d = Directory::new_from_existing(take(dir));
                    let files = take(&mut d.source_files);
                    d.set_files(Self::merge_hydrate_sorted_files(files, &[]), None);
                    state.add_dir(d);
                    dir_iter.next();
                }
                (None, Some(prev_dir)) => {
                    let mut d = Directory::new_deleted(prev_dir.path.clone(), prev_dir.depth);
                    d.set_files(
                        Self::merge_hydrate_sorted_files(Vec::new(), &prev_dir.source_files),
                        prev_dir.manifest_info.manifest.clone(),
                    );
                    state.add_dir(d);
                    prev_dir_iter.next();
                }
                _ => unreachable!("at least one directory must be set"),
            }
        }
        state.take_tree()
    }

    fn merge_hydrate_sorted_files(files: Vec<File>, prev_files: &[File]) -> Vec<File> {
        let mut merged_files: Vec<File> = Vec::with_capacity(files.len());
        let mut file_iter = files.into_iter().peekable();
        let mut prev_file_iter = prev_files.iter().filter(|f| !f.deleted).peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching files.
            let (cur, prev) = match (file_iter.peek_mut(), prev_file_iter.peek()) {
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
                    let modified = match (&file.meta, &prev_file.meta) {
                        (Some(a), Some(b)) => compare_meta(a, b),
                        _ => true, // conservative
                    };
                    if modified && file.read_content() {
                        let diff = file.content != prev_file.content;
                        merged_files.push(File::new_from_existing(take(file), diff));
                    } else {
                        merged_files.push(File::new_from_existing((*prev_file).clone(), false));
                    }
                    file_iter.next();
                    prev_file_iter.next();
                }
                (Some(file), None) => {
                    if file.read_content() {
                        merged_files.push(File::new_from_existing(take(file), true));
                    }
                    file_iter.next();
                }
                (None, Some(prev_file)) => {
                    merged_files.push(File::new_deleted(prev_file.path.clone()));
                    prev_file_iter.next();
                }
                _ => unreachable!("at least one file must be set"),
            }
        }
        merged_files // sorted by file name
    }
}

#[cfg(unix)]
fn compare_meta(a: &std::fs::Metadata, b: &std::fs::Metadata) -> bool {
    use std::os::unix::prelude::MetadataExt as _;
    (a.ctime(), a.ctime_nsec(), a.ino(), a.permissions(), a.len())
        != (b.ctime(), b.ctime_nsec(), b.ino(), b.permissions(), b.len())
}

#[cfg(not(unix))]
fn compare_meta(a: &std::fs::Metadata, b: &std::fs::Metadata) -> bool {
    (a.modified().ok(), a.permissions(), a.len()) != (b.modified().ok(), b.permissions(), b.len())
}

fn path_parent_filename(p: &Path) -> Option<&str> {
    p.parent()?.file_name()?.to_str()
}
