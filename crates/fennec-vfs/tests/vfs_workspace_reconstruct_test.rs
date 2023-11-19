extern crate proptest;
extern crate proptest_state_machine;

use std::{
    fs::{self},
    io::Write,
    mem::take,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use fennec_common::{types, util, workspace, MODULE_MANIFEST_FILENAME, PROJECT_NAME};
use fennec_vfs::Vfs;
use proptest::{prelude::*, sample, strategy::Union};
use proptest_state_machine::{ReferenceStateMachine, StateMachineTest};
use slotmap::SlotMap;

fn vfs_config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: None, // unfortunately overriden inside the proptest! macro, leading to warnings like https://github.com/proptest-rs/proptest/issues/233
        ..ProptestConfig::default()
    }
}

// Ensure that the VFS correctly reconstructs the workspace state.
#[test]
fn vfs_workspace_reconstruct_test() {
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
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
        parent: Option<NodeKey>,
    },
    RemoveNode {
        key: NodeKey,
    },
    UpdateNode {
        key: NodeKey,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
    },
    MarkScanRoot {
        key: NodeKey,
    },
    Scan {},
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transition::AddNode {
                name,
                directory,
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
                        "Add MANIFEST {name}, parent {parent:?}, valid manifest {valid_manifest}"
                    )
                } else {
                    let n = raw_content.len();
                    let pretty_content = String::from_utf8_lossy(&raw_content);
                    write!(
                        f,
                        "Add FILE {name}, parent {parent:?}, content ({n}) {pretty_content}"
                    )
                }
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
        NodeNameKind::ModuleManifest => "fennec\\.toml",
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

fn fennec_version_strategy() -> BoxedStrategy<types::FennecVersion> {
    ((0u64..=10u64), (0u64..=100u64), (0u64..=100u64))
        .prop_map(|(major, minor, patch)| types::FennecVersion {
            major,
            minor,
            patch,
        })
        .boxed()
}

fn raw_content_strategy() -> BoxedStrategy<Vec<u8>> {
    // Proptest is having much easier time with minimization when limiting content to 1 byte.
    any::<Option<u8>>()
        .prop_map(|v| v.into_iter().collect::<Vec<u8>>())
        .boxed()
}

fn manifest_strategy() -> BoxedStrategy<Option<workspace::ModuleManifest>> {
    Union::new(vec![
        Just(None).boxed(),
        (import_path_strategy(), fennec_version_strategy())
            .prop_map(|(path, version)| {
                Some(workspace::ModuleManifest {
                    module: path,
                    fennec: version,
                })
            })
            .boxed(),
    ])
    .boxed()
}

fn add_node_strategy(parents: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let name_kind_strategy = Union::new(vec![
        Just(NodeNameKind::Other),
        Just(NodeNameKind::SourceCode),
        Just(NodeNameKind::ModuleManifest),
    ]);
    let valid_ident_strategy = any::<bool>();
    let name_strategy = (name_kind_strategy, valid_ident_strategy)
        .prop_flat_map(|(name_kind, valid_ident)| node_name_strategy(name_kind, valid_ident));
    let directory_strategy = any::<bool>();
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
        directory_strategy,
        raw_content_strategy(),
        manifest_strategy(),
        maybe_parent_strategy,
    )
        .prop_map(
            |(name, directory, raw_content, manifest, parent)| Transition::AddNode {
                name,
                directory,
                raw_content,
                manifest,
                parent,
            },
        )
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
    scan_root: bool,
    raw_content: Vec<u8>,
    manifest: Option<workspace::ModuleManifest>,
    parent: Option<NodeKey>,
    children: Vec<NodeKey>,
}

impl Node {
    fn new(
        name: String,
        directory: bool,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
        parent: Option<NodeKey>,
    ) -> Node {
        Node {
            name,
            directory,
            scan_root: false,
            raw_content,
            manifest,
            parent,
            children: Vec::new(),
        }
    }

