// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{path::PathBuf, sync::Arc};

use crate::types;

pub struct File {
    pub source: PathBuf,
    pub content: Arc<str>, // empty in case of deleted file
}

pub struct PackageUpdate {
    pub source: PathBuf,
    pub path: types::ImportPath,
    pub files: Vec<File>,
    pub update: PackageUpdateKind,
}

pub enum PackageUpdateKind {
    PackageAdded,
    PackageRemoved,
    PackageUpdated,
}

pub struct ModuleManifest {
    pub module: types::ImportPath,
    pub fennec: types::FennecVersion,
}

pub struct ModuleUpdate {
    pub source: PathBuf,
    pub module: types::ImportPath,        // same as manifest.module
    pub manifest: Option<ModuleManifest>, // empty in case of no changes to the manifest
    pub packages: Vec<PackageUpdate>,
    pub update: ModuleUpdateKind,
}

pub enum ModuleUpdateKind {
    ModuleAdded,
    ModuleRemoved,
    ModuleUpdated,
}
