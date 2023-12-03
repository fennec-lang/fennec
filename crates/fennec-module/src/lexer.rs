// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use logos::Logos as _;

use crate::lexer_gen::LogosToken;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum TokenErrorKind {
    StringWithBackslashes,
    StringUnterminated,
    Other,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum TokenKind {
    Newline,
    Whitespace,
    KwModule,
    KwFennec,
    String,
    Comment,
    Error(TokenErrorKind),
}

pub(crate) struct Token {
    kind: TokenKind,
    len: u32,
}

const _: () = assert!(core::mem::size_of::<Token>() == 8); // just to be certain; it does not matter that much

pub(crate) struct Tokens {
    input: String,
    tokens: Vec<Token>,
}

fn to_token(token: Result<LogosToken, ()>, slice: &str) -> Token {
    let kind = match token {
        Ok(LogosToken::Newline) => TokenKind::Newline,
        Ok(LogosToken::Whitespace) => TokenKind::Whitespace,
        Ok(LogosToken::KeywordModule) => TokenKind::KwModule,
        Ok(LogosToken::KeywordFennec) => TokenKind::KwFennec,
        Ok(LogosToken::String) => TokenKind::String,
        Ok(LogosToken::ErrorStringWithBackslashes) => {
            TokenKind::Error(TokenErrorKind::StringWithBackslashes)
        }
        Ok(LogosToken::ErrorStringUnterminated) => {
            TokenKind::Error(TokenErrorKind::StringUnterminated)
        }
        Ok(LogosToken::Comment) => TokenKind::Comment,
        Err(()) => TokenKind::Error(TokenErrorKind::Other),
    };
    let len: u32 = slice
        .len()
        .try_into()
        .expect("token length must fit into 32 bits");
    Token { kind, len }
}

pub(crate) fn lex(input: String) -> Tokens {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = LogosToken::lexer(&input);
    while let Some(tok) = lexer.next() {
        let token = to_token(tok, lexer.slice());
        tokens.push(token);
    }
    Tokens { input, tokens }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    fn check_lex(input: &str, expected: &[(&str, TokenKind)]) {
        let tokens = lex(input.to_string());
        let mut sliced: Vec<(&'_ str, TokenKind)> = Vec::with_capacity(expected.len());
        let mut n = 0;
        for tok in tokens.tokens {
            let len = tok.len as usize;
            sliced.push((&tokens.input[n..n + len], tok.kind));
            n += len;
        }
        assert_eq!(expected, sliced, "lexing \"{input}\"");
    }

    #[test]
    fn lex_smoke() {
        use TokenErrorKind::*;
        use TokenKind::*;

        let cases: &[(&str, &[(&str, TokenKind)])] = &[
            ("", &[]),
            ("?", &[("?", Error(Other))]),
            (
                "fennec module",
                &[
                    ("fennec", KwFennec),
                    (" ", Whitespace),
                    ("module", KwModule),
                ],
            ),
            (
                "fennec \"hello\nmodule",
                &[
                    ("fennec", KwFennec),
                    (" ", Whitespace),
                    ("\"hello", Error(StringUnterminated)),
                    ("\n", Newline),
                    ("module", KwModule),
                ],
            ),
            (
                r#"
module "examples/hello"  // comment
fennec "0.1.0"
"#,
                &[
                    ("\n", Newline),
                    ("module", KwModule),
                    (" ", Whitespace),
                    ("\"examples/hello\"", String),
                    ("  ", Whitespace),
                    ("// comment", Comment),
                    ("\n", Newline),
                    ("fennec", KwFennec),
                    (" ", Whitespace),
                    ("\"0.1.0\"", String),
                    ("\n", Newline),
                ],
            ),
        ];

        for (input, expected) in cases {
            check_lex(input, expected);
        }
    }
}
