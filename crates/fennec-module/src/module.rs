// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::anyhow;
use fennec_common::{types, PROJECT_NAME, RELEASE_VERSION, RELEASE_VERSION_STR};

use crate::{analysis, parser};

pub fn extract(input: &str) -> Result<types::ImportPath, anyhow::Error> {
    let tree = parser::parse(input);
    if cfg!(fuzzing) {
        let reconstructed = parser::reconstruct_input(input, &tree);
        assert_eq!(input, &reconstructed);
    }
    let (module, version, errors) = analysis::visit(input, tree);
    if !errors.is_empty() {
        return Err(anyhow!("{errors:?}"));
    }
    let Some(module) = module else {
        return Err(anyhow!("missing module import path"));
    };
    let Some(version) = version else {
        return Err(anyhow!("missing {PROJECT_NAME} version requirement"));
    };
    let version_req = semver::VersionReq {
        comparators: vec![semver::Comparator {
            op: semver::Op::Caret,
            major: version.major,
            minor: Some(version.minor),
            patch: Some(version.patch),
            pre: version.pre.clone(),
        }],
    };
    if !version_req.matches(&RELEASE_VERSION) {
        let release: &semver::Version = &RELEASE_VERSION;
        return Err(anyhow!(
            "required {PROJECT_NAME} version {version} does not match {release}"
        ));
    }
    Ok(module)
}

pub fn write_with_current_version(
    w: &mut dyn std::io::Write,
    path: &types::ImportPath,
) -> Result<(), anyhow::Error> {
    write!(
        w,
        "module \"{path}\"\n{PROJECT_NAME} {RELEASE_VERSION_STR}\n"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use fennec_common::types;

    #[test]
    fn write_with_current_version_works() {
        let path = types::ImportPath::parse("examples/hello").unwrap();
        let mut buf = Vec::new();
        super::write_with_current_version(&mut buf, &path).unwrap();
    }
}
