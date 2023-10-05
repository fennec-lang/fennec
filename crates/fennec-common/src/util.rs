// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::OsStr,
    path::{Component, Path, PathBuf},
};

use crate::{
    import_path::{PACKAGE_RE, RESERVED_WINDOWS_NAMES},
    SOURCE_EXTENSION,
};

#[must_use]
pub fn is_valid_utf8_visible(file_name: &OsStr) -> bool {
    file_name // windows: WTF-8, unix: byte slice, usually UTF-8
        .to_str() // maybe UTF-8
        .map_or(false, |s| !s.starts_with('.'))
}

#[must_use]
pub fn valid_package_name(file_name: &OsStr) -> bool {
    file_name.to_str().map_or(false, |s| {
        PACKAGE_RE.is_match(s)
            && !RESERVED_WINDOWS_NAMES
                .iter()
                .any(|r| s.eq_ignore_ascii_case(r))
    })
}

#[must_use]
pub fn valid_source_file_name(file_name: &OsStr) -> bool {
    file_name
        .to_str()
        .map_or(false, |s| match s.rsplit_once('.') {
            Some((name, SOURCE_EXTENSION)) => valid_package_name(name.as_ref()),
            _ => false,
        })
}

// Adapted from the normalize-path crate, MIT license.
#[must_use]
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek() {
        let buf = PathBuf::from(c.as_os_str());
        components.next();
        buf
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }

    ret
}
