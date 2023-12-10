// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use logos::Logos as _;

use crate::lexer_gen::LogosToken;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TokenErrorKind {
    SingleCarriageReturn,
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
    LitString,
    Comment,
    LitNumber,
    Ident,
    Dot,
    Dash,
    Plus,
    Error(TokenErrorKind),
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            TokenKind::Newline => "newline",
            TokenKind::Whitespace => "space or tab",
            TokenKind::KwModule => "\"module\" keyword",
            TokenKind::KwFennec => "\"fennec\" keyword",
            TokenKind::LitString => "string",
            TokenKind::Comment => "comment",
            TokenKind::LitNumber => "number",
            TokenKind::Ident => "identifier",
            TokenKind::Dot => ".",
            TokenKind::Dash => "-",
            TokenKind::Plus => "+",
            TokenKind::Error(err) => match err {
                TokenErrorKind::SingleCarriageReturn => "carriage return (\\r)",
                TokenErrorKind::StringWithBackslashes => "string literal (with backslashes)",
                TokenErrorKind::StringUnterminated => "string literal (unterminated)",
                TokenErrorKind::Other => "error",
            },
            TokenKind::Eof => "eof",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

const _: () = assert!(core::mem::size_of::<Token>() == 8); // just to be certain; it does not matter that much

fn to_token(token: Result<LogosToken, ()>, slice: &str) -> Token {
    let kind = match token {
        Ok(LogosToken::Newline) => TokenKind::Newline,
        Ok(LogosToken::ErrorSingleCarriageReturn) => {
            TokenKind::Error(TokenErrorKind::SingleCarriageReturn)
        }
        Ok(LogosToken::Whitespace) => TokenKind::Whitespace,
        Ok(LogosToken::KeywordModule) => TokenKind::KwModule,
        Ok(LogosToken::KeywordFennec) => TokenKind::KwFennec,
        Ok(LogosToken::String) => TokenKind::LitString,
        Ok(LogosToken::ErrorStringWithBackslashes) => {
            TokenKind::Error(TokenErrorKind::StringWithBackslashes)
        }
        Ok(LogosToken::ErrorStringUnterminated) => {
            TokenKind::Error(TokenErrorKind::StringUnterminated)
        }
        Ok(LogosToken::Comment) => TokenKind::Comment,
        Ok(LogosToken::Number) => TokenKind::LitNumber,
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

pub(crate) fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = LogosToken::lexer(input);
    while let Some(tok) = lexer.next() {
        let token = to_token(tok, lexer.slice());
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    struct Tokens {
        input: String,
        tokens: Vec<super::Token>,
    }

    impl std::fmt::Debug for Tokens {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut exploded = Vec::with_capacity(self.tokens.len());
            let mut n = 0;
            for tok in &self.tokens {
                let len = tok.len as usize;
                exploded.push(((&self.input[n..n + len]).to_owned(), tok.kind));
                n += len;
            }
            exploded.fmt(f)
        }
    }

    fn lex(input: &str) -> Tokens {
        let tokens = super::lex(input);
        Tokens {
            input: input.to_owned(),
            tokens,
        }
    }

    #[test]
    fn lex_empty() {
        insta::assert_debug_snapshot!(lex(""));
    }

    #[test]
    fn lex_trivial_err() {
        insta::assert_debug_snapshot!(lex("?"));
    }

    #[test]
    fn lex_unterminated() {
        insta::assert_debug_snapshot!(lex("fennec \"hello\nmodule"));
    }

    #[test]
    fn lex_normal() {
        insta::assert_debug_snapshot!(lex(r#"
module "examples/hello"  // comment
fennec 0.1.0
"#));
    }
}
