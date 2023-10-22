extern crate proptest;
extern crate proptest_state_machine;

use proptest::prelude::*;
use proptest_state_machine::{ReferenceStateMachine, StateMachineTest};
use tempfile::TempDir;

fn vfs_config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: None, // does not help with https://github.com/proptest-rs/proptest/issues/233 for some reason
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
enum Transition {}

struct VfsTestMachine;

impl ReferenceStateMachine for VfsTestMachine {
    type State = ();
    type Transition = Transition;

    fn init_state() -> BoxedStrategy<Self::State> {
        todo!()
    }

    fn transitions(_state: &Self::State) -> BoxedStrategy<Self::Transition> {
        todo!()
    }

    fn apply(mut _state: Self::State, _transition: &Self::Transition) -> Self::State {
        todo!()
    }

    fn preconditions(_state: &Self::State, _transition: &Self::Transition) -> bool {
        todo!()
    }
}

struct VfsTest {
    dir: TempDir,
}

impl StateMachineTest for VfsTest {
    type SystemUnderTest = Self;
    type Reference = VfsTestMachine;

    fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        VfsTest {
            dir: TempDir::new().expect("it must be possible to create a temporary directory"),
        }
    }

    fn apply(
        _state: Self::SystemUnderTest,
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        _transition: <Self::Reference as ReferenceStateMachine>::Transition,
    ) -> Self::SystemUnderTest {
        todo!()
    }

    fn check_invariants(
        _state: &Self::SystemUnderTest,
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
        todo!()
    }
}
