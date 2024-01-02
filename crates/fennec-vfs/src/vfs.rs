// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::anyhow;
use debug_ignore::DebugIgnore;
use fennec_common::{types, util, workspace, MODULE_MANIFEST_FILENAME};
use line_index::{LineIndex, WideEncoding, WideLineCol};
use std::{
    cmp::Ordering,
    io::Read,
    mem::take,
    path::{Path, PathBuf},
    time::Duration,
    vec,
};

pub const DEFAULT_VFS_POLL_INTERVAL: Duration = Duration::from_millis(991);
const MAX_SOURCE_FILE_SIZE: u64 = (1 << 24) - 1; // 16 megabytes

#[derive(Default, Clone, Debug)]
struct File {
    path: PathBuf,
    meta: Metadata,
    content_changed: Option<bool>,
    content_fs: Option<types::Text>, // None initially or in case of read error (including deleted files)
    content_overlay: Option<(types::Text, i32)>,
}

#[derive(Default, Clone, Debug)]
struct Metadata {
    meta: Option<DebugIgnore<std::fs::Metadata>>,
}

impl Metadata {
    fn new(meta: Option<std::fs::Metadata>) -> Metadata {
        Metadata {
            meta: meta.map(std::convert::Into::into),
        }
    }

    #[cfg(unix)]
    fn unix_attrs(&self) -> (i64, i64, u64) {
        if let Some(meta) = &self.meta {
            use std::os::unix::prelude::MetadataExt as _;
            (meta.ctime(), meta.ctime_nsec(), meta.ino())
        } else {
            (0, 0, 0)
        }
    }

    #[cfg(not(unix))]
    fn unix_attrs(&self) -> (i64, i64, u64) {
        (0, 0, 0)
    }
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(a), Some(b)) = (&self.meta, &other.meta) {
            (
                self.unix_attrs(),
                a.modified().ok(),
                a.permissions(),
                a.len(),
            ) == (
                other.unix_attrs(),
                b.modified().ok(),
                b.permissions(),
                b.len(),
            )
        } else {
            false // conservative
        }
    }
}

impl Eq for Metadata {}

impl File {
    fn new_scratch(path: PathBuf, meta: Option<std::fs::Metadata>) -> File {
        File {
            path,
            meta: Metadata::new(meta),
            ..Default::default()
        }
    }

    fn new_merged(mut cur_file: File, prev_file: &File) -> File {
        // Current file is from the scan tree with no overlays.
        assert!(cur_file.meta.meta.is_some());
        assert!(cur_file.content_overlay.is_none());
        assert!(prev_file.content_changed.is_some());

        // If there was no overlay, the content from FS is guaranteed to be up-to-date.
        let (content_fs, meta, reuse_fs_content) = if prev_file.content_overlay.is_none() {
            if cur_file.meta == prev_file.meta {
                // The file did not change.
                (prev_file.content_fs.clone(), take(&mut cur_file.meta), true)
            } else if let Some((content_fs, meta)) = Self::read_content(&cur_file.path) {
                (Some(content_fs), Metadata::new(Some(meta)), false)
            } else {
                // Pretend that the file did not change.
                (prev_file.content_fs.clone(), take(&mut cur_file.meta), true)
            }
        } else {
            (None, Metadata::new(None), false)
        };

        let mut file = File {
            path: take(&mut cur_file.path),
            meta,
            content_changed: None,
            content_fs,
            content_overlay: prev_file.content_overlay.clone(),
        };
        file.content_changed = Some(
            prev_file.content_changed.unwrap_or_default() // overlay exists *and* has changed
                || (prev_file.content_overlay.is_none() && !reuse_fs_content && file.content() != prev_file.content()),
        );
        file
    }

    fn new_added(mut file: File) -> Option<File> {
        if let Some((content_fs, meta)) = Self::read_content(&file.path) {
            Some(File {
                path: take(&mut file.path),
                meta: Metadata::new(Some(meta)),
                content_changed: Some(true),
                content_fs: Some(content_fs),
                content_overlay: None,
            })
        } else {
            // File became inaccessible by the time we got to read it.
            None
        }
    }

