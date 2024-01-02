extern crate proptest;
extern crate proptest_state_machine;

use std::{
    cmp::Ordering,
    fs,
    io::Read,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use fennec_common::{types, util, workspace, MODULE_MANIFEST_FILENAME};
use fennec_vfs::Vfs;
use proptest::{prelude::*, sample, strategy::Union};
use proptest_state_machine::{ReferenceStateMachine, StateMachineTest};
use similar_asserts::assert_eq;
use slotmap::SlotMap;

fn vfs_config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: None, // unfortunately overriden inside the proptest! macro, leading to warnings like https://github.com/proptest-rs/proptest/issues/233
        ..ProptestConfig::default()
    }
}

// Ensure that the VFS correctly reconstructs the workspace state.
#[test]
fn vfs_workspace_reconstruct() {
    env_logger::builder()
        .is_test(true)
        .try_init()
        .expect("logging must be initialized successfully");
    let cfg = vfs_config();
    proptest!(cfg.clone(), |((initial_state, transitions) in <VfsMachine as StateMachineTest>::Reference::sequential_strategy(1..30))| {
        VfsMachine::test_sequential(cfg.clone(), initial_state, transitions)
    });
}

#[derive(Copy, Clone, Debug)]
enum NodeNameKind {
    SourceCode,
    ModuleManifest,
    Other,
}

#[derive(Clone)]
enum Transition {
    AddNode {
        name: String,
        directory: bool,
        overlay: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
        parent: Option<NodeKey>,
    },
    SetupOverlayNode {
        key: NodeKey, // not a directory
    },
    RemoveNode {
        key: NodeKey, // can be regular or overlay node
    },
    UpdateNode {
        key: NodeKey, // can be regular or overlay node
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    },
    SaveOverlayNode {
        key: NodeKey,
    },
    MarkScanRoot {
        key: NodeKey, // directory
    },
    Scan {},
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transition::AddNode {
                name,
                directory,
                overlay,
                raw_content,
                manifest,
                parent,
            } => {
                if *directory {
                    write!(f, "ADD DIR {name}, parent {parent:?}")
                } else if name == MODULE_MANIFEST_FILENAME {
                    let valid_manifest = manifest.is_some();
                    write!(
                        f,
                        "Add MANIFEST {name}, parent {parent:?}, valid manifest {valid_manifest}, overlay {overlay}"
                    )
                } else {
                    let n = raw_content.len();
                    let pretty_content = String::from_utf8_lossy(&raw_content);
                    write!(
                        f,
                        "Add FILE {name}, parent {parent:?}, overlay {overlay} content ({n}) {pretty_content}"
                    )
                }
            }
            Transition::SetupOverlayNode { key } => {
                write!(f, "SETUP OVERLAY {key:?}")
            }
            Transition::RemoveNode { key } => {
                write!(f, "REMOVE {key:?}")
            }
            Transition::UpdateNode {
                key,
                raw_content,
                manifest,
            } => {
                let valid_manifest = manifest.is_some();
                let n = raw_content.len();
                let pretty_content = String::from_utf8_lossy(&raw_content);
                write!(
                    f,
                    "UPDATE {key:?}, valid manifest {valid_manifest}, content ({n}) {pretty_content}"
                )
            }
            Transition::SaveOverlayNode { key } => {
                write!(f, "SAVE OVERLAY {key:?}")
            }
            Transition::MarkScanRoot { key } => {
                write!(f, "MARK SCAN ROOT {key:?}")
            }
            Transition::Scan {} => {
                write!(f, "SCAN")
            }
        }
    }
}

fn node_name_strategy(name_kind: NodeNameKind, valid_ident: bool) -> BoxedStrategy<String> {
    let expr = match name_kind {
        NodeNameKind::ModuleManifest => "fennec\\.mod",
        NodeNameKind::SourceCode => {
            if valid_ident {
                "[a-z][a-z0-9_]*\\.fn"
            } else {
                "[A-Z][A-Z0-9_]*\\.fn"
            }
        }
        NodeNameKind::Other => {
            if valid_ident {
                "[a-z][a-z0-9_]*"
            } else {
                "[A-Z][A-Z0-9_]*"
            }
        }
    };
    expr.prop_filter("filename must not be reserved on windows", |name| {
        !util::is_reserved_windows_filename(name)
    })
    .boxed()
}

fn import_path_strategy() -> BoxedStrategy<types::ImportPath> {
    let path_strategy = Union::new(vec![
        "[a-z]+\\/[a-z][a-z0-9_]*",
        "[a-z]+\\/[a-z][a-z0-9_]*\\/[a-z][a-z0-9_]*",
        "[a-z]+\\/[a-z][a-z0-9_]*\\/[a-z][a-z0-9_]*\\/[a-z][a-z0-9_]*",
    ]);
    path_strategy
        .prop_filter_map("import path must be valid", |path| {
            types::ImportPath::parse(&path).ok()
        })
        .boxed()
}

fn raw_content_strategy() -> BoxedStrategy<Vec<u8>> {
    // Proptest is having much easier time with minimization when limiting content to 1 byte.
    any::<Option<u8>>()
        .prop_map(|v| v.into_iter().collect::<Vec<u8>>())
        .boxed()
}

fn manifest_strategy() -> BoxedStrategy<Option<types::ImportPath>> {
    Union::new(vec![
        Just(None).boxed(),
        import_path_strategy().prop_map(|path| Some(path)).boxed(),
    ])
    .boxed()
}

