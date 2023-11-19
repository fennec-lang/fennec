// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context;
use fennec_common::types;

#[derive(serde::Deserialize)]
struct ManifestData {
    module: String,
}

pub fn extract_module(content: &str) -> Result<types::ImportPath, anyhow::Error> {
    let manifest: ManifestData =
        toml::from_str(content).context("failed to parse manifest TOML")?;
    let module_str = &manifest.module;
    let module = types::ImportPath::parse(module_str).with_context(|| {
        format!(r#"failed to parse manifest module import path "{module_str}""#)
    })?;
    Ok(module)
}
