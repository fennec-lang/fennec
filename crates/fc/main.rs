// Copyright 2022 Gregory Petrosyan <gregory.petrosyan@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::result::Result;

mod repl;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let mut env = repl::Env::new()?;

    loop {
        match env.readline() {
            Ok(line) => {
                println!("ok {}", line)
            }
            Err(err) => {
                if repl::eof(&err) {
                    break;
                }
                println!("error {}", err)
            }
        }
    }

    env.close()
}