fn add_node_strategy(gen_overlay: bool, parents: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let name_kind_strategy = Union::new(vec![
        Just(NodeNameKind::Other),
        Just(NodeNameKind::SourceCode),
        Just(NodeNameKind::ModuleManifest),
    ]);
    let valid_ident_strategy = any::<bool>();
    let name_strategy = (name_kind_strategy, valid_ident_strategy)
        .prop_flat_map(|(name_kind, valid_ident)| node_name_strategy(name_kind, valid_ident));
    let directory_strategy = any::<bool>();
    let directory_overlay_strategy = directory_strategy.prop_flat_map(move |directory| {
        if directory || !gen_overlay {
            Just((directory, false)).boxed()
        } else {
            any::<bool>()
                .prop_map(move |overlay| (directory, overlay))
                .boxed()
        }
    });
    let want_parent_strategy = any::<bool>();
    let maybe_parent_strategy = want_parent_strategy
        .prop_flat_map(move |want_parent| {
            if want_parent && !parents.is_empty() {
                sample::select((&parents).clone()) // no idea why it can't be just `parents`
                    .prop_map(Option::Some)
                    .boxed()
            } else {
                Just(Option::<NodeKey>::None).boxed()
            }
        })
        .boxed();

    (
        name_strategy,
        directory_overlay_strategy,
        raw_content_strategy(),
        manifest_strategy(),
        maybe_parent_strategy,
    )
        .prop_map(
            |(name, (directory, overlay), raw_content, manifest, parent)| Transition::AddNode {
                name,
                directory,
                overlay,
                raw_content,
                manifest,
                parent,
            },
        )
        .boxed()
}

fn setup_overlay_node_strategy(regular_files: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let key_strategy = sample::select(regular_files);
    key_strategy
        .prop_map(|key| Transition::SetupOverlayNode { key })
        .boxed()
}

fn remove_node_strategy(nodes: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let key_strategy = sample::select(nodes);
    key_strategy
        .prop_map(|key| Transition::RemoveNode { key })
        .boxed()
}

fn update_node_strategy(nodes: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let key_strategy = sample::select(nodes);
    (key_strategy, raw_content_strategy(), manifest_strategy())
        .prop_map(|(key, raw_content, manifest)| Transition::UpdateNode {
            key,
            raw_content,
            manifest,
        })
        .boxed()
}

fn save_overlay_node_strategy(overlay_files: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let key_strategy = sample::select(overlay_files);
    key_strategy
        .prop_map(|key| Transition::SaveOverlayNode { key })
        .boxed()
}

fn mark_scan_root_strategy(directories: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let dir_strategy = sample::select(directories);
    dir_strategy
        .prop_map(|key| Transition::MarkScanRoot { key })
        .boxed()
}

fn scan_strategy() -> BoxedStrategy<Transition> {
    Just(Transition::Scan {}).boxed()
}

slotmap::new_key_type! { struct NodeKey; }

#[derive(Clone, Debug)]
struct Node {
    name: String,
    directory: bool,
    removed_directory: bool,
    overlay: bool,
    scan_root: bool,
    raw_content: Vec<u8>,
    manifest: Option<types::ImportPath>,
    parent: Option<NodeKey>,
    children: Vec<NodeKey>,
}

impl Node {
    fn new(
        name: String,
        directory: bool,
        overlay: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
        parent: Option<NodeKey>,
    ) -> Node {
        assert!(!overlay || !directory); // overlay => !directory
        Node {
            name,
            directory,
            removed_directory: false,
            overlay,
            scan_root: false,
            raw_content,
            manifest,
            parent,
            children: Vec::new(),
        }
    }

    fn find_child(
        &self,
        name: &str,
        overlay: bool,
        nodes: &SlotMap<NodeKey, Node>,
    ) -> Result<usize, usize> {
        find_sorted_by_name_overlay(&self.children, name, overlay, nodes)
    }
}

