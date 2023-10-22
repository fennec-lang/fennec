// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::{types, util, workspace, MODULE_MANIFEST_FILENAME};
use once_cell::sync::Lazy;
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
static EMPTY_CONTENT: Lazy<Arc<str>> = Lazy::new(|| Arc::from(""));

#[derive(Default, Clone)]
struct File {
    path: PathBuf,
    modified: Option<SystemTime>,
    deleted: bool,
    content_changed: Option<bool>,
    content: Option<Arc<str>>, // None initially or in case of read error, and also for deleted files
}

impl File {
    fn new(path: PathBuf, modified: Option<SystemTime>) -> File {
        File {
            path,
            modified,
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

#[derive(Clone)]
struct ManifestInfo {
    manifest: workspace::ModuleManifest,
    manifest_changed: bool,
}

#[derive(Clone, Default)]
struct Directory {
    path: PathBuf,
    depth: usize,
    deleted: bool,
    subdirectories: Vec<usize>, // sorted by directory name
    source_files: Vec<File>,    // sorted by file name; manifest file is included
    content_changed: Option<bool>,
    manifest_info: Option<ManifestInfo>, // empty if manifest does not currently exist
}

impl Directory {
    fn new(path: PathBuf, depth: usize) -> Directory {
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

    fn new_deleted(path: PathBuf, depth: usize) -> Directory {
        Directory {
            path,
            depth,
            deleted: true,
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

    fn set_files(&mut self, files: Vec<File>) {
        self.source_files = files;
        self.content_changed = Some(
            self.source_files
                .iter()
                .any(|f| f.content_changed.expect("change marker must be set")),
        );
        let prev_info = take(&mut self.manifest_info);
        self.manifest_info = self.get_manifest(prev_info);
    }

    fn get_manifest(&self, prev_info: Option<ManifestInfo>) -> Option<ManifestInfo> {
        let ins = self
            .source_files
            .binary_search_by_key(&MODULE_MANIFEST_FILENAME, |f| f.file_name());
        match ins {
            Ok(ix) => {
                let m = &self.source_files[ix];
                if m.deleted {
                    return None;
                }
                let changed = m.content_changed.expect("change indicator must be set");
                let prev_manifest = prev_info
                    .expect("previous manifest info must exist")
                    .manifest;
                if !changed {
                    return Some(ManifestInfo {
                        manifest: prev_manifest,
                        manifest_changed: false,
                    });
                }
                let content = m
                    .content
                    .as_ref()
                    .expect("changed file must contain content")
                    .as_ref();
                let res = manifest::parse(content);
                match res {
                    Ok(manifest) => Some(ManifestInfo {
                        manifest,
                        manifest_changed: true,
                    }),
                    Err(err) => {
                        let disp = m.path.display();
                        log::warn!(r#"failed to parse module manifest "{disp}", ignoring: {err}"#);
                        Some(ManifestInfo {
                            manifest: prev_manifest,
                            manifest_changed: false,
                        })
                    }
                }
            }
            Err(_) => None,
        }
    }
}

struct DirTreeBuildState {
    tree: Vec<Directory>,
    cur_depth: usize,
    cur_dir_ix: usize,
    parents: Vec<Option<usize>>,   // parent index of i'th directory
    subdirectories_at: Vec<usize>, // next index to visit in subdirectories of i'th directory
}

impl DirTreeBuildState {
    // We could play tricks with reuse of the state between scans/traversals,
    // but let's avoid premature optimization for now.
    fn new() -> DirTreeBuildState {
        DirTreeBuildState {
            tree: vec![Directory::new(PathBuf::new(), 0)], // root entry
            cur_depth: 0,
            cur_dir_ix: 0,
            parents: vec![None],
            subdirectories_at: vec![0],
        }
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

#[derive(Default)]
struct ModTraverse {
    root: usize,
    packages: Vec<usize>,
    depth: usize,
    cur_ignore_pkgs_depth: Option<usize>,
}

impl ModTraverse {
    fn new(root: usize, depth: usize) -> ModTraverse {
        ModTraverse {
            root,
            depth,
            ..Default::default()
        }
    }

    fn index_root<'a>(&self, tree: &'a [Directory]) -> (&'a ManifestInfo, &'a Directory) {
        let mod_dir = &tree[self.root];
        let info = mod_dir
            .manifest_info
            .as_ref()
            .expect("manifest info must exist in module root");
        (info, mod_dir)
    }
}

pub struct Vfs {
    poll_interval: Duration,
    cleanup_stale_roots: bool,
    scan_state: Vec<ScanState>, // sorted; no element is a prefix of another element
}

impl Vfs {
    #[must_use]
    pub fn new(cleanup_stale_roots: bool) -> Vfs {
        Vfs {
            poll_interval: DEFAULT_POLL_INTERVAL,
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

            let mut should_scan = changes.force_scan || timed_out;
            for root in changes.scan_roots {
                // We don't want to re-scan if we got notified about a root we are already watching.
                should_scan |= self.add_root(root);
            }

            if should_scan {
                let updates = self.scan();
                if !updates.is_empty() {
                    state.signal_core_module_updates(updates);
                }
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
        let mut all_updates: Vec<workspace::ModuleUpdate> = Vec::new();
        for state in &mut self.scan_state {
            let scan_tree = Self::scan_root(&state.root);
            let tree = Self::merge_hydrate_sorted_preorder_dirs(scan_tree, &state.tree);
            let modules = Self::build_modules(&tree);
            let updates = Self::build_module_updates(&tree, &modules, &state.tree, &state.modules);
            all_updates.extend(updates);
            state.tree = tree;
            state.modules = modules;
        }
        if self.cleanup_stale_roots {
            // Since we only clean up after manifest all manifests are in the None state,
            // by that time we have already reported that the modules are deleted;
            // no need to special-case scan root removal in module diff calculation.
            self.scan_state.retain_mut(|s| {
                let any_manifest = s.tree.iter().any(|dir| dir.manifest_info.is_some());
                assert!(any_manifest || s.modules.is_empty());
                any_manifest
            });
        }
        all_updates
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
            // Push new module on the stack, if we are visiting an existing module root.
            if dir.manifest_info.is_some() {
                stack.push(ModTraverse::new(ix, dir.depth));
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
                if !ignore_finished {
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
            let (a_info, a_dir) = a.index_root(tree);
            let (b_info, b_dir) = b.index_root(tree);
            (&a_info.manifest.module, &a_dir.path).cmp(&(&b_info.manifest.module, &b_dir.path))
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
                    let (info, mod_dir) = module.index_root(tree);
                    let (prev_info, prev_mod_dir) = prev_module.index_root(prev_tree);
                    match (&info.manifest.module, &mod_dir.path)
                        .cmp(&(&prev_info.manifest.module, &prev_mod_dir.path))
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
                    let (info, dir) = module.index_root(tree);
                    let packages = Self::build_package_updates(
                        &info.manifest.module,
                        &dir.path,
                        &module.packages,
                        tree,
                        &prev_module.packages,
                        prev_tree,
                    );
                    if info.manifest_changed || !packages.is_empty() {
                        updates.push(workspace::ModuleUpdate {
                            source: dir.path.clone(),
                            module: info.manifest.module.clone(),
                            manifest: info.manifest_changed.then(|| info.manifest.clone()),
                            packages, // may be empty
                            update: workspace::ModuleUpdateKind::ModuleUpdated,
                        });
                    }
                    mod_iter.next();
                    prev_mod_iter.next();
                }
                (Some(module), None) => {
                    let (info, dir) = module.index_root(tree);
                    let packages = Self::build_package_updates(
                        &info.manifest.module,
                        &dir.path,
                        &module.packages,
                        tree,
                        &[],
                        &[],
                    );
                    updates.push(workspace::ModuleUpdate {
                        source: dir.path.clone(),
                        module: info.manifest.module.clone(),
                        manifest: Some(info.manifest.clone()),
                        packages,
                        update: workspace::ModuleUpdateKind::ModuleAdded,
                    });
                    mod_iter.next();
                }
                (None, Some(prev_module)) => {
                    let (info, dir) = prev_module.index_root(prev_tree);
                    updates.push(workspace::ModuleUpdate {
                        source: dir.path.clone(),
                        module: info.manifest.module.clone(),
                        manifest: None,
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
        mod_path: &types::ImportPath,
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
                    let files: Vec<workspace::File> = dir
                        .source_files
                        .iter()
                        .filter(|f| {
                            f.content_changed.expect("change marker must be set")
                                && !f.is_manifest()
                        })
                        .map(|f| workspace::File {
                            source: f.path.clone(),
                            content: f.content.as_ref().unwrap_or(&EMPTY_CONTENT).clone(),
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
                        files: files.collect(), // may be empty
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
        path: &types::ImportPath,
        mod_dir: &Path,
        pkg_dir: &Path,
    ) -> types::ImportPath {
        let rel = pkg_dir
            .strip_prefix(mod_dir)
            .expect("package must be a module subdirectory")
            .to_str()
            .expect("package relative path must be valid UTF-8");
        path.join(rel)
            .expect("join of package relative path to module import path must succeed")
    }

    fn scan_root(root: &PathBuf) -> Vec<Directory> {
        let mut state = DirTreeBuildState::new();
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
        prev_dirs: &[Directory],
    ) -> Vec<Directory> {
        let mut state = DirTreeBuildState::new();
        let mut dir_iter = dirs.into_iter().peekable();
        let mut prev_dir_iter = prev_dirs.iter().filter(|d| !d.deleted).peekable();
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
                    let mut d = Directory::new_from_existing(take(dir));
                    let files = take(&mut d.source_files);
                    d.set_files(Self::merge_hydrate_sorted_files(
                        files,
                        &prev_dir.source_files,
                    ));
                    state.add_dir(d);
                    dir_iter.next();
                    prev_dir_iter.next();
                }
                (Some(dir), None) => {
                    let mut d = Directory::new_from_existing(take(dir));
                    let files = take(&mut d.source_files);
                    d.set_files(Self::merge_hydrate_sorted_files(files, &[]));
                    state.add_dir(d);
                    dir_iter.next();
                }
                (None, Some(prev_dir)) => {
                    let mut d = Directory::new_deleted(prev_dir.path.clone(), prev_dir.depth);
                    d.set_files(Self::merge_hydrate_sorted_files(
                        Vec::new(),
                        &prev_dir.source_files,
                    ));
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
                    let modified = match (file.modified, prev_file.modified) {
                        (Some(t), Some(prev_t)) => t != prev_t,
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
