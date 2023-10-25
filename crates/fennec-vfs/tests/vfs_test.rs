extern crate proptest;
extern crate proptest_state_machine;

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
    proptest!(cfg.clone(), |((initial_state, transitions) in <VfsTest as StateMachineTest>::Reference::sequential_strategy(1..20))| {
        VfsTest::test_sequential(cfg.clone(), initial_state, transitions)
    });
}

#[derive(Clone, Debug)]
enum Transition {
    AddNode {
        name: String,
        directory: bool,
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
    expr.boxed()
}

fn add_node_strategy(parents: Vec<NodeKey>) -> BoxedStrategy<Transition> {
    let name_strategy = any::<bool>().prop_flat_map(node_name_strategy);
    let directory_strategy = any::<bool>();
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

    (name_strategy, directory_strategy, maybe_parent_strategy)
        .prop_map(|(name, directory, parent)| Transition::AddNode {
            name,
            directory,
            parent,
        })
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
    parent: Option<NodeKey>,
    children: Vec<NodeKey>,
}

impl Node {
    fn new(name: String, directory: bool, parent: Option<NodeKey>) -> Node {
        Node {
            name,
            directory,
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
struct VfsTestMachine {
    nodes: SlotMap<NodeKey, Node>,
    directories: Vec<NodeKey>,
}

impl VfsTestMachine {
    fn add_node(&mut self, name: String, directory: bool, parent: Option<NodeKey>) {
        let key = if let Some(parent_key) = parent {
            let parent_node = &self.nodes[parent_key];
            let ins = parent_node.find_child(&name, &self.nodes);
            match ins {
                Ok(_) => {
                    return; // no-op, we tried to add a child that already exists
                }
                Err(ix) => {
                    let key = self.nodes.insert(Node::new(name, directory, parent));
                    let parent_node = &mut self.nodes[parent_key];
                    parent_node.children.insert(ix, key);
                    key
                }
            }
        } else {
            self.nodes.insert(Node::new(name, directory, parent))
        };
        if directory {
            let dir_ix = self.directories.binary_search(&key).unwrap_err();
            self.directories.insert(dir_ix, key);
        }
    }

    fn remove_node(&mut self, key: NodeKey, unlink_from_parent: bool) {
        let node = self.eject(key, unlink_from_parent);
        for child in node.children {
            self.remove_node(child, false)
        }
    }

    fn eject(&mut self, key: NodeKey, unlink_from_parent: bool) -> Node {
        let node = self.nodes.remove(key).unwrap();
        if node.directory {
            let dir_ix = self.directories.binary_search(&key).unwrap();
            self.directories.remove(dir_ix);
        }
        if unlink_from_parent {
            if let Some(parent_key) = node.parent {
                let parent_node = &self.nodes[parent_key];
                let child_ix = parent_node.find_child(&node.name, &self.nodes).unwrap();
                let parent_node = &mut self.nodes[parent_key];
                parent_node.children.remove(child_ix);
            }
        }
        node
    }
}

impl ReferenceStateMachine for VfsTestMachine {
    type State = Self;
    type Transition = Transition;

    fn init_state() -> BoxedStrategy<Self::State> {
        Just(VfsTestMachine::default()).boxed()
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
                parent,
            } => {
                state.add_node(name.clone(), *directory, *parent);
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
                name: _,
                directory: _,
                parent,
            } => {
                if let Some(parent_key) = parent {
                    state.directories.contains(parent_key)
                } else {
                    true
                }
            }
            Transition::RemoveNode { key } => state.nodes.contains_key(*key),
        }
    }
}

struct VfsTest {
    _dir: TempDir,
}

impl StateMachineTest for VfsTest {
    type SystemUnderTest = Self;
    type Reference = VfsTestMachine;

    fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        VfsTest {
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
