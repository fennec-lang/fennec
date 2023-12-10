// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cell::Cell;

use crate::lexer::{lex, Token, TokenKind};

// Parser design heavily based on https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html.
pub(crate) fn parse(input: &str) -> Tree {
    let tokens = lex(input);
    let mut p = Parser::new(tokens);
    manifest(&mut p);
    p.build_tree()
}

pub(crate) struct Tree {
    kind: TreeKind,
    children: Vec<Node>,
}

#[derive(PartialEq, Eq, Debug)]
enum TreeKind {
    Unknown,
    Error,
    Manifest,
}

enum Node {
    Token(Token),
    Tree(Tree),
}

struct Parser {
    tokens: Vec<Token>,
    token_pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

#[derive(PartialEq, Eq, Debug)]
enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
    Error(String),
}

const FUEL_RESET: u32 = 64;

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        let tok_len = tokens.len();
        Parser {
            tokens,
            token_pos: 0,
            fuel: Cell::new(FUEL_RESET),
            events: Vec::with_capacity(tok_len * 2),
        }
    }

    fn build_tree(self) -> Tree {
        todo!()
    }

    fn open(&mut self) -> usize {
        let open_ix = self.events.len();
        self.events.push(Event::Open {
            kind: TreeKind::Unknown,
        });
        open_ix
    }

    fn close(&mut self, open_ix: usize, kind: TreeKind) {
        assert_eq!(
            self.events[open_ix],
            Event::Open {
                kind: TreeKind::Unknown
            }
        );
        self.events[open_ix] = Event::Open { kind };
        self.events.push(Event::Close);
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(FUEL_RESET);
        self.events.push(Event::Advance);
        self.token_pos += 1;
    }

    fn eof(&self) -> bool {
        self.token_pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> Token {
        assert!(self.fuel.get() > 0);
        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.token_pos + lookahead)
            .cloned()
            .unwrap_or(Token {
                kind: TokenKind::Eof,
                len: 0,
            })
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.nth(0).kind == kind
    }

    fn error(&mut self, err: String) {
        self.events.push(Event::Error(err));
    }

    #[must_use]
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }
        self.error(format!("expected {kind}"));
    }

    fn advance_with_error(&mut self, err: String) {
        let ix = self.open();
        self.error(err);
        self.advance();
        self.close(ix, TreeKind::Error);
    }
}

#[allow(clippy::enum_glob_use)]
use TokenKind::*;

#[allow(clippy::enum_glob_use)]
use TreeKind::*;

fn manifest(p: &mut Parser) {
    let ix = p.open();

    while !p.eof() {
        if p.at(KwModule) {
            module(p);
        } else if p.at(KwFennec) {
            fennec_version(p);
        } else {
            p.advance_with_error(format!("expected {KwModule} or {KwFennec}"));
        }
    }

    p.close(ix, Manifest);
}

fn module(p: &mut Parser) {
    assert!(p.at(KwModule));

    todo!();
}

fn fennec_version(p: &mut Parser) {
    assert!(p.at(KwFennec));

    todo!();
}