    fn find_child(&self, name: &str, nodes: &SlotMap<NodeKey, Node>) -> Result<usize, usize> {
        find_sorted_by_name(&self.children, name, nodes)
    }
}

fn format_manifest(manifest: Option<workspace::ModuleManifest>) -> String {
    if let Some(manifest) = manifest {
        let mut buf = Vec::new();
        let types::FennecVersion {
            major,
            minor,
            patch,
        } = manifest.fennec;
        let mod_path = manifest.module;
        write!(
            buf,
            "{PROJECT_NAME} = \"{major}.{minor}.{patch}\"\nmodule = \"{mod_path}\"\n"
        )
        .unwrap();
        String::from_utf8(buf).unwrap()
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, Default)]
struct VfsReferenceMachine {
    nodes: SlotMap<NodeKey, Node>,
    toplevel: Vec<NodeKey>,
    files: Vec<NodeKey>,
    directories: Vec<NodeKey>,
    scan_roots: types::HashSet<PathBuf>,
    last_scan: Vec<workspace::Module>,
    // Kludges to simplify name resolution in real state machine.
    last_added_node_parent: Option<PathBuf>,
    last_removed_node_path: Option<PathBuf>,
    // Hack to vary initialization of the SUT.
    rng_seed: u64,
    async_vfs: bool,
    cleanup_stale_roots: bool,
}

