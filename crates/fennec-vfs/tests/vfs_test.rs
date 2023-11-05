extern crate proptest;
extern crate proptest_state_machine;

use fennec_common::{util, workspace};
use proptest::{prelude::*, sample, strategy};
use proptest_state_machine::{ReferenceStateMachine, StateMachineTest};
use slotmap::SlotMap;
use tempfile::TempDir;

fn vfs_config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: None, // unfortunately overriden inside the proptest! macro, leading to warnings like https://github.com/proptest-rs/proptest/issues/233
        ..ProptestConfig::default()
    }
}

#[test]
fn vfs_test() {
    let cfg = vfs_config();
    proptest!(cfg.clone(), |((initial_state, transitions) in <VfsMachine as StateMachineTest>::Reference::sequential_strategy(1..20))| {
        VfsMachine::test_sequential(cfg.clone(), initial_state, transitions)
    });
}

#[derive(Clone, Debug)]
enum Transition {
    AddNode {
        name: String,
        directory: bool,
        file_content: Vec<u8>,
        parent: Option<NodeKey>,
    },
    RemoveNode {
        key: NodeKey,
    },
}

fn node_name_strategy(valid_ident: bool) -> BoxedStrategy<String> {
    let expr = if valid_ident {
        "[a-z][a-z0-9_]*"
    } else {
        "[A-Z][A-Z0-9_]*"
    };
    expr.prop_filter("filename must not be reserved on windows", |name| {
        !util::is_reserved_windows_filename(name)
    })
    .boxed()
}

fn add_node_strategy(parents: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let name_strategy = any::<bool>().prop_flat_map(node_name_strategy);
    let directory_strategy = any::<bool>();
    let file_content_strategy = any::<Vec<u8>>();
    let maybe_parent_strategy = any::<bool>()
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
        file_content_strategy,
        maybe_parent_strategy,
    )
        .prop_map(
            |(name, directory, file_content, parent)| Transition::AddNode {
                name,
                directory,
                file_content,
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

slotmap::new_key_type! { struct NodeKey; }

#[derive(Clone, Debug)]
struct Node {
    name: String,
    directory: bool,
    _file_content: Vec<u8>,
    parent: Option<NodeKey>,
    children: Vec<NodeKey>,
}

impl Node {
    fn new(name: String, directory: bool, file_content: Vec<u8>, parent: Option<NodeKey>) -> Node {
        Node {
            name,
            directory,
            _file_content: file_content,
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

fn sorted_vec_insert<T: Ord>(v: &mut Vec<T>, elem: T) {
    let ix = v.binary_search(&elem).unwrap_err();
    v.insert(ix, elem);
}

fn sorted_vec_remove<T: Ord>(v: &mut Vec<T>, elem: &T) {
    let ix = v.binary_search(elem).unwrap();
    v.remove(ix);
}

impl VfsReferenceMachine {
    fn add_node(
        &mut self,
        name: String,
        directory: bool,
        file_content: Vec<u8>,
        parent: Option<NodeKey>,
    ) {
        let key = self
            .nodes
            .insert(Node::new(name.clone(), directory, file_content, parent));
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

    fn remove(&mut self, key: NodeKey) -> Node {
        let node = self.nodes.remove(key).unwrap();
        if node.directory {
            sorted_vec_remove(&mut self.directories, &key);
        }
        node
    }

    fn _reconstruct(&self) -> Vec<workspace::Module> {
        todo!()
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
            let nodes_vec: Vec<NodeKey> = state.nodes.keys().collect();
            options.push(remove_node_strategy(nodes_vec));
        }

        strategy::Union::new(options).boxed()
    }

    fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            Transition::AddNode {
                name,
                directory,
                file_content,
                parent,
            } => {
                state.add_node(name.clone(), *directory, file_content.clone(), *parent);
            }
            Transition::RemoveNode { key } => {
                state.remove_node(*key, true);
            }
        }
        state
    }

    fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool {
        match transition {
            Transition::AddNode {
                name,
                directory: _,
                file_content: _,
                parent,
            } => {
                if let Some(parent_key) = parent {
                    state.directories.contains(parent_key)
                        && (&state.nodes[*parent_key])
                            .find_child(&name, &state.nodes)
                            .is_err()
                } else {
                    true
                }
            }
            Transition::RemoveNode { key } => state.nodes.contains_key(*key),
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
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
    }
}
