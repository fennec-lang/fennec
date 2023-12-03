// Copyright 2023 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;

use crate::util;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ImportPath {
    path: String,
    package: String,
    has_domain: bool,
}

impl std::fmt::Debug for ImportPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(f)
    }
}

pub(crate) static PACKAGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z][a-z0-9_]*$").expect(BAD_RE));
static PATH_ELEM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._\-~]+$").expect(BAD_RE));
static PATH_ELEM_DENY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"~[0-9]+$").expect(BAD_RE));
static DOMAIN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9.-]+$").expect(BAD_RE));
const BAD_RE: &str = "invalid regex literal";

impl ImportPath {
    // From https://go.dev/ref/mod#go-mod-file-ident:
    //
    // A module path must satisfy the following requirements:
    //
    // - The path must consist of one or more path elements separated by slashes
    //   (/, U+002F). It must not begin or end with a slash.
    // - Each path element is a non-empty string made of up ASCII letters, ASCII
    //   digits, and limited ASCII punctuation (-, ., _, and ~).
    // - A path element may not begin or end with a dot (., U+002E).
    // - The element prefix up to the first dot must not be a reserved file name
    //   on Windows, regardless of case (CON, com1, NuL, and so on).
    // - The element prefix up to the first dot must not end with a tilde
    //   followed by one or more digits (like EXAMPLE~1.COM).
    //
    // If the module path appears in a require directive and is not replaced, or
    // if the module paths appears on the right side of a replace directive, the
    // go command may need to download modules with that path, and some
    // additional requirements must be satisfied.
    //
    // - The leading path element (up to the first slash, if any), by convention
    //   a domain name, must contain only lower-case ASCII letters, ASCII digits,
    //   dots (., U+002E), and dashes (-, U+002D); it must contain at least one
    //   dot and cannot start with a dash.
    // - For a final path element of the form /vN where N looks numeric (ASCII
    //   digits and dots), N must not begin with a leading zero, must not be /v1,
    //   and must not contain any dots.

    pub fn parse(path: &str) -> Result<ImportPath, anyhow::Error> {
        Self::do_parse(path, false)
    }

    pub fn parse_external_dep(path: &str) -> Result<ImportPath, anyhow::Error> {
        Self::do_parse(path, true)
    }

