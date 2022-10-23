// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate lalrpop_util;

mod ast;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    InputTooBig,
}

lalrpop_mod!(
    #[allow(dead_code, clippy::all, clippy::pedantic)]
    fennec
);

#[test]
fn lalrpop_example() {
    use lalrpop_util::ParseError;

    let mut errors = Vec::new();

    let expr = fennec::ExprsParser::new().parse(&mut errors, "2147483648");
    assert!(expr.is_err());
    assert_eq!(
        expr.unwrap_err(),
        ParseError::User {
            error: Error::InputTooBig
        }
    );

    let expr = fennec::ExprsParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * error) + 3)]");

    let expr = fennec::ExprsParser::new()
        .parse(&mut errors, "22 * 44 + 66, *3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");

    let expr = fennec::ExprsParser::new().parse(&mut errors, "*").unwrap();
    assert_eq!(&format!("{:?}", expr), "[(error * error)]");

    assert_eq!(errors.len(), 4);
}