    fn new_removed(prev_file: &File) -> File {
        assert!(prev_file.content_changed.is_some());
        // File does not exist on disk, so it is either completely removed or overlay-only.
        File {
            path: prev_file.path.clone(),
            meta: Metadata::new(None),
            content_changed: Some(
                prev_file.content_changed.unwrap_or_default() // overlay exists *and* has changed
                    || prev_file.content_overlay.is_none(),
            ),
            content_fs: None,
            content_overlay: prev_file.content_overlay.clone(),
        }
    }

    fn deleted(&self) -> bool {
        self.content().is_none()
    }

    fn file_name(&self) -> &str {
        self.path
            .file_name()
            .expect("file must have a valid file name")
            .to_str()
            .expect("file name must be valid UTF-8")
    }

    fn detached(&self) -> bool {
        !util::valid_source_file_name(self.file_name())
    }

    fn is_manifest(&self) -> bool {
        self.file_name() == MODULE_MANIFEST_FILENAME
    }

    fn content(&self) -> Option<&types::Text> {
        if let Some((text, _)) = &self.content_overlay {
            Some(text)
        } else {
            self.content_fs.as_ref()
        }
    }

    fn read_content(path: &Path) -> Option<(types::Text, std::fs::Metadata)> {
        let res = Self::read_utf8_lossy(path);
        match res {
            Ok(content) => Some(content),
            Err(err) => {
                let disp = path.display();
                log::warn!(r#"failed to read content of "{disp}", ignoring: {err}"#);
                None
            }
        }
    }

    fn read_utf8_lossy(path: &Path) -> Result<(types::Text, std::fs::Metadata), anyhow::Error> {
        let mut file = std::fs::File::open(path)?;
        let meta = file.metadata()?;
        let len = meta.len();
        if len > MAX_SOURCE_FILE_SIZE {
            let disp = path.display();
            return Err(anyhow!(
                r#"source file "{disp}" size of {len} exceeds maximum of {MAX_SOURCE_FILE_SIZE}"#
            ));
        }
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let s = String::from_utf8_lossy(&buf);
        Ok((types::Text::from(s), meta))
        // Vec -> Cow -> Arc is quite a lot of copies, but hopefully guaranteed UTF-8 simplifies things downstream.
    }
}

#[derive(Clone, Default, Debug)]
struct Directory {
    path: PathBuf,
    depth: isize,
    ghost: bool,                // virtual parent of an overlay
    subdirectories: Vec<usize>, // sorted by directory name
    source_files: Vec<File>,    // sorted by file name; manifest file is included
    content_changed: Option<bool>,
    module: Option<types::ImportPath>, // empty if semi-valid manifest does not currently exist
}

impl Directory {
    fn new(path: PathBuf, depth: isize) -> Directory {
        Directory {
            path,
            depth,
            ..Default::default()
        }
    }

    fn new_ghost(path: PathBuf, depth: isize) -> Directory {
        let mut dir = Directory::new(path, depth);
        dir.ghost = true;
        dir
    }

    fn new_from_existing(mut dir: Directory) -> Directory {
        dir.subdirectories.truncate(0);
        dir.content_changed = None;
        dir.module = None;
        dir
    }

    fn name(&self) -> &str {
        self.path.file_name().map_or("" /* root entry */, |s| {
            s.to_str().expect("directory name must be valid UTF-8")
        })
    }

    fn set_files(&mut self, files: Vec<File>, prev_module: Option<types::ImportPath>) {
        self.source_files = files;
        self.content_changed = Some(
            self.source_files
                .iter()
                .any(|f| f.content_changed.expect("change marker must be set")),
        );
        self.module = self.get_module(prev_module);
    }

    fn manifest_pos(&self) -> Option<usize> {
        self.source_files
            .binary_search_by_key(&MODULE_MANIFEST_FILENAME, |f| f.file_name())
            .ok()
    }

    fn changed_module_manifest(&self) -> Option<types::Text> {
        let ix = self
            .manifest_pos()
            .expect("directory must contain a manifest");
        let m = &self.source_files[ix];
        let m_changed = m.content_changed.expect("change marker must be set");
        if m_changed {
            let content = m
                .content()
                .cloned()
                .expect("manifest source must be set in the module directory");
            Some(content)
        } else {
            None
        }
    }

