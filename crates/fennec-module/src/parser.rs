// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fennec_common::types::{TextRange, TextSize};
use std::cell::Cell;

use crate::lexer::{lex, Token, TokenKind};

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum TreeKind {
    Unknown,
    Error,
    Manifest,
    Empty,
    Module,
    Fennec,
}

use TokenKind as T;
use TreeKind as N;

pub(crate) fn parse(input: &str) -> Tree {
    let tokens = lex(input);
    let mut p = Parser::new(tokens);
    manifest(&mut p);
    p.build_tree()
}

pub(crate) fn reconstruct_input(input: &str, tree: &Tree) -> String {
    let mut s = String::new();
    let mut pos: TextSize = 0.into();
    reconstruct_tree(&mut s, &mut pos, input, tree);
    s
}

fn reconstruct_tree(s: &mut String, pos: &mut TextSize, input: &str, tree: &Tree) {
    for child in &tree.children {
        match child {
            Node::Token(tok) => {
                let loc = TextRange::at(*pos, tok.len);
                s.push_str(&input[loc]);
                *pos += tok.len;
            }
            Node::Error(_) => {}
            Node::Tree(tree) => reconstruct_tree(s, pos, input, tree),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Tree {
    pub(crate) kind: TreeKind,
    pub(crate) loc: TextRange,
    pub(crate) children: Vec<Node>,
}

#[derive(Debug)]
pub(crate) enum Node {
    Token(Token),
    Error(String),
    Tree(Tree),
}

struct Parser {
    tokens: Vec<Token>,
    token_pos: usize,
    pos: TextSize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

#[derive(PartialEq, Eq, Debug)]
enum Event {
    Open { kind: TreeKind, loc: TextRange },
    Close,
    Advance(usize),
    Error(String),
}

const FUEL_RESET: u32 = 64;

fn top(stack: &mut [Tree]) -> &mut Tree {
    stack.last_mut().expect("stack must be non-empty")
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        let tok_len = tokens.len();
        Parser {
            tokens,
            token_pos: 0,
            pos: 0.into(),
            fuel: Cell::new(FUEL_RESET),
            events: Vec::with_capacity(tok_len * 2),
        }
    }

    fn build_tree(self) -> Tree {
        assert!(self.eof());

        let mut tokens = self.tokens.into_iter();
        let mut stack = vec![Tree {
            kind: N::Unknown,
            loc: TextRange::empty(0.into()),
            children: Vec::new(),
        }];

        for event in self.events {
            match event {
                Event::Open { kind, loc } => {
                    assert_ne!(kind, N::Unknown);
                    stack.push(Tree {
                        kind,
                        loc,
                        children: Vec::new(),
                    });
                }
                Event::Close => {
                    let tree = stack.pop().expect("stack must be non-empty");
                    top(&mut stack).children.push(Node::Tree(tree));
                }
                Event::Advance(n) => {
                    let children = &mut top(&mut stack).children;
                    for _ in 0..n {
                        let tok = tokens.next().expect("token to advance over must exist");
                        children.push(Node::Token(tok));
                    }
                }
                Event::Error(err) => {
                    top(&mut stack).children.push(Node::Error(err));
                }
            }
        }

        assert!(tokens.next().is_none());
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].children.len(), 1);

        let Node::Tree(tree) = stack[0].children.remove(0) else {
            panic!("invalid node at the top of stack");
        };
        tree
    }

    fn open(&mut self) -> usize {
        let open_ix = self.events.len();
        self.events.push(Event::Open {
            kind: N::Unknown,
            loc: TextRange::empty(self.pos),
        });
        open_ix
    }

    fn close(&mut self, open_ix: usize, close_kind: TreeKind) {
        let Event::Open { kind, loc } = &mut self.events[open_ix] else {
            panic!("mismatching close event");
        };
        assert_eq!(*kind, N::Unknown);
        *kind = close_kind;
        *loc = TextRange::new(loc.start(), self.pos);
        self.events.push(Event::Close);
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(FUEL_RESET);
        let mut trivia = 0;
        // Skip trivia.
        while !self.eof() && self.tokens[self.token_pos].kind.is_trivia() {
            trivia += 1;
            self.pos += self.tokens[self.token_pos].len;
            self.token_pos += 1;
        }
        if self.eof() {
            // Avoid advancing past the end.
            self.events.push(Event::Advance(trivia));
        } else {
            // Do the advance.
            self.pos += self.tokens[self.token_pos].len;
            self.token_pos += 1;
            self.events.push(Event::Advance(trivia + 1));
        }
    }

    fn eof(&self) -> bool {
        self.token_pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> Token {
        assert!(self.fuel.get() > 0);
        self.fuel.set(self.fuel.get() - 1);
        self.tokens[self.token_pos..]
            .iter()
            .filter(|tok| !tok.kind.is_trivia())
            .nth(lookahead)
            .cloned()
            .unwrap_or(Token::eof())
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
        self.close(ix, N::Error);
    }
}

fn manifest(p: &mut Parser) {
    use T::{KwFennec, KwModule, Newline};
    let ix = p.open();

    while !p.eof() {
        if p.at(KwModule) {
            module(p);
        } else if p.at(KwFennec) {
            fennec(p);
        } else if p.at(Newline) {
            empty(p);
        } else {
            p.advance_with_error(format!("expected {KwModule}, {KwFennec} or {Newline}"));
        }
    }

    p.close(ix, N::Manifest);
}

fn module(p: &mut Parser) {
    assert!(p.at(T::KwModule));
    let ix = p.open();

    p.expect(T::KwModule);
    p.expect(T::String);
    p.expect(T::Newline);

    p.close(ix, N::Module);
}

fn fennec(p: &mut Parser) {
    assert!(p.at(T::KwFennec));
    let ix = p.open();

    p.expect(T::KwFennec);
    p.expect(T::Version);

    p.close(ix, N::Fennec);
}

fn empty(p: &mut Parser) {
    assert!(p.at(T::Newline));
    let ix = p.open();

    while p.eat(T::Newline) {}

    p.close(ix, N::Empty);
}

#[cfg(test)]
mod tests {
    use crate::lexer::SlicedToken;
    use fennec_common::types::{TextRange, TextSize};

    struct Tree<'input> {
        kind: super::TreeKind,
        loc: TextRange,
        children: Vec<Node<'input>>,
    }

    enum Node<'input> {
        Token(SlicedToken<'input>),
        Error(String),
        Tree(Tree<'input>),
    }

    impl<'input> std::fmt::Debug for Tree<'input> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let kind = &self.kind;
            let loc = &self.loc;
            let children = &self.children;
            write!(f, "{kind:?}[{loc:?}]: {children:#?}")
        }
    }

    impl<'input> std::fmt::Debug for Node<'input> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node::Token(tok) => tok.fmt(f),
                Node::Tree(tree) => tree.fmt(f),
                Node::Error(err) => err.fmt(f),
            }
        }
    }

    fn to_tree_impl<'input>(
        tree: super::Tree,
        input: &'input str,
        pos: &mut TextSize,
    ) -> Tree<'input> {
        Tree {
            kind: tree.kind,
            loc: tree.loc,
            children: tree
                .children
                .into_iter()
                .map(|n| to_node_impl(n, input, pos))
                .collect(),
        }
    }

    fn to_node_impl<'input>(
        node: super::Node,
        input: &'input str,
        pos: &mut TextSize,
    ) -> Node<'input> {
        match node {
            super::Node::Token(tok) => {
                let sliced = tok.to_sliced(*pos, input);
                *pos += tok.len;
                Node::Token(sliced)
            }
            super::Node::Tree(tree) => Node::Tree(to_tree_impl(tree, input, pos)),
            super::Node::Error(err) => Node::Error(err),
        }
    }

    fn parse<'input>(input: &'input str) -> Tree<'input> {
        let tree = super::parse(input);
        let mut pos = TextSize::new(0);
        to_tree_impl(tree, input, &mut pos)
    }

    #[test]
    fn parse_empty() {
        insta::assert_debug_snapshot!(parse(""));
    }

    #[test]
    fn parse_trivial_err() {
        insta::assert_debug_snapshot!(parse("?"));
    }

    #[test]
    fn parse_unterminated() {
        insta::assert_debug_snapshot!(parse("fennec \"hello\nmodule"));
    }

    #[test]
    fn parse_normal() {
        insta::assert_debug_snapshot!(parse(
            r#"
module "examples/hello"  // comment
fennec 0.1.0
"#
        ));
    }
}