fn format_manifest(path: Option<types::ImportPath>) -> String {
    if let Some(path) = path {
        let mut buf = Vec::new();
        fennec_module::write_with_current_version(&mut buf, &path).unwrap();
        String::from_utf8(buf).unwrap()
    } else {
        String::new()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct File {
    source: PathBuf,
    content: types::Text,
    detached: bool, // invalid file name
}

impl Default for File {
    fn default() -> Self {
        File {
            source: PathBuf::default(),
            content: types::EMPTY_TEXT.clone(),
            detached: false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Package {
    source: PathBuf,
    path: Option<types::ImportPath>, // empty in case of detached package
    files: Vec<File>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Module {
    source: PathBuf,
    path: Option<types::ImportPath>, // empty in case of detached module
    manifest: types::Text,
    packages: Vec<Package>,
}

#[derive(Clone, Debug, Default)]
struct VfsReferenceMachine {
    nodes: SlotMap<NodeKey, Node>,
    toplevel: Vec<NodeKey>,
    files: Vec<NodeKey>,
    regular_files: Vec<NodeKey>,
    overlay_files: Vec<NodeKey>,
    directories: Vec<NodeKey>,
    scan_roots: types::HashSet<PathBuf>,
    last_scan: Vec<Module>,
    // Kludges to simplify name resolution in real state machine.
    last_added_node_parent: Option<PathBuf>,
    last_removed_node_path: Option<PathBuf>,
    last_removed_node_is_overlay: Option<bool>,
    // Hack to vary initialization of the SUT.
    rng_seed: u64,
    async_vfs: bool,
    cleanup_stale_roots: bool,
}

fn find_sorted_by_name_overlay(
    v: &Vec<NodeKey>,
    name: &str,
    overlay: bool,
    nodes: &SlotMap<NodeKey, Node>,
) -> Result<usize, usize> {
    v.binary_search_by_key(&(name, overlay), |k| (&nodes[*k].name, nodes[*k].overlay))
}

fn sorted_vec_contains<T: Ord>(v: &Vec<T>, elem: &T) -> bool {
    v.binary_search(elem).is_ok()
}

fn sorted_vec_insert<T: Ord>(v: &mut Vec<T>, elem: T) {
    let ix = v.binary_search(&elem).unwrap_err();
    v.insert(ix, elem);
}

fn sorted_vec_remove<T: Ord>(v: &mut Vec<T>, elem: &T) {
    let ix = v.binary_search(elem).unwrap();
    v.remove(ix);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct ModuleLoc {
    source: PathBuf,
    module: Option<types::ImportPath>,
}

impl VfsReferenceMachine {
    fn new(rng_seed: u64, async_vfs: bool, mut cleanup_stale_roots: bool) -> VfsReferenceMachine {
        if async_vfs {
            // When using async VFS, marking of scan roots can trigger an "extra" scan,
            // which is absent in the reference machine. This scan can lead to removal
            // of the scan root that reference machine still considers valid.
            cleanup_stale_roots = false;
        }
        VfsReferenceMachine {
            rng_seed,
            async_vfs,
            cleanup_stale_roots,
            ..Default::default()
        }
    }

    fn on_add_node(
        &mut self,
        name: String,
        directory: bool,
        overlay: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
        parent: Option<NodeKey>,
    ) {
        self.last_added_node_parent = parent.map(|key| self.node_path(key));
        let mut key = self.nodes.insert(Node::new(
            name.clone(),
            directory,
            overlay,
            raw_content,
            manifest,
            parent,
        ));
        let mut inserted = true;
        if let Some(parent_key) = parent {
            let child_pos = find_sorted_by_name_overlay(
                &self.nodes[parent_key].children,
                &name,
                overlay,
                &self.nodes,
            );
            match child_pos {
                Ok(child_ix) => {
                    self.nodes.remove(key);
                    inserted = false;
                    key = (&mut self.nodes[parent_key]).children[child_ix];
                }
                Err(child_ix) => {
                    (&mut self.nodes[parent_key]).children.insert(child_ix, key);
                }
            };
        } else {
            let child_pos =
                find_sorted_by_name_overlay(&self.toplevel, &name, overlay, &self.nodes);
            match child_pos {
                Ok(child_ix) => {
                    self.nodes.remove(key);
                    inserted = false;
                    key = self.toplevel[child_ix];
                }
                Err(child_ix) => {
                    self.toplevel.insert(child_ix, key);
                }
            };
        }
        if inserted {
            if directory {
                sorted_vec_insert(&mut self.directories, key);
            } else {
                sorted_vec_insert(&mut self.files, key);
                if overlay {
                    sorted_vec_insert(&mut self.overlay_files, key);
                } else {
                    sorted_vec_insert(&mut self.regular_files, key);
                }
            }
        } else {
            assert!(directory && self.nodes[key].removed_directory);
            self.nodes[key].removed_directory = false;
        }
    }

    fn on_setup_overlay_node(&mut self, key: NodeKey) {
        let node = &self.nodes[key];
        assert!(!node.overlay && !node.directory);

        let has_overlay = {
            let nodes = if let Some(parent) = node.parent {
                &self.nodes[parent].children
            } else {
                &self.toplevel
            };
            find_sorted_by_name_overlay(nodes, &node.name, true, &self.nodes).is_ok()
        };
        if has_overlay {
            return; // just ignore duplicate overlays instead of only generating new ones
        }

        self.on_add_node(
            node.name.clone(),
            false,
            true,
            node.raw_content.clone(),
            node.manifest.clone(),
            node.parent,
        );
    }

    fn on_remove_node(&mut self, key: NodeKey) {
        self.last_removed_node_path = Some(self.node_path(key));
        self.last_removed_node_is_overlay = Some(self.node_overlay(key));
        self.do_remove_node(key, false);
    }

    fn do_remove_node(&mut self, key: NodeKey, recur: bool) {
        let node = &self.nodes[key];
        if node.directory {
            assert!(recur || !node.removed_directory);
            self.nodes[key].removed_directory = true;
            for child_key in self.nodes[key].children.clone() {
                self.do_remove_node(child_key, true);
            }
        } else if !(recur && node.overlay) {
            // Unlink the node from the parent.
            if let Some(parent_key) = node.parent {
                let child_ix = find_sorted_by_name_overlay(
                    &self.nodes[parent_key].children,
                    &node.name,
                    node.overlay,
                    &self.nodes,
                )
                .unwrap();
                (&mut self.nodes[parent_key]).children.remove(child_ix);
            } else {
                let child_ix = find_sorted_by_name_overlay(
                    &self.toplevel,
                    &node.name,
                    node.overlay,
                    &self.nodes,
                )
                .unwrap();
                self.toplevel.remove(child_ix);
            }
            // Purge unlinked node.
            self.force_remove(key);
        }
    }

    fn on_update_node(
        &mut self,
        key: NodeKey,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    ) {
        let node = &mut self.nodes[key];
        assert!(!node.directory);
        node.raw_content = raw_content;
        node.manifest = manifest;
    }

    fn on_save_overlay_node(&mut self, key: NodeKey) {
        assert!(self.nodes[key].overlay);

        let regular = {
            let nodes = if let Some(parent) = self.nodes[key].parent {
                &self.nodes[parent].children
            } else {
                &self.toplevel
            };
            let res = find_sorted_by_name_overlay(nodes, &self.nodes[key].name, false, &self.nodes);
            res.ok().map(|ix| nodes[ix])
        };

        if let Some(regular) = regular {
            self.nodes[regular].raw_content = self.nodes[key].raw_content.clone();
            self.nodes[regular].manifest = self.nodes[key].manifest.clone();
        } else {
            let mut parent = self.nodes[key].parent;
            while let Some(parent_key) = parent {
                self.nodes[parent_key].removed_directory = false;
                parent = self.nodes[parent_key].parent;
            }
            let node = &self.nodes[key];
            self.on_add_node(
                node.name.clone(),
                false,
                false,
                node.raw_content.clone(),
                node.manifest.clone(),
                node.parent,
            );
        }
    }

    fn on_mark_scan_root(&mut self, key: NodeKey) {
        assert!(self.nodes[key].directory);
        self.scan_roots.insert(self.node_path(key));
    }

    fn on_scan(&mut self) {
        let mut root_keys: Vec<NodeKey> = self
            .directories
            .iter()
            .copied()
            .filter(|key| {
                let path = self.node_path(*key);
                self.scan_roots.contains(&path)
            })
            .collect();
        root_keys.sort();
        for (key, node) in &mut self.nodes {
            node.scan_root = root_keys.binary_search(&key).is_ok();
        }
        self.last_scan = self.reconstruct();
        if self.cleanup_stale_roots {
            self.scan_roots.retain(|r| {
                self.files.iter().any(|k| {
                    Self::node_path_impl(&self.nodes, *k).file_name()
                        == Some(MODULE_MANIFEST_FILENAME.as_ref())
                        && util::has_prefix(&Self::node_path_impl(&self.nodes, *k), r)
                })
            });
        }
    }

    fn force_remove(&mut self, key: NodeKey) -> Node {
        let node = self.nodes.remove(key).unwrap();
        if node.directory {
            sorted_vec_remove(&mut self.directories, &key);
        } else {
            sorted_vec_remove(&mut self.files, &key);
            if node.overlay {
                sorted_vec_remove(&mut self.overlay_files, &key);
            } else {
                sorted_vec_remove(&mut self.regular_files, &key);
            }
        }
        node
    }

    fn node_path(&self, key: NodeKey) -> PathBuf {
        Self::node_path_impl(&self.nodes, key)
    }

    fn node_overlay(&self, key: NodeKey) -> bool {
        self.nodes[key].overlay
    }

    fn node_path_impl(nodes: &SlotMap<NodeKey, Node>, mut key: NodeKey) -> PathBuf {
        let mut rev = PathBuf::new();
        loop {
            let node = &nodes[key];
            rev.push(&node.name);
            if let Some(parent_key) = node.parent {
                key = parent_key;
            } else {
                break;
            }
        }
        PathBuf::from_iter(rev.components().rev())
    }

    fn reconstruct(&self) -> Vec<Module> {
        let mut modules: Vec<Module> = Vec::new();
        for top_key in &self.toplevel {
            let node = &self.nodes[*top_key];
            if node.directory {
                let mut module_map: types::HashMap<ModuleLoc, Module> = types::HashMap::default();
                self.reconstruct_subtree(&mut module_map, PathBuf::new(), None, node, None, false);
                modules.extend(module_map.into_values());
            }
        }
        modules
    }

    fn reconstruct_subtree(
        &self,
        modules: &mut types::HashMap<ModuleLoc, Module>,
        parent_source: PathBuf,
        parent_path: Option<types::ImportPath>,
        node: &Node,
        mut cur_module: Option<ModuleLoc>,
        mut in_scan_root: bool,
    ) {
        let cur_dir_source = parent_source.join(&node.name);
        let mut cur_pkg_path = parent_path.and_then(|p| p.join(&node.name).ok());
        // Until we hit a scan root, do nothing.
        in_scan_root = in_scan_root || node.scan_root;
        if in_scan_root {
            // Should we update the current module?
            let manifest_loc = node
                .find_child(MODULE_MANIFEST_FILENAME, true, &self.nodes)
                .or_else(|_| node.find_child(MODULE_MANIFEST_FILENAME, false, &self.nodes));
            if let Ok(manifest_ix) = manifest_loc {
                let manifest_key = node.children[manifest_ix];
                let manifest_node = &self.nodes[manifest_key];
                if !manifest_node.directory {
                    let loc = ModuleLoc {
                        source: cur_dir_source.clone(),
                        module: manifest_node.manifest.as_ref().map(|m| m.clone()),
                    };
                    cur_module = Some(loc.clone());
                    cur_pkg_path = loc.module.clone();
                    modules.insert(
                        loc.clone(),
                        Module {
                            source: loc.source,
                            path: loc.module,
                            manifest: types::Text::from(format_manifest(
                                manifest_node.manifest.clone(),
                            )),
                            packages: Vec::new(),
                        },
                    );
                }
            }
            // If we are inside a module and have hit an invalid package directory, reset the import path.
            if let Some(loc) = &cur_module {
                if cur_dir_source != loc.source && !util::valid_package_name(&node.name) {
                    cur_pkg_path = None;
                }
                let pkg = self.node_package(node, cur_dir_source.clone(), cur_pkg_path.clone());
                modules.get_mut(&loc).unwrap().packages.extend(pkg);
            }
        }
        for child_key in &node.children {
            let child_node = &self.nodes[*child_key];
            if child_node.directory {
                self.reconstruct_subtree(
                    modules,
                    cur_dir_source.clone(),
                    cur_pkg_path.clone(),
                    child_node,
                    cur_module.clone(),
                    in_scan_root,
                );
            }
        }
    }

    fn removed_node_is_empty(&self, node: &Node) -> bool {
        assert!(node.removed_directory);

        let subdirs_empty = node
            .children
            .iter()
            .map(|key| &self.nodes[*key])
            .filter(|n| n.directory)
            .all(|n| self.removed_node_is_empty(n));
        let files_empty = !node.children.iter().map(|key| &self.nodes[*key]).any(|n| {
            !n.directory
                && (n.name == MODULE_MANIFEST_FILENAME || util::valid_source_extension(&n.name))
        });

        subdirs_empty && files_empty
    }

    fn node_package(
        &self,
        node: &Node,
        cur_dir_source: PathBuf,
        cur_pkg_path: Option<types::ImportPath>,
    ) -> Option<Package> {
        if node.removed_directory && self.removed_node_is_empty(node) {
            return None;
        }
        let regular_files = self.node_child_files(node, &cur_dir_source, false);
        let overlay_files = self.node_child_files(node, &cur_dir_source, true);
        let mut files = Vec::with_capacity(regular_files.capacity());
        let mut regular_files_iter = regular_files.into_iter().peekable();
        let mut overlay_files_iter = overlay_files.into_iter().peekable();
        loop {
            let (regular, overlay) =
                match (regular_files_iter.peek_mut(), overlay_files_iter.peek_mut()) {
                    (None, None) => break,
                    (Some(regular_file), Some(overlay_file)) => {
                        match regular_file.source.cmp(&overlay_file.source) {
                            Ordering::Equal => (Some(regular_file), Some(overlay_file)),
                            Ordering::Less => (Some(regular_file), None),
                            Ordering::Greater => (None, Some(overlay_file)),
                        }
                    }
                    only_one => only_one,
                };
            match (regular, overlay) {
                (Some(_), Some(overlay_file)) => {
                    files.push(take(overlay_file));
                    regular_files_iter.next();
                    overlay_files_iter.next();
                }
                (Some(regular_file), None) => {
                    files.push(take(regular_file));
                    regular_files_iter.next();
                }
                (None, Some(overlay_file)) => {
                    files.push(take(overlay_file));
                    overlay_files_iter.next();
                }
                _ => unreachable!("at least one file must be set"),
            }
        }
        Some(Package {
            source: cur_dir_source,
            path: cur_pkg_path,
            files,
        })
    }

    fn node_child_files(&self, node: &Node, cur_dir_source: &Path, overlay: bool) -> Vec<File> {
        assert!(node.directory);
        node.children
            .iter()
            .map(|key| &self.nodes[*key])
            .filter(|node| {
                !node.directory
                    && node.overlay == overlay
                    && util::valid_source_extension(&node.name)
            })
            .map(|node| File {
                source: cur_dir_source.join(&node.name),
                content: types::Text::from(String::from_utf8_lossy(&node.raw_content)),
                detached: !util::valid_source_file_name(&node.name),
            })
            .collect()
    }
}

impl ReferenceStateMachine for VfsReferenceMachine {
    type State = Self;
    type Transition = Transition;

    fn init_state() -> BoxedStrategy<Self::State> {
        let rng_seed_strategy = any::<u64>();
        let async_vfs_strategy = any::<bool>();
        let cleanup_stale_roots_strategy = any::<bool>();
        (
            rng_seed_strategy,
            async_vfs_strategy,
            cleanup_stale_roots_strategy,
        )
            .prop_flat_map(|(rng_seed, async_vfs, cleanup_stale_roots)| {
                Just(VfsReferenceMachine::new(
                    rng_seed,
                    async_vfs,
                    cleanup_stale_roots,
                ))
            })
            .boxed()
    }

    fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition> {
        let mut options: Vec<(u32, BoxedStrategy<Transition>)> = Vec::new();
        let gen_overlay = !state.async_vfs;

        options.push((2, add_node_strategy(gen_overlay, state.directories.clone())));
        if gen_overlay && !state.regular_files.is_empty() {
            options.push((1, setup_overlay_node_strategy(state.regular_files.clone())));
        }
        if !state.nodes.is_empty() {
            options.push((1, remove_node_strategy(state.nodes.keys().collect())));
        }
        if !state.files.is_empty() {
            options.push((3, update_node_strategy(state.files.clone())));
        }
        if gen_overlay && !state.overlay_files.is_empty() {
            options.push((1, save_overlay_node_strategy(state.overlay_files.clone())));
        }
        if !state.directories.is_empty() {
            options.push((1, mark_scan_root_strategy(state.directories.clone())));
        }
        options.push((2, scan_strategy()));

        Union::new_weighted(options).boxed()
    }

    fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            Transition::AddNode {
                name,
                directory,
                overlay,
                raw_content,
                manifest,
                parent,
            } => {
                state.on_add_node(
                    name.clone(),
                    *directory,
                    *overlay,
                    raw_content.clone(),
                    manifest.clone(),
                    *parent,
                );
            }
            Transition::SetupOverlayNode { key } => {
                state.on_setup_overlay_node(*key);
            }
            Transition::RemoveNode { key } => {
                state.on_remove_node(*key);
            }
            Transition::UpdateNode {
                key,
                raw_content,
                manifest,
            } => {
                state.on_update_node(*key, raw_content.clone(), manifest.clone());
            }
            Transition::SaveOverlayNode { key } => {
                state.on_save_overlay_node(*key);
            }
            Transition::MarkScanRoot { key } => {
                state.on_mark_scan_root(*key);
            }
            Transition::Scan {} => {
                state.on_scan();
            }
        };
        state
    }

    fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool {
        match transition {
            Transition::AddNode {
                name,
                directory,
                overlay,
                parent,
                ..
            } => {
                let siblings = if let Some(parent_key) = parent {
                    if !sorted_vec_contains(&state.directories, parent_key)
                        || (state.nodes[*parent_key].removed_directory && !*overlay)
                    {
                        // Parent must be a directory. If it is a removed ones, only overlays are possible.
                        return false;
                    }
                    &state.nodes[*parent_key].children
                } else {
                    &state.toplevel
                };
                // Sibling with the same name is only OK if it is a removed directory
                // and we are (re-)creating a regular directory in its place.
                let regular_ix =
                    find_sorted_by_name_overlay(siblings, &name, false, &state.nodes).ok();
                let overlay_ix =
                    find_sorted_by_name_overlay(siblings, &name, true, &state.nodes).ok();
                match (regular_ix, overlay_ix) {
                    (Some(regular_ix), None) if *directory => {
                        state.nodes[siblings[regular_ix]].removed_directory
                    }
                    (None, None) => true,
                    _ => false,
                }
            }
            Transition::SetupOverlayNode { key } => {
                if let Some(node) = state.nodes.get(*key) {
                    !node.overlay && !node.directory
                } else {
                    false
                }
            }
            Transition::RemoveNode { key } => {
                let node = state.nodes.get(*key);
                if let Some(node) = node {
                    !node.removed_directory
                } else {
                    false
                }
            }
            Transition::UpdateNode { key, .. } => {
                state.nodes.contains_key(*key) && sorted_vec_contains(&state.files, key)
            }
            Transition::SaveOverlayNode { key } => {
                state.nodes.contains_key(*key) && state.node_overlay(*key)
            }
            Transition::MarkScanRoot { key } => {
                // We consider removed directory to be OK.
                state.nodes.contains_key(*key) && sorted_vec_contains(&state.directories, key)
            }
            Transition::Scan {} => true,
        }
    }
}

struct VfsMachine {
    rng: fastrand::Rng,
    dir: tempfile::TempDir,
    inode_holders: Vec<fs::File>,
    vfs: Option<Vfs>,
    state: Option<Arc<types::SyncState>>,
    vfs_join_handle: Option<thread::JoinHandle<()>>,
    last_scan_id: u64,
    pending_updates: Vec<workspace::ModuleUpdate>,
    pending_overlay_updates: Vec<types::OverlayUpdate>,
    overlay: types::HashMap<PathBuf, (Vec<u8>, i32)>,
    changed_since_last_scan: bool,
    modules: types::HashMap<ModuleLoc, Module>,
}

impl Drop for VfsMachine {
    fn drop(&mut self) {
        if self.vfs_join_handle.is_some() {
            self.state.as_ref().unwrap().signal_exit();
            take(&mut self.vfs_join_handle)
                .unwrap()
                .join()
                .expect("vfs run must finish successfully");
        }
    }
}

impl VfsMachine {
    fn new(rng_seed: u64, async_vfs: bool, cleanup_stale_roots: bool) -> VfsMachine {
        let (vfs, state, handle) = if async_vfs {
            let state = Arc::new(types::SyncState::new());
            let state_clone = state.clone();
            let h = thread::Builder::new()
                .name("VFS".to_owned())
                .spawn(move || {
                    let mut vfs = Vfs::new(cleanup_stale_roots, Duration::from_secs(86400));
                    vfs.run(&state_clone);
                })
                .unwrap();
            (None, Some(state), Some(h))
        } else {
            let vfs = Vfs::new(cleanup_stale_roots, Duration::from_secs(0));
            (Some(vfs), None, None)
        };
        VfsMachine {
            rng: fastrand::Rng::with_seed(rng_seed),
            dir: tempfile::TempDir::new()
                .expect("it must be possible to create a temporary directory"),
            inode_holders: Vec::new(),
            vfs,
            state,
            vfs_join_handle: handle,
            last_scan_id: 0,
            modules: types::HashMap::default(),
            pending_updates: Vec::new(),
            pending_overlay_updates: Vec::new(),
            overlay: types::HashMap::default(),
            changed_since_last_scan: true,
        }
    }

    fn full_path(&self, path: PathBuf) -> PathBuf {
        self.dir.path().join(path)
    }

    fn add_node(
        &mut self,
        path: PathBuf,
        directory: bool,
        is_manifest: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    ) {
        if directory {
            fs::create_dir(path).unwrap();
        } else {
            let mut file = fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(path)
                .unwrap();
            Self::write_content(&mut file, is_manifest, raw_content, manifest);
        }
    }

    fn add_overlay_node(
        &mut self,
        path: PathBuf,
        is_manifest: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    ) {
        let mut content = Vec::new();
        Self::write_content(&mut content, is_manifest, raw_content, manifest);
        self.add_overlay(path, content);
    }

    fn setup_overlay_node(&mut self, path: PathBuf) {
        if !self.overlay.contains_key(&path) {
            let mut file = std::fs::File::open(&path).unwrap();
            let mut content = Vec::new();
            file.read_to_end(&mut content).unwrap();
            self.add_overlay(path, content);
        }
    }

    fn add_overlay(&mut self, path: PathBuf, content: Vec<u8>) {
        let text_content = types::Text::from(String::from_utf8_lossy(&content));
        let prev = self.overlay.insert(path.clone(), (content, 0));
        assert!(prev.is_none());
        self.pending_overlay_updates
            .push(types::OverlayUpdate::AddOverlay(path, text_content, 0));
    }

    fn remove_node(&mut self, path: PathBuf) {
        // Instead of removing, make the files hidden to avoid inode reuse.
        let hidden = path
            .parent()
            .unwrap()
            .join(String::from(".") + &self.rng.u128(0..u128::MAX).to_string());
        fs::rename(path, hidden).unwrap();
    }

    fn remove_overlay_node(&mut self, path: PathBuf) {
        self.overlay.remove(&path).unwrap();
        self.pending_overlay_updates
            .push(types::OverlayUpdate::RemoveOverlay(path));
    }

    fn update_node(
        &mut self,
        path: PathBuf,
        write_manifest: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    ) {
        // We remove and re-add the file to guarantee the metadata change (at least on Linux,
        // the inode number should be different). Otherwise, we can miss updates
        // due to the filesystem using the cached timestamps.
        let _ = fs::remove_file(&path);
        // We hope dearly that new temporary file can avoid inode reuse problem.
        let tmp = tempfile::tempfile().unwrap();
        self.inode_holders.push(tmp);
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .unwrap();
        Self::write_content(&mut file, write_manifest, raw_content, manifest);
    }

    fn update_overlay_node(
        &mut self,
        path: PathBuf,
        write_manifest: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    ) {
        let mut content = Vec::new();
        Self::write_content(&mut content, write_manifest, raw_content, manifest);
        let over = self.overlay.get_mut(&path).unwrap();
        over.0 = content;
        over.1 += 1;
        let change = types::OverlayChange {
            content: String::from_utf8_lossy(&over.0).into(),
            range: None,
            utf8_pos: false,
        };
        self.pending_overlay_updates
            .push(types::OverlayUpdate::ChangeOverlay(
                path,
                vec![change],
                over.1,
            ));
    }

    fn save_overlay_node(&mut self, path: PathBuf) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap(); // overlay node may have its parents removed
        let content = self.overlay.get(&path).unwrap().0.clone();
        self.update_node(path, false, content, None);
    }

    fn mark_scan_root(&mut self, path: PathBuf) {
        if let Some(vfs) = &mut self.vfs {
            let updates = vfs.__test_add_root(path);
            self.pending_updates.extend(updates);
        } else {
            self.state
                .as_ref()
                .unwrap()
                .signal_vfs_new_roots(vec![path]);
        }
    }

    fn write_content(
        w: &mut dyn std::io::Write,
        write_manifest: bool,
        raw_content: Vec<u8>,
        manifest: Option<types::ImportPath>,
    ) {
        if write_manifest {
            let content = format_manifest(manifest);
            w.write_all(content.as_bytes()).unwrap();
        } else {
            w.write_all(&raw_content).unwrap();
        }
    }

    fn scan(&mut self) {
        let pending = take(&mut self.pending_updates);
        self.apply(pending);
        let updates = if let Some(vfs) = &mut self.vfs {
            let overlay_pending = take(&mut self.pending_overlay_updates);
            let should_scan = self.changed_since_last_scan || overlay_pending.len() % 2 == 0;
            vfs.__test_scan(should_scan, overlay_pending)
        } else {
            let state = self.state.as_ref().unwrap().as_ref();
            self.last_scan_id += 1;
            state.signal_vfs_force_scan(self.last_scan_id);
            let mut updates: Vec<workspace::ModuleUpdate> = Vec::new();
            loop {
                let buf = state.wait_core();
                updates.extend(buf.module_updates);
                if let Some(id) = buf.last_force_scan_id {
                    if id == self.last_scan_id {
                        break;
                    }
                }
            }
            updates
        };
        self.apply(updates);
    }

    fn apply(&mut self, updates: Vec<workspace::ModuleUpdate>) {
        for m_upd in updates {
            match m_upd.update {
                workspace::UpdateKind::Added => {
                    let prev = self.modules.insert(
                        ModuleLoc {
                            source: m_upd.source.clone(),
                            module: m_upd.module.clone(),
                        },
                        Module {
                            source: m_upd.source,
                            path: m_upd.module,
                            manifest: m_upd.manifest.unwrap(),
                            packages: m_upd
                                .packages
                                .into_iter()
                                .map(|p_upd| Package {
                                    source: p_upd.source,
                                    path: p_upd.path,
                                    files: p_upd
                                        .files
                                        .into_iter()
                                        .map(|f| File {
                                            source: f.source,
                                            content: f.content.unwrap(),
                                            detached: f.detached,
                                        })
                                        .collect(),
                                })
                                .collect(),
                        },
                    );
                    assert!(prev.is_none());
                }
                workspace::UpdateKind::Removed => {
                    let prev = self.modules.remove(&ModuleLoc {
                        source: m_upd.source,
                        module: m_upd.module,
                    });
                    assert!(prev.is_some());
                }
                workspace::UpdateKind::Updated => {
                    let m = self
                        .modules
                        .get_mut(&ModuleLoc {
                            source: m_upd.source,
                            module: m_upd.module,
                        })
                        .unwrap();
                    if let Some(manifest) = m_upd.manifest {
                        m.manifest = manifest;
                    }
                    for p_upd in m_upd.packages {
                        match p_upd.update {
                            workspace::UpdateKind::Added => {
                                m.packages.push(Package {
                                    source: p_upd.source,
                                    path: p_upd.path,
                                    files: p_upd
                                        .files
                                        .into_iter()
                                        .map(|f| File {
                                            source: f.source,
                                            content: f.content.unwrap(),
                                            detached: f.detached,
                                        })
                                        .collect(),
                                });
                            }
                            workspace::UpdateKind::Removed => {
                                let ix = m
                                    .packages
                                    .iter()
                                    .position(|p| p.source == p_upd.source && p.path == p_upd.path)
                                    .unwrap();
                                m.packages.swap_remove(ix);
                            }
                            workspace::UpdateKind::Updated => {
                                // Yup, O(N^2).
                                let p = m
                                    .packages
                                    .iter_mut()
                                    .find(|p| p.source == p_upd.source && p.path == p_upd.path)
                                    .unwrap();
                                for f_upd in p_upd.files {
                                    let ix = p.files.iter().position(|f| f.source == f_upd.source);
                                    if let Some(ix) = ix {
                                        if let Some(content) = f_upd.content {
                                            p.files[ix].content = content;
                                        } else {
                                            p.files.swap_remove(ix);
                                        }
                                    } else {
                                        let content = f_upd.content.unwrap();
                                        p.files.push(File {
                                            source: f_upd.source,
                                            content,
                                            detached: f_upd.detached,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl StateMachineTest for VfsMachine {
    type SystemUnderTest = Self;
    type Reference = VfsReferenceMachine;

    fn init_test(
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        log::debug!("[begin new VFS state machine run] ==========================================");
        VfsMachine::new(
            ref_state.rng_seed,
            ref_state.async_vfs,
            ref_state.cleanup_stale_roots,
        )
    }

    fn apply(
        mut state: Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        transition: <Self::Reference as ReferenceStateMachine>::Transition,
    ) -> Self::SystemUnderTest {
        log::debug!("EXECUTING {transition:?} ===================================================");
        let mut is_scan = false;
        let mut is_overlay = false;
        match transition {
            Transition::AddNode {
                name,
                directory,
                overlay,
                raw_content,
                manifest,
                parent: _,
            } => {
                let parent =
                    state.full_path(ref_state.last_added_node_parent.clone().unwrap_or_default());
                let is_manifest = name == MODULE_MANIFEST_FILENAME;
                let path = parent.join(name);
                if overlay {
                    log::debug!("OVERLAY variant");
                    state.add_overlay_node(path, is_manifest, raw_content, manifest);
                    is_overlay = true;
                } else {
                    state.add_node(path, directory, is_manifest, raw_content, manifest);
                }
            }
            Transition::SetupOverlayNode { key } => {
                let path = state.full_path(ref_state.node_path(key));
                state.setup_overlay_node(path);
                is_overlay = true;
            }
            Transition::RemoveNode { .. } => {
                let path = state.full_path(ref_state.last_removed_node_path.clone().unwrap());
                if ref_state.last_removed_node_is_overlay.unwrap() {
                    log::debug!("OVERLAY variant");
                    state.remove_overlay_node(path);
                    is_overlay = true;
                } else {
                    state.remove_node(path);
                }
            }
            Transition::UpdateNode {
                key,
                raw_content,
                manifest,
            } => {
                let path = state.full_path(ref_state.node_path(key));
                let overlay = ref_state.node_overlay(key);
                let write_manifest = path.file_name() == Some(MODULE_MANIFEST_FILENAME.as_ref());
                if overlay {
                    log::debug!("OVERLAY variant");
                    state.update_overlay_node(path, write_manifest, raw_content, manifest);
                    is_overlay = true;
                } else {
                    state.update_node(path, write_manifest, raw_content, manifest);
                }
            }
            Transition::SaveOverlayNode { key } => {
                let path = state.full_path(ref_state.node_path(key));
                state.save_overlay_node(path);
            }
            Transition::MarkScanRoot { key } => {
                let path = state.full_path(ref_state.node_path(key));
                state.mark_scan_root(path);
            }
            Transition::Scan {} => {
                state.scan();
                is_scan = true;
            }
        };
        if is_scan {
            state.changed_since_last_scan = false;
        } else if !is_overlay {
            state.changed_since_last_scan = true;
        }
        state
    }

    fn check_invariants(
        state: &Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
        let modules = canonicalize_modules(
            state.modules.values().cloned().collect(),
            Some(state.dir.path()),
        );
        let ref_modules = canonicalize_modules(ref_state.last_scan.clone(), None);
        assert_eq!(
            ref_modules, modules,
            "modules (right) does not match reference modules (left)"
        );

        for m in &modules {
            for p in &m.packages {
                for f in &p.files {
                    let file_name = f.source.file_name().unwrap().to_str().unwrap();
                    assert!(f.detached || util::valid_source_file_name(file_name));
                }
            }
        }
    }
}

fn fix_source(source: &mut PathBuf, root: Option<&Path>) {
    if let Some(root) = root {
        *source = source.strip_prefix(root).unwrap().to_owned();
    }
}

fn canonicalize_modules(mut modules: Vec<Module>, root: Option<&Path>) -> Vec<Module> {
    for m in &mut modules {
        for p in &mut m.packages {
            for f in &mut p.files {
                fix_source(&mut f.source, root);
            }
            p.files.sort_by(|a, b| a.source.cmp(&b.source));
            fix_source(&mut p.source, root);
        }
        m.packages.sort_by(|a, b| a.source.cmp(&b.source));
        fix_source(&mut m.source, root);
    }
    modules.sort_by(|a, b| a.source.cmp(&b.source));
    modules
}
