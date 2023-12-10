// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::types::TextSize;
use logos::Logos as _;

use crate::lexer_gen::LogosToken;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TokenErrorKind {
    Identifier,
    SingleCarriageReturn,
    StringWithBackslashes,
    StringUnterminated,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Newline,
    Whitespace,
    KwModule,
    KwFennec,
    String,
    Comment,
    Version,
    Error(TokenErrorKind),
    Eof,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            TokenKind::Whitespace | TokenKind::Comment | TokenKind::Error(_)
        )
    }
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        let s = match *self {
            Newline => "LF",
            Whitespace => "WS",
            KwModule => "KW_MODULE",
            KwFennec => "KW_FENNEC",
            String => "STR",
            Comment => "COMMENT",
            Version => "VER",
            Error(err) => match err {
                TokenErrorKind::Identifier => "ERR(IDENT)",
                TokenErrorKind::SingleCarriageReturn => "ERR(CR)",
                TokenErrorKind::StringWithBackslashes => "ERR(STR_ESC)",
                TokenErrorKind::StringUnterminated => "ERR(STR_TERM)",
                TokenErrorKind::Other => "ERR(OTHER)",
            },
            Eof => "EOF",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        let s = match *self {
            Newline => "newline",
            Whitespace => "space or tab",
            KwModule => "\"module\" keyword",
            KwFennec => "\"fennec\" keyword",
            String => "string",
            Comment => "comment",
            Version => "semantic version",
            Error(err) => match err {
                TokenErrorKind::Identifier => "identifier",
                TokenErrorKind::SingleCarriageReturn => "carriage return (\\r)",
                TokenErrorKind::StringWithBackslashes => "string literal (with backslashes)",
                TokenErrorKind::StringUnterminated => "string literal (unterminated)",
                TokenErrorKind::Other => "error",
            },
            Eof => "eof",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub len: TextSize,
}

impl Token {
    pub fn eof() -> Token {
        Token {
            kind: TokenKind::Eof,
            len: 0.into(),
        }
    }
}

const _: () = assert!(core::mem::size_of::<Token>() == 8); // just to be certain; it does not matter that much

fn to_token(token: Result<LogosToken, ()>, slice: &str) -> Token {
    use LogosToken as LT;
    use TokenKind as T;
    let kind = match token {
        Ok(LT::Newline) => T::Newline,
        Ok(LT::ErrorSingleCarriageReturn) => T::Error(TokenErrorKind::SingleCarriageReturn),
        Ok(LT::Whitespace) => T::Whitespace,
        Ok(LT::KeywordModule) => T::KwModule,
        Ok(LT::KeywordFennec) => T::KwFennec,
        Ok(LT::String) => T::String,
        Ok(LT::ErrorStringWithBackslashes) => T::Error(TokenErrorKind::StringWithBackslashes),
        Ok(LT::ErrorStringUnterminated) => T::Error(TokenErrorKind::StringUnterminated),
        Ok(LT::Comment) => T::Comment,
        Ok(LT::ErrorIdentifier) => T::Error(TokenErrorKind::Identifier),
        Ok(LT::Version) => T::Version,
        Err(()) => T::Error(TokenErrorKind::Other),
    };
    let len: TextSize = slice
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
                let len: usize = tok.len.into();
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
