// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;

pub struct AbsolutePath(String); // UTF-8 + forward slash
pub struct RelativePath(String); // UTF-8 + forward slash

impl AbsolutePath {
    #[must_use]
    pub fn from_path(path: &Path) -> Option<AbsolutePath> {
        let path = path.to_str()?;
        let path = if cfg!(windows) {
            path.replace('\\', "/")
        } else {
            path.to_string()
        };
        Some(AbsolutePath(path))
    }
}

pub struct ChangeBuffer {
    // TODO: atomic? flag for core to check
}

impl ChangeBuffer {
    #[must_use]
    pub fn new() -> ChangeBuffer {
        ChangeBuffer {}
    }
}

impl Default for ChangeBuffer {
    fn default() -> Self {
        Self::new()
    }
}
