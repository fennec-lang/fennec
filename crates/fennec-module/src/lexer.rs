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

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    TokNewline,
    TokWhitespace,
    TokKwModule,
    TokKwFennec,
    TokString,
    TokComment,
    TokVersion,
    TokError(TokenErrorKind),
    TokEof,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        self == TokenKind::TokWhitespace || self == TokenKind::TokComment
    }
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        let s = match *self {
            TokNewline => "LF",
            TokWhitespace => "WS",
            TokKwModule => "KW_MODULE",
            TokKwFennec => "KW_FENNEC",
            TokString => "STR",
            TokComment => "COMMENT",
            TokVersion => "VER",
            TokError(err) => match err {
                TokenErrorKind::Identifier => "ERR(IDENT)",
                TokenErrorKind::SingleCarriageReturn => "ERR(CR)",
                TokenErrorKind::StringWithBackslashes => "ERR(STR_ESC)",
                TokenErrorKind::StringUnterminated => "ERR(STR_TERM)",
                TokenErrorKind::Other => "ERR(OTHER)",
            },
            TokEof => "EOF",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        let s = match *self {
            TokNewline => "newline",
            TokWhitespace => "space or tab",
            TokKwModule => "\"module\" keyword",
            TokKwFennec => "\"fennec\" keyword",
            TokString => "string",
            TokComment => "comment",
            TokVersion => "semantic version",
            TokError(err) => match err {
                TokenErrorKind::Identifier => "identifier",
                TokenErrorKind::SingleCarriageReturn => "carriage return (\\r)",
                TokenErrorKind::StringWithBackslashes => "string literal (with backslashes)",
                TokenErrorKind::StringUnterminated => "string literal (unterminated)",
                TokenErrorKind::Other => "error",
            },
            TokEof => "eof",
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
            kind: TokenKind::TokEof,
            len: 0.into(),
        }
    }
}

const _: () = assert!(core::mem::size_of::<Token>() == 8); // just to be certain; it does not matter that much

fn to_token(token: Result<LogosToken, ()>, slice: &str) -> Token {
    let kind = match token {
        Ok(LogosToken::Newline) => TokenKind::TokNewline,
        Ok(LogosToken::ErrorSingleCarriageReturn) => {
            TokenKind::TokError(TokenErrorKind::SingleCarriageReturn)
        }
        Ok(LogosToken::Whitespace) => TokenKind::TokWhitespace,
        Ok(LogosToken::KeywordModule) => TokenKind::TokKwModule,
        Ok(LogosToken::KeywordFennec) => TokenKind::TokKwFennec,
        Ok(LogosToken::String) => TokenKind::TokString,
        Ok(LogosToken::ErrorStringWithBackslashes) => {
            TokenKind::TokError(TokenErrorKind::StringWithBackslashes)
        }
        Ok(LogosToken::ErrorStringUnterminated) => {
            TokenKind::TokError(TokenErrorKind::StringUnterminated)
        }
        Ok(LogosToken::Comment) => TokenKind::TokComment,
        Ok(LogosToken::ErrorIdentifier) => TokenKind::TokError(TokenErrorKind::Identifier),
        Ok(LogosToken::Version) => TokenKind::TokVersion,
        Err(()) => TokenKind::TokError(TokenErrorKind::Other),
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
