// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::anyhow;
use fennec_common::{
    types::{self, TextRange, TextSize},
    PROJECT_NAME,
};

use crate::{
    lexer::{Token, TokenKind},
    parser::{Node, Tree, TreeKind},
};

pub(crate) fn visit(
    input: &str,
    tree: Tree,
) -> (
    Option<types::ImportPath>,
    Option<semver::Version>,
    Vec<Error>,
) {
    let mut vis = Visitor {
        input,
        ..Default::default()
    };
    vis.visit_tree(tree);
    (vis.module, vis.version, vis.errors)
}

#[derive(Default)]
struct Visitor<'input> {
    input: &'input str,
    pos: TextSize,
    module: Option<types::ImportPath>,
    version: Option<semver::Version>,
    errors: Vec<Error>,
}

impl<'input> Visitor<'input> {
    fn visit_tree(&mut self, tree: Tree) {
        for (ix, node) in tree.children.into_iter().enumerate() {
            if tree.kind == TreeKind::Module && self.module.is_none() {
                self.try_parse_module_child(&node);
            } else if tree.kind == TreeKind::Fennec && self.version.is_none() {
                self.try_parse_version_child(&node);
            }

            let err_loc = if tree.kind == TreeKind::Error && ix == 0 {
                Some(tree.loc)
            } else {
                None
            };
            self.visit_node(node, err_loc);
        }
    }

    fn visit_node(&mut self, node: Node, err_loc: Option<TextRange>) {
        match node {
            Node::Token(tok) => {
                if let TokenKind::Error(kind) = tok.kind {
                    self.errors.push(Error::new(
                        TextRange::at(self.pos, tok.len),
                        kind.to_string(),
                    ));
                }
                self.pos += tok.len;
            }
            Node::Tree(tree) => {
                self.visit_tree(tree);
            }
            Node::Error(err) => {
                let loc = err_loc.unwrap_or(TextRange::empty(self.pos));
                self.errors.push(Error::new(loc, err));
            }
        }
    }

    fn parse_module(module_str: &str) -> Result<types::ImportPath, anyhow::Error> {
        let unquoted = &module_str[1..module_str.len() - 1];
        types::ImportPath::parse(unquoted)
    }

    fn try_parse_module_child(&mut self, node: &Node) {
        let Node::Token(Token {
            kind: TokenKind::String,
            len,
        }) = node
        else {
            return;
        };
        let loc = TextRange::at(self.pos, *len);
        let module_str = &self.input[loc];
        let module = Self::parse_module(module_str);
        match module {
            Ok(module) => {
                self.module = Some(module);
            }
            Err(err) => {
                self.errors.push(Error::new(
                    loc,
                    format!("failed to parse import path {module_str:?}: {err}"),
                ));
            }
        }
    }

    fn parse_version(version_str: &str) -> Result<semver::Version, anyhow::Error> {
        let v = semver::Version::parse(version_str)?;
        if !v.build.is_empty() {
            return Err(anyhow!("non-empty semver build metadata"));
        }
        Ok(v)
    }

    fn try_parse_version_child(&mut self, node: &Node) {
        let Node::Token(Token {
            kind: TokenKind::Version,
            len,
        }) = node
        else {
            return;
        };
        let loc = TextRange::at(self.pos, *len);
        let version_str = &self.input[loc];
        let version = Self::parse_version(version_str);
        match version {
            Ok(version) => {
                self.version = Some(version);
            }
            Err(err) => {
                self.errors.push(Error::new(
                    loc,
                    format!("failed to parse {PROJECT_NAME} version {version_str:?}: {err}"),
                ));
            }
        }
    }
}

pub(crate) struct Error {
    loc: TextRange,
    err: String,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let loc = &self.loc;
        let err = &self.err;
        write!(f, "{loc:?}: {err}") // TODO: should include source and be compatible with the standard error location syntax
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(loc: TextRange, err: String) -> Error {
        Error { loc, err }
    }
}
