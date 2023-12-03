// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{cell::Cell, ops::Range};

use crate::lexer::{lex, Token, TokenKind};

// Parser design heavily based on https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html.
pub(crate) fn parse(input: String) -> Tree {
    let tokens = lex(&input);
    let mut p = Parser::new(input, tokens);
    manifest(&mut p);
    p.build_tree()
}

pub(crate) struct Tree {
    kind: TreeKind,
    children: Vec<Node>,
}

#[derive(PartialEq, Eq, Debug)]
enum TreeKind {
    Error,
    Manifest,
}

enum Node {
    Token(Token),
    Tree(Tree),
}

struct Parser {
    input: String,
    tokens: Vec<Token>,
    token_pos: usize,
    source_pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
    errors: Vec<(Range<usize>, String)>,
}

#[derive(PartialEq, Eq, Debug)]
enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}

const FUEL_RESET: u32 = 64;

impl Parser {
    fn new(input: String, tokens: Vec<Token>) -> Parser {
        Parser {
            input,
            tokens,
            token_pos: 0,
            source_pos: 0,
            fuel: Cell::new(FUEL_RESET),
            events: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn build_tree(self) -> Tree {
        todo!()
    }

    fn open(&mut self) -> usize {
        let open_ix = self.events.len();
        self.events.push(Event::Open {
            kind: TreeKind::Error,
        });
        open_ix
    }

    fn close(&mut self, open_ix: usize, kind: TreeKind) {
        assert_eq!(
            self.events[open_ix],
            Event::Open {
                kind: TreeKind::Error
            }
        );
        self.events[open_ix] = Event::Open { kind };
        self.events.push(Event::Close);
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(FUEL_RESET);
        self.events.push(Event::Advance);
        let len = self.tokens[self.token_pos].len as usize;
        self.token_pos += 1;
        self.source_pos += len;
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

    fn error(&mut self, start_pos: usize, end_pos: usize, err: String) {
        self.errors.push((start_pos..end_pos, err));
    }

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
        let len = self.nth(0).len as usize;
        let got = &self.input[self.source_pos..self.source_pos + len];
        self.error(
            self.source_pos,
            self.source_pos,
            format!("expected {kind}, got {got:?}"),
        );
    }

    fn advance_with_error(&mut self, err: String) {
        let ix = self.open();
        let start_pos = self.source_pos;
        self.advance();
        self.error(start_pos, self.source_pos, err);
        self.close(ix, TreeKind::Error);
    }
}

fn manifest(_p: &mut Parser) {
    todo!()
}
