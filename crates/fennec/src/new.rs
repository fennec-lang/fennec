// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context;
use fennec_common::{types, MODULE_MANIFEST_FILENAME};
use std::{fs, path::Path};

#[derive(clap::Args)]
pub struct Args {
    /// Module path
    module_path: String,

    /// Directory to use. Defaults to the package name from the module path
    #[arg(long)]
    dir: Option<String>,
}

pub fn cmd(args: &Args) -> anyhow::Result<()> {
    let mod_path = &args.module_path;
    let mod_path = types::ImportPath::parse(mod_path)
        .with_context(|| format!(r#"invalid module path "{mod_path}""#))?;

    let dir = args.dir.as_deref().unwrap_or(mod_path.package());
    fs::create_dir_all(dir)
        .with_context(|| format!(r#"failed to create module directory "{dir}""#))?;

    let path = Path::new(dir).join(MODULE_MANIFEST_FILENAME);
    let disp = path.display();
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .with_context(|| format!(r#"failed to create "{disp}""#))?;

    fennec_module::write_with_current_version(&mut file, &mod_path)
        .with_context(|| format!(r#"failed to write module declaration to "{disp}""#))?;

    Ok(())
}
