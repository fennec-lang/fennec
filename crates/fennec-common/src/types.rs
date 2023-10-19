// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;
pub type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;

pub use crate::import_path::*;
pub use crate::sync_state::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FennecVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}