    fn get_module(&self, prev_module: Option<types::ImportPath>) -> Option<types::ImportPath> {
        let ix = self.manifest_pos();
        if let Some(ix) = ix {
            let m = &self.source_files[ix];
            if m.deleted() {
                return None;
            }
            let changed = m.content_changed.expect("change indicator must be set");
            if !changed {
                return prev_module;
            }
            let content = m
                .content()
                .expect("changed file must contain content")
                .as_ref();
            let res = fennec_module::extract(content);
            match res {
                Ok(module) => Some(module),
                Err(err) => {
                    let disp = m.path.display();
                    log::warn!(r#"failed to parse module manifest "{disp}": {err}"#);
                    // We might go back to pretending that the manifest has not changed instead,
                    // as this avoids the module removal. However, that breaks the invariant that
                    // the VFS state is equal to the fresh re-scan, so let's not do that.
                    None
                }
            }
        } else {
            None
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
    active_overlays: Vec<PathBuf>, // sorted
    overlay_updates: Vec<types::OverlayUpdate>,
}

impl ScanState {
    fn new(root: PathBuf) -> ScanState {
        let mut tree = vec![
            Directory::new(PathBuf::new(), -1),
            Directory::new(root.clone(), 0),
        ];
        tree[0].subdirectories = vec![1];

        ScanState {
            root,
            tree,
            modules: Vec::default(),
            active_overlays: Vec::default(),
            overlay_updates: Vec::default(),
        }
    }

    fn extract_overlays(&self) -> Vec<types::OverlayUpdate> {
        let mut updates = Vec::with_capacity(self.active_overlays.len());
        for dir in &self.tree {
            for file in &dir.source_files {
                if let Some((text, version)) = &file.content_overlay {
                    updates.push(types::OverlayUpdate::AddOverlay(
                        file.path.clone(),
                        text.clone(),
                        *version,
                    ));
                }
            }
        }
        updates
    }
}

#[derive(Default, Debug)]
struct ModTraverse {
    root: usize,
    packages: Vec<(usize, bool)>,
    depth: isize,
    cur_detached_pkgs_depth: Option<isize>,
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
    out_of_root_updates: Vec<types::OverlayUpdate>,
}

impl Vfs {
    #[must_use]
    pub fn new(cleanup_stale_roots: bool, poll_interval: Duration) -> Vfs {
        Vfs {
            poll_interval,
            cleanup_stale_roots,
            scan_state: Vec::default(),
            out_of_root_updates: Vec::default(),
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

            self.scan(&mut updates, should_scan, changes.overlay_updates);
            state.signal_core_module_updates(updates, changes.force_scan_id);
        }
    }

    pub fn __test_add_root(&mut self, root: PathBuf) -> Vec<workspace::ModuleUpdate> {
        let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
        self.add_root(root, &mut updates);
        updates
    }

    pub fn __test_scan(
        &mut self,
        should_scan: bool,
        overlay_updates: Vec<types::OverlayUpdate>,
    ) -> Vec<workspace::ModuleUpdate> {
        let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
        self.scan(&mut updates, should_scan, overlay_updates);
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
                let mut new_state = ScanState::new(root);
                while let Some(next_state) = self.scan_state.get(ix) {
                    if util::has_prefix(&next_state.root, &new_state.root) {
                        // Trying to add parent of an existing root; remove old one instead.
                        let state = self.scan_state.remove(ix);
                        for module in &state.modules {
                            let dir = module.index_root(&state.tree);
                            updates.push(workspace::ModuleUpdate {
                                source: dir.path.clone(),
                                module: dir.module.clone(),
                                manifest: None,
                                packages: Vec::new(),
                                update: workspace::UpdateKind::Removed,
                            });
                        }
                        // Transfer all overlays to pending updates to add them.
                        new_state.overlay_updates.extend(state.extract_overlays());
                    } else {
                        break;
                    }
                }
                self.scan_state.insert(ix, new_state);
                true
            }
        }
    }

    fn scan(
        &mut self,
        updates: &mut Vec<workspace::ModuleUpdate>,
        should_scan: bool,
        overlay_updates: Vec<types::OverlayUpdate>,
    ) {
        let to_apply = overlay_updates.len();
        log::trace!("scan start ({to_apply} new overlay updates, should_scan {should_scan})");
        self.prepare_overlay_updates(overlay_updates);
        for state in &mut self.scan_state {
            // Really expensive; would be great to find a cheaper alternative.
            let mut tree = state.tree.clone();
            for dir in &mut tree {
                dir.content_changed = Some(false);
                for file in &mut dir.source_files {
                    file.content_changed = Some(false);
                }
            }
            let to_apply = state.overlay_updates.len();
            log::trace!("tree (before {to_apply} updates) {tree:#?}");
            for update in take(&mut state.overlay_updates) {
                log::trace!("applying overlay update {update:?}");
                Self::apply_overlay_update(
                    &mut tree,
                    &mut state.active_overlays,
                    &state.root,
                    update,
                );
            }
            // Ensure we have no leftover directories consisting only of newly removed overlays.
            Self::clean_abandoned(&mut tree);
            log::trace!("tree (after {to_apply} updates and cleanup) {tree:#?}");
            if should_scan {
                let mut scan_tree = Self::scan_root(&state.root);
                log::trace!("scan tree {scan_tree:#?}");
                // Ensure all "ghost" directories exist in the scan tree before merging.
                for overlay_path in &state.active_overlays {
                    Self::tree_ensure_overlay_path(&mut scan_tree, overlay_path, &state.root, true);
                }
                tree = Self::merge_hydrate_sorted_preorder_dirs(scan_tree, &tree);
                log::trace!("merged scan tree {tree:#?}");
            }
            // Tree invariants, at this point:
            // - all real and "ghost" directories exist
            // - removed files are represented inside directories
            // - all file content and change markers are correct
            // - all directory change markers (non-recursive!) are correct and based on file ones
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
            let mut ix = 0;
            while ix < self.scan_state.len() {
                let any_manifest = self.scan_state[ix].tree.iter().any(|dir| {
                    if let Some(ix) = dir.manifest_pos() {
                        !dir.source_files[ix].deleted()
                    } else {
                        false
                    }
                });
                assert!(any_manifest || self.scan_state[ix].modules.is_empty());
                if any_manifest {
                    ix += 1;
                } else {
                    let s = self.scan_state.remove(ix);
                    self.out_of_root_updates.extend(s.extract_overlays());
                }
            }
        }
    }

    fn prepare_overlay_updates(&mut self, mut updates: Vec<types::OverlayUpdate>) {
        // Preserve the update order: newer updates go after older ones.
        let mut all_updates = take(&mut self.out_of_root_updates);
        all_updates.extend(take(&mut updates));
        // Add-[Change]-Remove chains can lead to us to mark phantom files/directories as changed,
        // which can cause us to emit bogus update events or bogus packages. Remove the chains.
        let mut by_path: types::HashMap<PathBuf, Vec<types::OverlayUpdate>> =
            types::HashMap::default();
        for update in all_updates {
            match &update {
                types::OverlayUpdate::AddOverlay(path, _, _) => {
                    by_path.insert(path.clone(), vec![update]);
                }
                types::OverlayUpdate::ChangeOverlay(path, _, _) => {
                    if let Some(v) = by_path.get_mut(path) {
                        v.push(update);
                    } else {
                        updates.push(update);
                    }
                }
                types::OverlayUpdate::RemoveOverlay(path) => {
                    if by_path.remove(path).is_none() {
                        updates.push(update);
                    }
                }
            }
        }
        for path_updates in by_path.into_values() {
            updates.extend(path_updates);
        }
        // Finally, group updates by scan roots.
        for update in updates {
            let path = update.path();
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if name != MODULE_MANIFEST_FILENAME && !util::valid_source_extension(name) {
                log::warn!("ignoring unrelated overlay update to {path:?}");
                continue;
            }
            let pos = self
                .scan_state
                .iter()
                .position(|s| util::has_strict_prefix(path, &s.root));
            if let Some(ix) = pos {
                log::trace!("will apply {update:?}");
                self.scan_state[ix].overlay_updates.push(update);
            } else {
                log::trace!("postpone (out of root) {update:?}");
                self.out_of_root_updates.push(update);
            }
        }
    }

    fn clean_abandoned(tree: &mut Vec<Directory>) {
        let mut ix = tree.len() - 1;
        while ix > 0 {
            let abandoned = tree[ix].ghost
                && tree[ix].subdirectories.is_empty()
                && tree[ix]
                    .source_files
                    .iter()
                    .all(|f| f.deleted() && f.content_changed.unwrap_or_default());
            if abandoned {
                tree.remove(ix);
                for dir in tree.iter_mut() {
                    dir.subdirectories.retain(|subdir| *subdir != ix);
                    for subdir in &mut dir.subdirectories {
                        if *subdir > ix {
                            *subdir -= 1;
                        }
                    }
                }
            }
            ix -= 1;
        }
    }

    fn apply_overlay_update(
        tree: &mut Vec<Directory>,
        active_overlays: &mut Vec<PathBuf>,
        root: &Path,
        update: types::OverlayUpdate,
    ) {
        let (file, dir_ix) = Self::tree_ensure_overlay_path(tree, update.path(), root, false)
            .expect("dir_only mode is false");

        let mut content_changed = false;
        match update {
            types::OverlayUpdate::AddOverlay(_, text, version) => 'add: {
                if let Some((_, cur_version)) = &file.content_overlay {
                    let path = file.path.display();
                    log::error!("attempting to add an overlay over existing one (new version {version}, old {cur_version}) for {path}");
                    if version <= *cur_version {
                        break 'add;
                    }
                }
                content_changed = Some(&text) != file.content_fs.as_ref();
                file.content_overlay = Some((text, version));
                if let Err(insert_ix) = active_overlays.binary_search(&file.path) {
                    active_overlays.insert(insert_ix, file.path.clone());
                }
            }
            types::OverlayUpdate::ChangeOverlay(_, changes, version) => {
                if let Some((cur_text, cur_version)) = &file.content_overlay {
                    if version <= *cur_version {
                        let path = file.path.display();
                        log::error!("attempting to change an overlay with wrong version (new version {version}, old {cur_version}) for {path}");
                    } else {
                        let text =
                            Self::apply_overlay_changes(String::from(cur_text.as_ref()), changes);
                        content_changed = text != cur_text.as_ref();
                        file.content_overlay = Some((types::Text::from(text), version));
                    }
                } else {
                    let path = file.path.display();
                    log::error!("attempting to change an overlay when none is existing for {path}");
                }
            }
            types::OverlayUpdate::RemoveOverlay(_) => {
                if file.content_overlay.is_none() {
                    let path = file.path.display();
                    log::error!("attempting to remove an overlay when none is existing for {path}");
                } else {
                    let prev_overlay = take(&mut file.content_overlay);
                    // We did not re-read the content while the overlay was active, so we must re-read it now.
                    if let Some((content_fs, meta)) = File::read_content(&file.path) {
                        file.content_fs = Some(content_fs);
                        file.meta = Metadata::new(Some(meta));
                    } else {
                        file.content_fs = None;
                        file.meta = Metadata::new(None);
                    }
                    content_changed = file.content_fs != prev_overlay.map(|ov| ov.0);
                    if let Ok(remove_ix) = active_overlays.binary_search(&file.path) {
                        active_overlays.remove(remove_ix);
                    }
                }
            }
        };

        let is_manifest = file.is_manifest();
        if !file.content_changed.unwrap_or_default() {
            file.content_changed = Some(content_changed);
        }
        if content_changed {
            tree[dir_ix].content_changed = Some(true);
        }
        if is_manifest {
            let prev_module = take(&mut tree[dir_ix].module);
            tree[dir_ix].module = tree[dir_ix].get_module(prev_module);
        }
    }

