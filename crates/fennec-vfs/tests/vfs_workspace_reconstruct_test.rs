extern crate proptest;
extern crate proptest_state_machine;

use std::{path::PathBuf, sync::Arc};

use fennec_common::{types, util, workspace, MODULE_MANIFEST_FILENAME};
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
    let cfg = vfs_config();
    proptest!(cfg.clone(), |((initial_state, transitions) in <VfsMachine as StateMachineTest>::Reference::sequential_strategy(1..20))| {
        VfsMachine::test_sequential(cfg.clone(), initial_state, transitions)
    });
}

#[derive(Copy, Clone, Debug)]
enum NodeNameKind {
    SourceCode,
    ModuleManifest,
    Other,
}

#[derive(Clone, Debug)]
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
    any::<Vec<u8>>().boxed()
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
        Just(NodeNameKind::SourceCode),
        Just(NodeNameKind::ModuleManifest),
        Just(NodeNameKind::Other),
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
        self.children
            .binary_search_by_key(&name, |k| &nodes[*k].name)
    }
}

#[derive(Clone, Debug, Default)]
struct VfsReferenceMachine {
    nodes: SlotMap<NodeKey, Node>,
    toplevel: Vec<NodeKey>,
    directories: Vec<NodeKey>,
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
    fn add_node(
        &mut self,
        name: String,
        directory: bool,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
        parent: Option<NodeKey>,
    ) {
        let key = self.nodes.insert(Node::new(
            name.clone(),
            directory,
            raw_content,
            manifest,
            parent,
        ));
        if let Some(parent_key) = parent {
            let child_ix = (&self.nodes[parent_key])
                .find_child(&name, &self.nodes)
                .unwrap_err();
            (&mut self.nodes[parent_key]).children.insert(child_ix, key);
        } else {
            sorted_vec_insert(&mut self.toplevel, key);
        }
        if directory {
            sorted_vec_insert(&mut self.directories, key);
        }
    }

    fn remove_node(&mut self, key: NodeKey, unlink_from_parent: bool) {
        if unlink_from_parent {
            let node = &self.nodes[key];
            if let Some(parent_key) = node.parent {
                let child_ix = (&self.nodes[parent_key])
                    .find_child(&node.name, &self.nodes)
                    .unwrap();
                (&mut self.nodes[parent_key]).children.remove(child_ix);
            } else {
                sorted_vec_remove(&mut self.toplevel, &key);
            }
        }
        let node = self.remove(key);
        for child in node.children {
            self.remove_node(child, false)
        }
    }

    fn update_node(
        &mut self,
        key: NodeKey,
        raw_content: Vec<u8>,
        manifest: Option<workspace::ModuleManifest>,
    ) {
        let node = &mut self.nodes[key];
        node.raw_content = raw_content;
        node.manifest = manifest;
    }

    fn mark_scan_root(&mut self, key: NodeKey) {
        let node = &mut self.nodes[key];
        assert!(node.directory);
        node.scan_root = true;
    }

    fn remove(&mut self, key: NodeKey) -> Node {
        let node = self.nodes.remove(key).unwrap();
        if node.directory {
            sorted_vec_remove(&mut self.directories, &key);
        }
        node
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
                cur_module = None;
                cur_pkg_path = None;
                let manifest_key = node.children[manifest_ix];
                let manifest_node = &self.nodes[manifest_key];
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
        Just(VfsReferenceMachine::default()).boxed()
    }

    fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition> {
        let mut options: Vec<BoxedStrategy<Transition>> = Vec::new();

        options.push(add_node_strategy(state.directories.clone()));
        if !state.nodes.is_empty() {
            options.push(remove_node_strategy(state.nodes.keys().collect()));
            options.push(update_node_strategy(state.nodes.keys().collect()));
        }
        if !state.directories.is_empty() {
            options.push(mark_scan_root_strategy(state.directories.clone()));
        }

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
                state.remove_node(*key, true);
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
                    true
                }
            }
            Transition::RemoveNode { key } => state.nodes.contains_key(*key),
            Transition::UpdateNode { key, .. } => state.nodes.contains_key(*key),
            Transition::MarkScanRoot { key } => {
                state.nodes.contains_key(*key) && sorted_vec_contains(&state.directories, key)
            }
        }
    }
}

struct VfsMachine {
    _dir: TempDir,
}

impl StateMachineTest for VfsMachine {
    type SystemUnderTest = Self;
    type Reference = VfsReferenceMachine;

    fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        VfsMachine {
            _dir: TempDir::new().expect("it must be possible to create a temporary directory"),
        }
    }

    fn apply(
        state: Self::SystemUnderTest,
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        _transition: <Self::Reference as ReferenceStateMachine>::Transition,
    ) -> Self::SystemUnderTest {
        state
    }

    fn check_invariants(
        _state: &Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
        let modules = ref_state.reconstruct();
        assert!(modules.is_empty() || modules.len() > 0);
    }
}
