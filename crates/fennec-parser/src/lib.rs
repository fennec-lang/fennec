// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub example);

#[test]
fn lalrpop_example() {
    assert!(example::TermParser::new().parse("22").is_ok());
    assert!(example::TermParser::new().parse("(22)").is_ok());
    assert!(example::TermParser::new().parse("((((22))))").is_ok());
    assert!(example::TermParser::new().parse("((22)").is_err());
}
