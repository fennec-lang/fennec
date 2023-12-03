// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use logos::Logos as _;

use crate::lexer_gen::LogosToken;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TokenErrorKind {
    StringWithBackslashes,
    StringUnterminated,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TokenKind {
    Newline,
    Whitespace,
    KwModule,
    KwFennec,
    String,
    Comment,
    Number,
    Ident,
    Dot,
    Dash,
    Plus,
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

impl std::fmt::Debug for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut exploded: Vec<(String, TokenKind)> = Vec::with_capacity(self.tokens.len());
        let mut n = 0;
        for tok in &self.tokens {
            let len = tok.len as usize;
            exploded.push(((&self.input[n..n + len]).to_owned(), tok.kind));
            n += len;
        }
        exploded.fmt(f)
    }
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
        Ok(LogosToken::Number) => TokenKind::Number,
        Ok(LogosToken::Ident) => TokenKind::Ident,
        Ok(LogosToken::Dot) => TokenKind::Dot,
        Ok(LogosToken::Dash) => TokenKind::Dash,
        Ok(LogosToken::Plus) => TokenKind::Plus,
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

    #[test]
    fn lex_empty() {
        insta::assert_debug_snapshot!(lex("".to_owned()));
    }

    #[test]
    fn lex_trivial_err() {
        insta::assert_debug_snapshot!(lex("?".to_owned()));
    }

    #[test]
    fn lex_unterminated() {
        insta::assert_debug_snapshot!(lex("fennec \"hello\nmodule".to_owned()));
    }

    #[test]
    fn lex_normal() {
        insta::assert_debug_snapshot!(lex(r#"
module "examples/hello"  // comment
fennec 0.1.0
"#
        .to_owned()));
    }
}
