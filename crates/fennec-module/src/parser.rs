// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cell::Cell;

use crate::lexer::{lex, Token, TokenKind};

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Eq, Debug)]
enum TreeKind {
    TreeUnknown,
    TreeError,
    TreeManifest,
    TreeEmpty,
    TreeModule,
    TreeFennec,
}

#[allow(clippy::enum_glob_use)]
use TokenKind::*;

#[allow(clippy::enum_glob_use)]
use TreeKind::*;

pub(crate) fn parse(input: &str) -> Tree {
    let tokens = lex(input);
    let mut p = Parser::new(tokens);
    manifest(&mut p);
    p.build_tree()
}

#[derive(Debug)]
pub(crate) struct Tree {
    kind: TreeKind,
    children: Vec<Node>,
}

#[derive(Debug)]
enum Node {
    Token(Token),
    Error(String),
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
    Advance(usize),
    Error(String),
}

const FUEL_RESET: u32 = 64;

fn top(stack: &mut Vec<Tree>) -> &mut Tree {
    stack.last_mut().expect("stack must be non-empty")
}

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
        assert!(self.eof());

        let mut tokens = self.tokens.into_iter();
        let mut stack = vec![Tree {
            kind: TreeUnknown,
            children: Vec::new(),
        }];

        for event in self.events {
            match event {
                Event::Open { kind } => {
                    assert_ne!(kind, TreeUnknown);
                    stack.push(Tree {
                        kind,
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

        if let Node::Tree(tree) = stack[0].children.remove(0) {
            tree
        } else {
            panic!("invalid node at the top of stack");
        }
    }

    fn open(&mut self) -> usize {
        let open_ix = self.events.len();
        self.events.push(Event::Open { kind: TreeUnknown });
        open_ix
    }

    fn close(&mut self, open_ix: usize, kind: TreeKind) {
        assert_eq!(self.events[open_ix], Event::Open { kind: TreeUnknown });
        self.events[open_ix] = Event::Open { kind };
        self.events.push(Event::Close);
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(FUEL_RESET);
        let mut trivia = 0;
        while !self.eof() && self.tokens[self.token_pos].kind.is_trivia() {
            trivia += 1;
            self.token_pos += 1;
        }
        // Avoid advancing past the end of tokens.
        let adv = usize::from(!self.eof());
        assert!(trivia + adv > 0);
        self.events.push(Event::Advance(trivia + adv));
        self.token_pos += adv;
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
        self.close(ix, TreeError);
    }
}

fn manifest(p: &mut Parser) {
    let ix = p.open();

    while !p.eof() {
        if p.at(TokKwModule) {
            module(p);
        } else if p.at(TokKwFennec) {
            fennec(p);
        } else if p.at(TokNewline) {
            empty(p);
        } else {
            p.advance_with_error(format!(
                "expected {TokKwModule}, {TokKwFennec} or {TokNewline}"
            ));
        }
    }

    p.close(ix, TreeManifest);
}

fn module(p: &mut Parser) {
    assert!(p.at(TokKwModule));
    let ix = p.open();

    p.expect(TokKwModule);
    p.expect(TokString);
    p.expect(TokNewline);

    p.close(ix, TreeModule);
}

fn fennec(p: &mut Parser) {
    assert!(p.at(TokKwFennec));
    let ix = p.open();

    p.expect(TokKwFennec);
    p.expect(TokVersion);

    p.close(ix, TreeFennec);
}

fn empty(p: &mut Parser) {
    assert!(p.at(TokNewline));
    let ix = p.open();

    while p.eat(TokNewline) {}

    p.close(ix, TreeEmpty);
}

#[cfg(test)]
mod tests {
    use fennec_common::types::TextSize;

    struct Tree {
        kind: super::TreeKind,
        children: Vec<Node>,
    }

    enum Node {
        Token(super::TokenKind, String),
        Error(String),
        Tree(Tree),
    }

    impl std::fmt::Debug for Tree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let kind = &self.kind;
            let children = &self.children;
            write!(f, "{kind:?}: {children:#?}")
        }
    }

    impl std::fmt::Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node::Token(kind, s) => write!(f, "{kind:?} {s:?}"),
                Node::Tree(tree) => tree.fmt(f),
                Node::Error(err) => err.fmt(f),
            }
        }
    }

    fn to_tree_impl(tree: super::Tree, input: &str, pos: &mut TextSize) -> Tree {
        Tree {
            kind: tree.kind,
            children: tree
                .children
                .into_iter()
                .map(|n| to_node_impl(n, input, pos))
                .collect(),
        }
    }

    fn to_node_impl(node: super::Node, input: &str, pos: &mut TextSize) -> Node {
        match node {
            super::Node::Token(tok) => {
                let from: usize = (*pos).into();
                let to: usize = (*pos + tok.len).into();
                *pos += tok.len;
                Node::Token(tok.kind, input[from..to].to_owned())
            }
            super::Node::Tree(tree) => Node::Tree(to_tree_impl(tree, input, pos)),
            super::Node::Error(err) => Node::Error(err),
        }
    }

    fn parse(input: &str) -> Tree {
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
