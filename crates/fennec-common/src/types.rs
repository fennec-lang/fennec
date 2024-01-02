// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use once_cell::sync::Lazy;
use std::sync::Arc;

pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;
pub type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;

pub use crate::import_path::*;
pub use crate::sync_state::*;

pub type Text = Arc<str>;
pub use line_index::LineCol;
pub use text_size::{TextRange, TextSize};

pub static EMPTY_TEXT: Lazy<Text> = Lazy::new(|| Text::from(""));

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FennecVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}
