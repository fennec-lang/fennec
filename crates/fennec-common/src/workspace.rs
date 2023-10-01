// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::types;

pub struct ModuleManifest {
    pub module: types::ImportPath,
    pub fennec: String,
}

pub struct Package {
    pub path: types::ImportPath,
    pub files: types::HashMap<String, File>,
}

pub struct File {
    pub name: String,
    pub content: Arc<str>, // empty in case of deleted file
}

pub struct ModuleUpdate {
    pub path: types::ImportPath,
    pub data: ModuleUpdateData,
}

pub enum ModuleUpdateData {
    ModuleAdded(ModuleManifest),
    ModuleRemoved,
    ModuleManifestUpdated(Vec<ManifestUpdate>),
    ModulePackagesUpdated(Vec<PackageUpdate>),
}

pub enum ManifestUpdate {
    FennecVersionUpdated(String),
}

pub struct PackageUpdate {
    pub path: types::ImportPath,
    pub data: PackageUpdateData,
}

pub enum PackageUpdateData {
    PackageAdded(Package),
    PackageRemoved,
    PackageUpdated(Vec<File>),
}
