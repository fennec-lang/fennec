// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::OsStr;

#[must_use]
pub fn is_valid_utf8_visible(file_name: &OsStr) -> bool {
    file_name // windows: WTF-8, unix: byte slice, usually UTF-8
        .to_str() // maybe UTF-8
        .map_or(false, |s| !s.starts_with('.'))
}
