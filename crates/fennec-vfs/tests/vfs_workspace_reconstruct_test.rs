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
use tempfile::TempDir;

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
                    let pretty_content = String::from_utf8_lossy(&raw_content);
                    write!(
                        f,
                        "Add FILE {name}, parent {parent:?}, content {pretty_content}"
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
                let pretty_content = String::from_utf8_lossy(&raw_content);
                write!(
                    f,
                    "UPDATE {key:?}, valid manifest {valid_manifest}, content {pretty_content}"
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
    async_vfs: bool,
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
    module: types::ImportPath,
}

impl VfsReferenceMachine {
    fn new(async_vfs: bool) -> VfsReferenceMachine {
        VfsReferenceMachine {
            async_vfs,
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

    fn node_path(&self, mut key: NodeKey) -> PathBuf {
        let mut rev = PathBuf::new();
        loop {
            let node = &self.nodes[key];
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
                    cur_module = None;
                    cur_pkg_path = None;
                    if let Some(manifest) = &manifest_node.manifest {
                        let loc = ModuleLoc {
                            source: cur_dir_source.clone(),
                            module: manifest.module.clone(),
                        };
                        cur_module = Some(loc.clone());
                        cur_pkg_path = Some(loc.module.clone());
                        modules.insert(
                            loc.clone(),
                            workspace::Module {
                                source: loc.source,
                                manifest: manifest.clone(),
                                packages: Vec::new(),
                            },
                        );
                    }
                }
            }
            // If we are inside a module and have not hit an invalid package directory yet, add a package.
            if let Some(loc) = &cur_module {
                if cur_dir_source != loc.source && !util::valid_package_name(&node.name) {
                    cur_module = None;
                    cur_pkg_path = None;
                } else {
                    let pkg = workspace::Package {
                        source: cur_dir_source.clone(),
                        path: cur_pkg_path.clone().unwrap(),
                        files: node
                            .children
                            .iter()
                            .map(|key| &self.nodes[*key])
                            .filter(|node| {
                                !node.directory && util::valid_source_file_name(&node.name)
                            })
                            .map(|node| workspace::File {
                                source: cur_dir_source.join(&node.name),
                                content: Arc::from(String::from_utf8_lossy(&node.raw_content)),
                            })
                            .collect(),
                    };
                    modules.get_mut(&loc).unwrap().packages.push(pkg);
                }
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
        Just(VfsReferenceMachine::new(false)).boxed()
    }

    fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition> {
        let mut options: Vec<BoxedStrategy<Transition>> = Vec::new();

        options.push(add_node_strategy(state.directories.clone()));
        if !state.nodes.is_empty() {
            options.push(remove_node_strategy(state.nodes.keys().collect()));
        }
        if !state.files.is_empty() {
            options.push(update_node_strategy(state.files.clone()));
        }
        if !state.directories.is_empty() {
            options.push(mark_scan_root_strategy(state.directories.clone()));
        }
        options.push(scan_strategy());

        Union::new(options).boxed()
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
    dir: TempDir,
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
    fn new(async_vfs: bool) -> VfsMachine {
        let cleanup_stale_roots = false;
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
            (None, Some(state.clone()), Some(h))
        } else {
            let vfs = Vfs::new(cleanup_stale_roots, Duration::from_secs(0));
            (Some(vfs), None, None)
        };
        VfsMachine {
            dir: TempDir::new().expect("it must be possible to create a temporary directory"),
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
        let md = fs::metadata(&path).unwrap();
        if md.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
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
            if let Some(manifest) = manifest {
                let types::FennecVersion {
                    major,
                    minor,
                    patch,
                } = manifest.fennec;
                let mod_path = manifest.module;
                write!(
                    file,
                    "{PROJECT_NAME} = \"{major}.{minor}.{patch}\"\nmodule = \"{mod_path}\"\n"
                )
                .unwrap();
            } else {
                file.write_all(&[]).unwrap();
            }
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
                workspace::ModuleUpdateKind::ModuleAdded => {
                    let prev = self.modules.insert(
                        ModuleLoc {
                            source: m_upd.source.clone(),
                            module: m_upd.module,
                        },
                        workspace::Module {
                            source: m_upd.source,
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
                                        })
                                        .collect(),
                                })
                                .collect(),
                        },
                    );
                    assert!(prev.is_none());
                }
                workspace::ModuleUpdateKind::ModuleRemoved => {
                    let prev = self.modules.remove(&ModuleLoc {
                        source: m_upd.source,
                        module: m_upd.module,
                    });
                    assert!(prev.is_some());
                }
                workspace::ModuleUpdateKind::ModuleUpdated => {
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
                            workspace::PackageUpdateKind::PackageAdded => {
                                m.packages.push(workspace::Package {
                                    source: p_upd.source,
                                    path: p_upd.path,
                                    files: p_upd
                                        .files
                                        .into_iter()
                                        .map(|f| workspace::File {
                                            source: f.source,
                                            content: f.content.unwrap(),
                                        })
                                        .collect(),
                                });
                            }
                            workspace::PackageUpdateKind::PackageRemoved => {
                                let ix = m
                                    .packages
                                    .iter()
                                    .position(|p| p.source == p_upd.source && p.path == p_upd.path)
                                    .unwrap();
                                m.packages.swap_remove(ix);
                            }
                            workspace::PackageUpdateKind::PackageUpdated => {
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
        VfsMachine::new(ref_state.async_vfs)
    }

    fn apply(
        mut state: Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        transition: <Self::Reference as ReferenceStateMachine>::Transition,
    ) -> Self::SystemUnderTest {
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