    fn do_parse(path: &str, expect_domain: bool) -> Result<ImportPath, anyhow::Error> {
        let mut last: Option<&str> = None;
        let mut before_last: Option<&str> = None;
        let mut has_domain = false;
        for elem in path.split('/') {
            Self::check_path_element(elem)?;
            if last.is_none() {
                let d = Self::check_domain(elem);
                has_domain = d.is_ok();
                if expect_domain {
                    d?;
                }
            } else {
                before_last = last;
            }
            last = Some(elem);
        }

        let last = last.ok_or(anyhow!(
            "import path must consist of at least one valid path element"
        ))?;

        // For consistency, we require version suffix to always be valid.
        let has_version = Self::check_version_suffix(last)?;

        // On top of Go rules, we additionally require that last
        // non-version path element is a valid identifier (package name).
        let package = if has_version {
            before_last.ok_or(anyhow!("import path must contain a non-version element"))?
        } else {
            last
        };
        Self::check_package(package)?;

        Ok(ImportPath {
            path: path.into(),
            package: package.into(),
            has_domain,
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn package(&self) -> &str {
        &self.package
    }

    #[must_use]
    pub fn has_domain(&self) -> bool {
        self.has_domain
    }

    pub fn join(&self, rel: &str) -> Result<ImportPath, anyhow::Error> {
        let mut path = self.path.clone();
        path += "/";
        path += rel;
        Self::do_parse(&path, false)
    }

    fn check_path_element(elem: &str) -> Result<(), anyhow::Error> {
        if elem.is_empty() {
            return Err(anyhow!("import path element must be non-empty"));
        }

        if elem.starts_with('.') || elem.ends_with('.') {
            return Err(anyhow!(
                "import path element must not start nor end with a dot"
            ));
        }

        if elem.contains("..") {
            return Err(anyhow!(
                "import path element must not contain two dots in a row"
            ));
        }

        if !PATH_ELEM_RE.is_match(elem) {
            let re = PATH_ELEM_RE.as_str();
            return Err(anyhow!("import path element must match {re}"));
        }

        let prefix = match elem.split_once('.') {
            Some(s) => s.0,
            None => elem,
        };

        if PATH_ELEM_DENY_RE.is_match(prefix) {
            let re = PATH_ELEM_DENY_RE.as_str();
            return Err(anyhow!("import path element prefix must not match {re}"));
        }

        if util::is_reserved_windows_filename(prefix) {
            return Err(anyhow!("import path element must not have {prefix} prefix"));
        }

        Ok(())
    }

    fn check_domain(elem: &str) -> Result<(), anyhow::Error> {
        if !DOMAIN_RE.is_match(elem) {
            let re = DOMAIN_RE.as_str();
            return Err(anyhow!("import path domain must match {re}"));
        }

        if elem.starts_with('-') {
            return Err(anyhow!("import path domain must not start with a dash"));
        }

        if !elem.contains('.') {
            return Err(anyhow!("import path domain must contain a dot"));
        }

        Ok(())
    }

    fn check_version_suffix(elem: &str) -> Result<bool, anyhow::Error> {
        if !util::is_version_like(elem) {
            return Ok(false);
        }

        if elem == "v1" {
            return Err(anyhow!("import path version suffix must not be v1"));
        }

        if elem.starts_with("v0") {
            return Err(anyhow!("import path version suffix must not start with v0"));
        }

        if elem.contains('.') {
            return Err(anyhow!("import path version suffix must not contain dots"));
        }

        Ok(true)
    }

    fn check_package(elem: &str) -> Result<(), anyhow::Error> {
        if !PACKAGE_RE.is_match(elem) {
            let re = PACKAGE_RE.as_str();
            return Err(anyhow!("import path package name must match {re}"));
        }

        Ok(())
    }
}

impl fmt::Display for ImportPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn parse_path() -> Result<(), anyhow::Error> {
        let expected = [
            (
                "fmt",
                ImportPath {
                    path: "fmt".into(),
                    package: "fmt".into(),
                    has_domain: false,
                },
            ),
            (
                "math/bits",
                ImportPath {
                    path: "math/bits".into(),
                    package: "bits".into(),
                    has_domain: false,
                },
            ),
            (
                "math/bits/v2",
                ImportPath {
                    path: "math/bits/v2".into(),
                    package: "bits".into(),
                    has_domain: false,
                },
            ),
            (
                "example/hello",
                ImportPath {
                    path: "example/hello".into(),
                    package: "hello".into(),
                    has_domain: false,
                },
            ),
            (
                "example.org/hello",
                ImportPath {
                    path: "example.org/hello".into(),
                    package: "hello".into(),
                    has_domain: true,
                },
            ),
            (
                "github.com/fennec-lang/fennec",
                ImportPath {
                    path: "github.com/fennec-lang/fennec".into(),
                    package: "fennec".into(),
                    has_domain: true,
                },
            ),
            (
                "github.com/fennec-lang/fennec/v2/test",
                ImportPath {
                    path: "github.com/fennec-lang/fennec/v2/test".into(),
                    package: "test".into(),
                    has_domain: true,
                },
            ),
        ];

        for p in expected {
            let r = ImportPath::parse(p.0)?;
            assert_eq!(r, p.1);
        }

        let errors = [
            "",
            "/test",
            "test/",
            "/test/",
            "test.mod",
            "v2",
            "test/v1",
            "con",
            "test/com1",
        ];

        for e in errors {
            let r = ImportPath::parse(e);
            assert!(r.is_err());
        }

        let dep_errors = [
            "test",
            "test/hello",
            "example..org/test",
            "example.org//test",
            "example.org/CON.2/test",
            "example.org/hello~0/test",
            "example.org/hello~0.com/test",
            "example.org/hello/test~",
            "example.org/test/v1",
            "example.org/test/v0.1",
            "example.org/test/v2.5",
        ];

        for e in dep_errors {
            let r = ImportPath::parse_external_dep(e);
            assert!(r.is_err());
        }

        Ok(())
    }
}
