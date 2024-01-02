// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    ffi::OsStr,
    path::{Component, Path, PathBuf},
};

use crate::{import_path::PACKAGE_RE, SOURCE_EXTENSION};

// https://docs.microsoft.com/en-us/windows/desktop/fileio/naming-a-file
const RESERVED_WINDOWS_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", //
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", //
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

static VERSION_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^v[0-9.]+$").expect("invalid version regex literal"));

#[must_use]
pub fn is_reserved_windows_filename(file_name: &str) -> bool {
    RESERVED_WINDOWS_NAMES
        .iter()
        .any(|r| file_name.eq_ignore_ascii_case(r))
}

#[must_use]
pub fn is_version_like(file_name: &str) -> bool {
    VERSION_RE.is_match(file_name)
}

#[must_use]
pub fn is_valid_utf8_visible(file_name: &OsStr) -> bool {
    file_name // windows: WTF-8, unix: byte slice, usually UTF-8
        .to_str() // maybe UTF-8
        .map_or(false, |s| !s.starts_with('.'))
}

#[must_use]
fn valid_file_name(file_name: &str) -> bool {
    PACKAGE_RE.is_match(file_name) && !is_reserved_windows_filename(file_name)
}

#[must_use]
pub fn valid_package_name(file_name: &str) -> bool {
    valid_file_name(file_name) && !is_version_like(file_name)
}

#[must_use]
pub fn valid_source_extension(file_name: &str) -> bool {
    matches!(file_name.rsplit_once('.'), Some((_, SOURCE_EXTENSION)))
}

#[must_use]
pub fn valid_source_file_name(file_name: &str) -> bool {
    match file_name.rsplit_once('.') {
        Some((name, SOURCE_EXTENSION)) => valid_file_name(name),
        _ => false,
    }
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

#[must_use]
pub fn has_prefix(path: &Path, prefix: &Path) -> bool {
    path.strip_prefix(prefix).is_ok()
}

#[must_use]
pub fn has_strict_prefix(path: &Path, prefix: &Path) -> bool {
    path.strip_prefix(prefix)
        .map_or(false, |p| !p.as_os_str().is_empty())
}
