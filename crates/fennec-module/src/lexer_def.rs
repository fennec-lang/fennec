// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum LogosToken {
    #[token("\n")]
    #[token("\r\n")]
    Newline,

    #[regex(r"[ \t]+")]
    Whitespace,

    #[token("module")]
    Module,

    #[token("fennec")]
    Fennec,

    #[regex(r#""([^"\\\r\n])*""#)]
    String,
}