    fn apply_overlay_changes(mut text: String, changes: Vec<types::OverlayChange>) -> String {
        for change in changes {
            if let Some((mut start, mut end)) = change.range {
                // This is painfully inefficient and slow.
                let index = LineIndex::new(&text);
                if !change.utf8_pos {
                    let Some(utf8_start) = index.to_utf8(
                        WideEncoding::Utf16,
                        WideLineCol {
                            line: start.line,
                            col: start.col,
                        },
                    ) else {
                        log::error!(
                            "invalid utf-16 change range start: {start:?}; ignoring the change"
                        );
                        continue;
                    };
                    let Some(utf8_end) = index.to_utf8(
                        WideEncoding::Utf16,
                        WideLineCol {
                            line: end.line,
                            col: end.col,
                        },
                    ) else {
                        log::error!(
                            "invalid utf-16 change range end: {end:?}; ignoring the change"
                        );
                        continue;
                    };
                    start = utf8_start;
                    end = utf8_end;
                }
                let Some(start) = index.offset(start) else {
                    log::error!("invalid utf-8 change range start: {start:?}; ignoring the change");
                    continue;
                };
                let Some(end) = index.offset(end) else {
                    log::error!("invalid utf-8 change range end: {end:?}; ignoring the change");
                    continue;
                };
                let (start, end): (usize, usize) = (start.into(), end.into());
                text.replace_range(start..end, &change.content);
            } else {
                text = change.content;
            }
        }
        text
    }

