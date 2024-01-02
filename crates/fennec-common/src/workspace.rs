// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

use crate::types;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FileUpdate {
    pub source: PathBuf,
    pub content: Option<types::Text>, // empty in case file was removed
    pub detached: bool,               // invalid file name
}

#[derive(Clone, Debug)]
pub struct PackageUpdate {
    pub source: PathBuf,
    pub path: Option<types::ImportPath>, // empty in case of detached package
    pub files: Vec<FileUpdate>,
    pub update: UpdateKind,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UpdateKind {
    Added,
    Removed,
    Updated,
}

#[derive(Clone, Debug)]
pub struct ModuleUpdate {
    pub source: PathBuf,
    pub module: Option<types::ImportPath>, // empty in case of detached module, otherwise same as manifest.module
    pub manifest: Option<types::Text>,
    pub packages: Vec<PackageUpdate>, // empty in case of no changes to the packages or module was removed
    pub update: UpdateKind,
}