fn find_sorted_by_name(
    v: &Vec<NodeKey>,
    name: &str,
    nodes: &SlotMap<NodeKey, Node>,
) -> Result<usize, usize> {
    v.binary_search_by_key(&name, |k| &nodes[*k].name)
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

    fn add_node(
        &mut self,
        name: String,
        directory: bool,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
        parent: Option<NodeKey>,
    ) {
        self.last_added_node_parent = parent.map(|key| self.node_path(key));
        let key = self.nodes.insert(Node::new(
            name.clone(),
            directory,
            raw_content,
            manifest,
            parent,
        ));
        if let Some(parent_key) = parent {
            let child_ix =
                find_sorted_by_name(&self.nodes[parent_key].children, &name, &self.nodes)
                    .unwrap_err();
            (&mut self.nodes[parent_key]).children.insert(child_ix, key);
        } else {
            let child_ix = find_sorted_by_name(&self.toplevel, &name, &self.nodes).unwrap_err();
            self.toplevel.insert(child_ix, key);
        }
        if directory {
            sorted_vec_insert(&mut self.directories, key);
        } else {
            sorted_vec_insert(&mut self.files, key);
        }
    }

    fn remove_node(&mut self, key: NodeKey) {
        self.do_remove_node(key, true);
    }

    fn do_remove_node(&mut self, key: NodeKey, unlink_from_parent: bool) {
        if unlink_from_parent {
            self.last_removed_node_path = Some(self.node_path(key));
            let node = &self.nodes[key];
            if let Some(parent_key) = node.parent {
                let child_ix =
                    find_sorted_by_name(&self.nodes[parent_key].children, &node.name, &self.nodes)
                        .unwrap();
                (&mut self.nodes[parent_key]).children.remove(child_ix);
            } else {
                let child_ix =
                    find_sorted_by_name(&self.toplevel, &node.name, &self.nodes).unwrap();
                self.toplevel.remove(child_ix);
            }
        }
        let node = self.remove(key);
        for child in node.children {
            self.do_remove_node(child, false)
        }
    }

    fn update_node(
        &mut self,
        key: NodeKey,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
    ) {
        let node = &mut self.nodes[key];
        assert!(!node.directory);
        node.raw_content = raw_content;
        node.manifest = manifest;
    }

    fn mark_scan_root(&mut self, key: NodeKey) {
        assert!(self.nodes[key].directory);
        self.scan_roots.insert(self.node_path(key));
    }

    fn scan(&mut self) {
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

    fn remove(&mut self, key: NodeKey) -> Node {
        let node = self.nodes.remove(key).unwrap();
        if node.directory {
            sorted_vec_remove(&mut self.directories, &key);
        } else {
            sorted_vec_remove(&mut self.files, &key);
        }
        node
    }

    fn node_path(&self, key: NodeKey) -> PathBuf {
        Self::node_path_impl(&self.nodes, key)
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

    fn reconstruct(&self) -> Vec<workspace::Module> {
        let mut modules: Vec<workspace::Module> = Vec::new();
        for top_key in &self.toplevel {
            let node = &self.nodes[*top_key];
            if node.directory {
                let mut module_map: types::HashMap<ModuleLoc, workspace::Module> =
                    types::HashMap::default();
                self.reconstruct_subtree(&mut module_map, PathBuf::new(), None, node, None, false);
                modules.extend(module_map.into_values());
            }
        }
        modules
    }

    fn reconstruct_subtree(
        &self,
        modules: &mut types::HashMap<ModuleLoc, workspace::Module>,
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
            let manifest_loc = node.find_child(MODULE_MANIFEST_FILENAME, &self.nodes);
            if let Ok(manifest_ix) = manifest_loc {
                let manifest_key = node.children[manifest_ix];
                let manifest_node = &self.nodes[manifest_key];
                if !manifest_node.directory {
                    let loc = ModuleLoc {
                        source: cur_dir_source.clone(),
                        module: manifest_node.manifest.as_ref().map(|m| m.module.clone()),
                    };
                    cur_module = Some(loc.clone());
                    cur_pkg_path = loc.module.clone();
                    modules.insert(
                        loc.clone(),
                        workspace::Module {
                            source: loc.source,
                            path: loc.module,
                            manifest: Arc::from(format_manifest(manifest_node.manifest.clone())),
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
                let pkg = workspace::Package {
                    source: cur_dir_source.clone(),
                    path: cur_pkg_path.clone(),
                    files: node
                        .children
                        .iter()
                        .map(|key| &self.nodes[*key])
                        .filter(|node| !node.directory && util::valid_source_extension(&node.name))
                        .map(|node| workspace::File {
                            source: cur_dir_source.join(&node.name),
                            content: Arc::from(String::from_utf8_lossy(&node.raw_content)),
                            detached: !util::valid_source_file_name(&node.name),
                        })
                        .collect(),
                };
                modules.get_mut(&loc).unwrap().packages.push(pkg);
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

        options.push((2, add_node_strategy(state.directories.clone())));
        if !state.nodes.is_empty() {
            options.push((1, remove_node_strategy(state.nodes.keys().collect())));
        }
        if !state.files.is_empty() {
            options.push((2, update_node_strategy(state.files.clone())));
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
                raw_content,
                manifest,
                parent,
            } => {
                state.add_node(
                    name.clone(),
                    *directory,
                    raw_content.clone(),
                    manifest.clone(),
                    *parent,
                );
            }
            Transition::RemoveNode { key } => {
                state.remove_node(*key);
            }
            Transition::UpdateNode {
                key,
                raw_content,
                manifest,
            } => {
                state.update_node(*key, raw_content.clone(), manifest.clone());
            }
            Transition::MarkScanRoot { key } => {
                state.mark_scan_root(*key);
            }
            Transition::Scan {} => {
                state.scan();
            }
        };
        state
    }

    fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool {
        match transition {
            Transition::AddNode { name, parent, .. } => {
                if let Some(parent_key) = parent {
                    sorted_vec_contains(&state.directories, parent_key)
                        && (&state.nodes[*parent_key])
                            .find_child(&name, &state.nodes)
                            .is_err()
                } else {
                    find_sorted_by_name(&state.toplevel, &name, &state.nodes).is_err()
                }
            }
            Transition::RemoveNode { key } => state.nodes.contains_key(*key),
            Transition::UpdateNode { key, .. } => {
                state.nodes.contains_key(*key) && sorted_vec_contains(&state.files, key)
            }
            Transition::MarkScanRoot { key } => {
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
    modules: types::HashMap<ModuleLoc, workspace::Module>,
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
        }
    }

    fn full_path(&self, path: PathBuf) -> PathBuf {
        self.dir.path().join(path)
    }

    fn add_node(
        &mut self,
        name: String,
        directory: bool,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
        parent: PathBuf,
    ) {
        let is_manifest = name == MODULE_MANIFEST_FILENAME;
        let path = parent.join(name);
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

    fn remove_node(&mut self, path: PathBuf) {
        // Instead of removing, make the files hidden to avoid inode reuse.
        let hidden = path
            .parent()
            .unwrap()
            .join(String::from(".") + &self.rng.u128(0..u128::MAX).to_string());
        fs::rename(path, hidden).unwrap();
    }

    fn update_node(
        &mut self,
        path: PathBuf,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
    ) {
        let is_manifest = path.file_name() == Some(MODULE_MANIFEST_FILENAME.as_ref());
        // We remove and re-add the file to guarantee the metadata change (at least on Linux,
        // the inode number should be different). Otherwise, we can miss updates
        // due to the filesystem using the cached timestamps.
        fs::remove_file(&path).unwrap();
        // We hope dearly that new temporary file can avoid inode reuse problem.
        let tmp = tempfile::tempfile().unwrap();
        self.inode_holders.push(tmp);
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .unwrap();
        Self::write_content(&mut file, is_manifest, raw_content, manifest);
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
        file: &mut fs::File,
        write_manifest: bool,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
    ) {
        if write_manifest {
            let content = format_manifest(manifest);
            file.write_all(content.as_bytes()).unwrap();
        } else {
            file.write_all(&raw_content).unwrap();
        }
    }

    fn scan(&mut self) {
        let pending = take(&mut self.pending_updates);
        self.apply(pending);
        let updates = if let Some(vfs) = &mut self.vfs {
            vfs.__test_scan()
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
                        workspace::Module {
                            source: m_upd.source,
                            path: m_upd.module,
                            manifest: m_upd.manifest.unwrap(),
                            packages: m_upd
                                .packages
                                .into_iter()
                                .map(|p_upd| workspace::Package {
                                    source: p_upd.source,
                                    path: p_upd.path,
                                    files: p_upd
                                        .files
                                        .into_iter()
                                        .map(|f| workspace::File {
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
                                m.packages.push(workspace::Package {
                                    source: p_upd.source,
                                    path: p_upd.path,
                                    files: p_upd
                                        .files
                                        .into_iter()
                                        .map(|f| workspace::File {
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
                                        p.files.push(workspace::File {
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
        match transition {
            Transition::AddNode {
                name,
                directory,
                raw_content,
                manifest,
                ..
            } => {
                let parent =
                    state.full_path(ref_state.last_added_node_parent.clone().unwrap_or_default());
                state.add_node(name, directory, raw_content, manifest, parent);
            }
            Transition::RemoveNode { .. } => {
                let path = state.full_path(ref_state.last_removed_node_path.clone().unwrap());
                state.remove_node(path);
            }
            Transition::UpdateNode {
                key,
                raw_content,
                manifest,
            } => {
                let path = state.full_path(ref_state.node_path(key));
                state.update_node(path, raw_content, manifest);
            }
            Transition::MarkScanRoot { key } => {
                let path = state.full_path(ref_state.node_path(key));
                state.mark_scan_root(path);
            }
            Transition::Scan {} => {
                state.scan();
            }
        };
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
        pretty_assertions::assert_eq!(
            ref_modules,
            modules,
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

fn canonicalize_modules(
    mut modules: Vec<workspace::Module>,
    root: Option<&Path>,
) -> Vec<workspace::Module> {
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