    fn tree_ensure_overlay_path<'tree>(
        tree: &'tree mut Vec<Directory>,
        path: &Path,
        root: &Path,
        dir_only: bool,
    ) -> Option<(&'tree mut File, usize)> {
        assert_eq!(tree[0].depth, -1); // root-of-root

        let root_parent = root.parent().expect("root path must have a valid parent");
        let rel_path = path
            .strip_prefix(root_parent)
            .expect("update path must be a subdirectory of a scan root");

        log::trace!("searching for {rel_path:?} overlay node");
        let mut dir_components = rel_path
            .parent()
            .expect("file path must have a valid parent")
            .components()
            .peekable();

        // Navigate to the innermost subdirectory. First component must be the scan root.
        let mut cur_parent_ix = 0;
        loop {
            let Some(component) = dir_components.peek() else {
                log::trace!("end of components");
                break;
            };
            let res = tree[cur_parent_ix]
                .subdirectories
                .binary_search_by_key(&component.as_os_str(), |ix| tree[*ix].name().as_ref());
            if let Ok(ix) = res {
                let dir_ix = tree[cur_parent_ix].subdirectories[ix];
                log::trace!("moving current parent index to {dir_ix} for {component:?}");
                cur_parent_ix = dir_ix;
                dir_components.next();
            } else {
                log::trace!("subdirectory {component:?} no found");
                break;
            }
        }

        // Create all leftover subdirectories. We must preserve the preorder.
        for component in dir_components {
            let mut parent_path = tree[cur_parent_ix].path.as_path();
            if parent_path.as_os_str().is_empty() {
                // We are re-inserting the scan root; ensure absolute path.
                parent_path = root_parent;
            }
            let dir_path = parent_path.join(component);
            let dir = Directory::new_ghost(dir_path, tree[cur_parent_ix].depth + 1);

            let ix = tree[cur_parent_ix]
                .subdirectories
                .partition_point(|ix| tree[*ix].name() < dir.name());
            let dir_ix = if tree[cur_parent_ix].subdirectories.is_empty() {
                // Insert immediately after the current parent.
                cur_parent_ix + 1
            } else if ix == tree[cur_parent_ix].subdirectories.len() {
                // Insert immediately before the next sibling directory (same as after the last recursive subdirectory).
                let mut dir_ix = tree[cur_parent_ix]
                    .subdirectories
                    .last()
                    .expect("subdirectories are not empty")
                    + 1;
                while dir_ix < tree.len() && tree[dir_ix].path < dir.path {
                    dir_ix += 1;
                }
                dir_ix
            } else {
                // Insert before the subdirectory at `ix`.
                tree[cur_parent_ix].subdirectories[ix]
            };

            assert!(dir_ix > cur_parent_ix);
            // Fixup all subdir references caused by shifting the tree.
            for dir in tree.iter_mut() {
                for subdir_ix in &mut dir.subdirectories {
                    if *subdir_ix >= dir_ix {
                        *subdir_ix += 1;
                    }
                }
            }
            tree.insert(dir_ix, dir);
            tree[cur_parent_ix].subdirectories.insert(ix, dir_ix);
            cur_parent_ix = dir_ix;
            log::trace!("moving current parent index to {dir_ix} after inserting {component:?}");
        }

        if dir_only {
            return None;
        }

        // Finally, grab or create the file.
        let name = rel_path
            .file_name()
            .expect("file path must have a valid file name");
        let res = tree[cur_parent_ix]
            .source_files
            .binary_search_by_key(&name, |f| f.file_name().as_ref());
        match res {
            Ok(ix) => Some((&mut tree[cur_parent_ix].source_files[ix], cur_parent_ix)),
            Err(ix) => {
                let file_path = tree[cur_parent_ix].path.join(name);
                let file = File::new_scratch(file_path, None);
                tree[cur_parent_ix].source_files.insert(ix, file);
                Some((&mut tree[cur_parent_ix].source_files[ix], cur_parent_ix))
            }
        }
    }

