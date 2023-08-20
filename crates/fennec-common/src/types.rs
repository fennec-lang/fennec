// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;

pub struct RootPath(String); // UTF-8 + forward slash
pub struct FilePath(String); // UTF-8 + forward slash

impl RootPath {
    #[must_use]
    pub fn from_path(path: &Path) -> Option<RootPath> {
        // TODO: do we need to do any normalization here?
        // TODO: check what we get on windows
        Some(RootPath(path.to_str()?.to_string()))
    }
}

pub struct Change {}
