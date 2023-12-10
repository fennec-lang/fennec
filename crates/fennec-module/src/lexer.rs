// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::Range;

use fennec_common::types::TextSize;
use logos::Logos as _;

use crate::lexer_gen::LogosToken;

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
        use TokenKind as T;
        let s = match *self {
            T::Newline => "LF",
            T::Whitespace => "WS",
            T::KwModule => "KW_MODULE",
            T::KwFennec => "KW_FENNEC",
            T::String => "STR",
            T::Comment => "COMMENT",
            T::Version => "VER",
            T::Error(err) => return write!(f, "ERR({err:?})"),
            T::Eof => "EOF",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind as T;
        let s = match *self {
            T::Newline => "newline",
            T::Whitespace => "space or tab",
            T::KwModule => "\"module\" keyword",
            T::KwFennec => "\"fennec\" keyword",
            T::String => "string",
            T::Comment => "comment",
            T::Version => "semantic version",
            T::Error(err) => return write!(f, "error ({err})"),
            T::Eof => "eof",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenErrorKind {
    Identifier,
    SingleCarriageReturn,
    StringWithBackslashes,
    StringUnterminated,
    Other,
}

impl std::fmt::Debug for TokenErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenErrorKind as TE;
        let s = match self {
            TE::Identifier => "IDENT",
            TE::SingleCarriageReturn => "CR",
            TE::StringWithBackslashes => "STR_ESC",
            TE::StringUnterminated => "STR_TERM",
            TE::Other => "OTHER",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for TokenErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenErrorKind as TE;
        let s = match self {
            TE::Identifier => "identifier",
            TE::SingleCarriageReturn => "carriage return (\\r)",
            TE::StringWithBackslashes => "string literal (with backslashes)",
            TE::StringUnterminated => "string literal (unterminated)",
            TE::Other => "other",
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

fn to_token(token: Result<LogosToken, ()>, span: Range<usize>) -> Token {
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
    let len: TextSize = (span.end - span.start)
        .try_into()
        .expect("token length must fit into 32 bits");
    Token { kind, len }
}

pub(crate) fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = LogosToken::lexer(input);
    while let Some(tok) = lexer.next() {
        let token = to_token(tok, lexer.span());
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use fennec_common::types::{TextRange, TextSize};

    struct Tokens {
        input: String,
        tokens: Vec<super::Token>,
    }

    impl std::fmt::Debug for Tokens {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut exploded = Vec::with_capacity(self.tokens.len());
            let mut start = TextSize::new(0);
            for tok in &self.tokens {
                let loc = TextRange::at(start, tok.len);
                exploded.push(((&self.input[loc]).to_owned(), tok.kind));
                start += tok.len;
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
