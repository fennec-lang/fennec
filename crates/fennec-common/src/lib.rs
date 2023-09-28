// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]

pub mod module;
pub mod types;
pub mod util;

pub const PROJECT_NAME: &str = "fennec";
pub const RELEASE_VERSION: &str = "0.1.6";
pub const MODULE_MANIFEST_FILENAME: &str = "fennec.toml";
pub const SOURCE_EXTENSION: &str = "fn";
