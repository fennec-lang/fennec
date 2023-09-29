// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{path::PathBuf, sync::Arc};

use crate::types;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ContentVersion(u32);

// TODO: fs-path-based view from the VFS vs import-path-based from the core

pub struct Workspace {
    _version: ContentVersion,
    _modules: types::HashMap<PathBuf, Module>,
}

pub struct Module {
    _version: ContentVersion,
    _packages: types::HashMap<types::ImportPath, Package>,
}

pub struct Package {
    _version: ContentVersion,
    _files: types::HashMap<String, File>,
}

pub struct File {
    _version: ContentVersion,
    _content: Arc<str>,
}
