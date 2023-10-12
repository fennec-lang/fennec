// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{anyhow, Context};
use fennec_common::{types, workspace};

#[derive(serde::Deserialize)]
struct ManifestData {
    module: String,
    fennec: String,
}

pub fn parse(content: &str) -> Result<workspace::ModuleManifest, anyhow::Error> {
    let manifest: ManifestData =
        toml::from_str(content).context("failed to parse manifest TOML")?;
    let ip_str = &manifest.module;
    let ip = types::ImportPath::parse(ip_str)
        .with_context(|| format!(r#"failed to parse manifest module import path "{ip_str}""#))?;
    let ver_str = &manifest.fennec;
    let ver = semver::Version::parse(ver_str)
        .with_context(|| format!(r#"failed to parse manifest fennec version "{ver_str}""#))?;
    if !ver.pre.is_empty() {
        return Err(anyhow!(
            r#"module manifest fennec version "{ver_str}" contains a pre-release part"#
        ));
    }
    if !ver.build.is_empty() {
        return Err(anyhow!(
            r#"module manifest fennec version "{ver_str}" contains a build metadata part"#
        ));
    }
    Ok(workspace::ModuleManifest {
        module: ip,
        fennec: types::FennecVersion {
            major: ver.major,
            minor: ver.minor,
            patch: ver.patch,
        },
    })
}