    fn build_modules(tree: &[Directory]) -> Vec<ModTraverse> {
        let mut modules: Vec<ModTraverse> = Vec::new();
        let mut stack: Vec<ModTraverse> = Vec::new();
        for (ix, dir) in tree.iter().enumerate() {
            // Pop finished modules off the stack until the depth is increasing
            // (equal depth means we are processing a potential new module at the same depth, so old one is done).
            while !stack.is_empty() && dir.depth <= stack.last().expect("stack is not empty").depth
            {
                modules.push(stack.pop().expect("stack is not empty"));
            }
            if let Some(m_ix) = dir.manifest_pos() {
                if !dir.source_files[m_ix].deleted() {
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
            // or the directory name is a valid package name (for non-detached packages).
            let detached = if root_pkg {
                false
            } else {
                // Maybe we have finished a subtree with an invalid path element?
                let detached_finished = match m.cur_detached_pkgs_depth {
                    Some(depth) => dir.depth <= depth,
                    None => true,
                };
                if detached_finished {
                    m.cur_detached_pkgs_depth = None;
                    // Maybe we have to start a new ignore subtree?
                    let valid = util::valid_package_name(dir.name());
                    if valid {
                        false
                    } else {
                        m.cur_detached_pkgs_depth = Some(dir.depth);
                        true
                    }
                } else {
                    assert!(m.cur_detached_pkgs_depth.is_some());
                    true
                }
            };
            m.packages.push((ix, detached));
        }
        modules.extend(stack.into_iter().rev());
        // Sort modules so that we can later merge them.
        modules.sort_unstable_by(|a, b| {
            let a_dir = a.index_root(tree);
            let b_dir = b.index_root(tree);
            (&a_dir.module, &a_dir.path).cmp(&(&b_dir.module, &b_dir.path))
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
                    match (&mod_dir.module, &mod_dir.path)
                        .cmp(&(&prev_mod_dir.module, &prev_mod_dir.path))
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
                        dir.module.as_ref(),
                        &dir.path,
                        &module.packages,
                        tree,
                        &prev_module.packages,
                        prev_tree,
                    );
                    let manifest = dir.changed_module_manifest();
                    if manifest.is_some() || !packages.is_empty() {
                        updates.push(workspace::ModuleUpdate {
                            source: dir.path.clone(),
                            module: dir.module.clone(),
                            manifest,
                            packages, // may be empty
                            update: workspace::UpdateKind::Updated,
                        });
                    }
                    mod_iter.next();
                    prev_mod_iter.next();
                }
                (Some(module), None) => {
                    let dir = module.index_root(tree);
                    let packages = Self::build_package_updates(
                        dir.module.as_ref(),
                        &dir.path,
                        &module.packages,
                        tree,
                        &[],
                        &[],
                    );
                    updates.push(workspace::ModuleUpdate {
                        source: dir.path.clone(),
                        module: dir.module.clone(),
                        manifest: dir.changed_module_manifest(),
                        packages,
                        update: workspace::UpdateKind::Added,
                    });
                    mod_iter.next();
                }
                (None, Some(prev_module)) => {
                    let dir = prev_module.index_root(prev_tree);
                    updates.push(workspace::ModuleUpdate {
                        source: dir.path.clone(),
                        module: dir.module.clone(),
                        manifest: None,
                        packages: Vec::new(),
                        update: workspace::UpdateKind::Removed,
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
        packages: &[(usize, bool)],
        tree: &[Directory],
        prev_packages: &[(usize, bool)],
        prev_tree: &[Directory],
    ) -> Vec<workspace::PackageUpdate> {
        let mut updates: Vec<workspace::PackageUpdate> = Vec::new();
        let mut package_dir_iter = packages
            .iter()
            .map(|(ix, detached)| (&tree[*ix], *detached))
            .peekable();
        let mut prev_package_dir_iter = prev_packages
            .iter()
            .map(|(ix, detached)| (&prev_tree[*ix], *detached))
            .peekable();
        loop {
            // Loop invariant: (cur, prev) point to a pair of matching package directories.
            let (cur, prev) = match (package_dir_iter.peek(), prev_package_dir_iter.peek()) {
                (None, None) => break,
                (Some(cur), Some(prev)) => match cur.0.path.cmp(&prev.0.path) {
                    Ordering::Equal => (Some(cur), Some(prev)),
                    Ordering::Less => (Some(cur), None),
                    Ordering::Greater => (None, Some(prev)),
                },
                only_one => only_one,
            };
            match (cur, prev) {
                (Some(&(dir, detached)), Some(_)) => {
                    let files: Vec<workspace::FileUpdate> = dir
                        .source_files
                        .iter()
                        .filter(|f| {
                            f.content_changed.expect("change marker must be set")
                                && !f.is_manifest()
                        })
                        .map(|f| workspace::FileUpdate {
                            source: f.path.clone(),
                            content: f.content().cloned(),
                            detached: f.detached(),
                        })
                        .collect();
                    if !files.is_empty() {
                        updates.push(workspace::PackageUpdate {
                            source: dir.path.clone(),
                            path: Self::pkg_import_path(detached, mod_path, mod_dir, &dir.path),
                            files,
                            update: workspace::UpdateKind::Updated,
                        });
                    }
                    package_dir_iter.next();
                    prev_package_dir_iter.next();
                }
                (Some(&(dir, detached)), None) => {
                    let files =
                        dir.source_files
                            .iter()
                            .filter(|f| !f.deleted() && !f.is_manifest())
                            .map(|f| {
                                workspace::FileUpdate {
                                source: f.path.clone(),
                                content: Some(f.content().expect(
                                    "content must be set on all files in a package directory",
                                ).clone()),
                                detached: f.detached(),
                            }
                            });
                    updates.push(workspace::PackageUpdate {
                        source: dir.path.clone(),
                        path: Self::pkg_import_path(detached, mod_path, mod_dir, &dir.path),
                        files: files.collect(), // may be empty
                        update: workspace::UpdateKind::Added,
                    });
                    package_dir_iter.next();
                }
                (None, Some(&(prev_dir, detached))) => {
                    updates.push(workspace::PackageUpdate {
                        source: prev_dir.path.clone(),
                        path: Self::pkg_import_path(detached, mod_path, mod_dir, &prev_dir.path),
                        files: Vec::new(),
                        update: workspace::UpdateKind::Removed,
                    });
                    prev_package_dir_iter.next();
                }
                _ => unreachable!("at least one package must be set"),
            };
        }
        updates
    }

    fn pkg_import_path(
        detached: bool,
        path: Option<&types::ImportPath>,
        mod_dir: &Path,
        pkg_dir: &Path,
    ) -> Option<types::ImportPath> {
        if detached {
            return None;
        }
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
                        if name == MODULE_MANIFEST_FILENAME || util::valid_source_extension(name) {
                            let meta = entry.metadata().ok();
                            if meta.is_some() {
                                let file = File::new_scratch(entry.into_path(), meta);
                                state.add_file(file);
                            }
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
        let mut prev_dir_iter = prev_dirs.iter().skip(1).peekable();
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
            match (cur, prev) {
                (Some(dir), prev) => {
                    state.navigate(dir.depth, path_parent_filename(&dir.path));
                    let mut d = Directory::new_from_existing(take(dir));
                    let files = take(&mut d.source_files);
                    let prev_files = prev.map_or(&[] as &[File], |d| &d.source_files);
                    let prev_module = prev.and_then(|d| d.module.clone());
                    d.set_files(
                        Self::merge_hydrate_sorted_files(files, prev_files),
                        prev_module,
                    );
                    state.add_dir(d);
                    dir_iter.next();
                    if prev.is_some() {
                        prev_dir_iter.next();
                    }
                }
                (None, Some(_)) => {
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
        let mut prev_file_iter = prev_files
            .iter()
            .filter(|f| !f.deleted() || f.content_changed.unwrap_or_default())
            .peekable();
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
                    let merged = File::new_merged(take(file), prev_file);
                    merged_files.push(merged);
                    file_iter.next();
                    prev_file_iter.next();
                }
                (Some(file), None) => {
                    let merged = File::new_added(take(file));
                    merged_files.extend(merged);
                    file_iter.next();
                }
                (None, Some(prev_file)) => {
                    let merged = File::new_removed(prev_file);
                    merged_files.push(merged);
                    prev_file_iter.next();
                }
                _ => unreachable!("at least one file must be set"),
            }
        }
        merged_files // sorted by file name
    }
}

fn path_parent_filename(p: &Path) -> Option<&str> {
    p.parent()?.file_name()?.to_str()
}
