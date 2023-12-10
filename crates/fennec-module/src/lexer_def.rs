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

    #[token("\r")]
    ErrorSingleCarriageReturn, // LSP considers it a valid EOL

    #[regex(r"[ \t]+")]
    Whitespace,

    #[token("module")]
    KeywordModule,

    #[token("fennec")]
    KeywordFennec,

    #[regex(r#""([^"\\\r\n])*""#, priority = 5)] // clashes with ErrorStringWithBackslashes
    String,

    #[regex(r#""([^"\r\n])*""#)]
    ErrorStringWithBackslashes,

    #[regex(r#""([^"\r\n])*"#)]
    ErrorStringUnterminated,

    #[regex(r"//[^\r\n]*")]
    Comment,

    #[regex(r"[0-9a-zA-Z]+")]
    ErrorIdentifier,

    // Exact regex, can be found at https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string.
    // We recognize a superset, for more resilient parsing.
    #[regex(r"\d+(\.\d+)?(\.\d+)?(-([0-9a-zA-Z-]+)(\.([0-9a-zA-Z-]+))*)?(\+([0-9a-zA-Z-]+(\.[0-9a-zA-Z-]+)*))?", priority = 2)] // clashes with ErrorIdentifier
    Version,
}
