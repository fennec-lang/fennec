// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{path::PathBuf, sync::Arc};

use crate::types;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct File {
    pub source: PathBuf,
    pub content: Arc<str>,
    pub detached: bool, // invalid file name
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Package {
    pub source: PathBuf,
    pub path: Option<types::ImportPath>, // empty in case of detached package
    pub files: Vec<File>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Module {
    pub source: PathBuf,
    pub manifest: Option<ModuleManifest>, // empty in case of detached module
    pub packages: Vec<Package>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FileUpdate {
    pub source: PathBuf,
    pub content: Option<Arc<str>>, // empty in case file was removed
    pub detached: bool,            // invalid file name
}

#[derive(Clone, Debug)]
pub struct PackageUpdate {
    pub source: PathBuf,
    pub path: Option<types::ImportPath>, // empty in case of detached package
    pub files: Vec<FileUpdate>,
    pub update: PackageUpdateKind,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackageUpdateKind {
    PackageAdded,
    PackageRemoved,
    PackageUpdated,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ModuleManifest {
    pub module: types::ImportPath,
    pub fennec: types::FennecVersion,
}

#[derive(Clone, Debug)]
pub enum ModuleManifestUpdate {
    Unknown,
    Updated(Option<ModuleManifest>),
}

#[derive(Clone, Debug)]
pub struct ModuleUpdate {
    pub source: PathBuf,
    pub module: Option<types::ImportPath>, // empty in case of detached module, otherwise same as manifest.module
    pub manifest: ModuleManifestUpdate,
    pub packages: Vec<PackageUpdate>, // empty in case of no changes to the packages or module was removed
    pub update: ModuleUpdateKind,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ModuleUpdateKind {
    ModuleAdded,
    ModuleRemoved,
    ModuleUpdated,
}
