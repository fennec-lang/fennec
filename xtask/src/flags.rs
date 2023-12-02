// Copyright 2022 Gregory Petrosyan <pgregory@pgregory.net>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

xflags::xflags! {
    src "./src/flags.rs"

    cmd xtask {
        /// Verbose output
        optional -v, --verbose

        /// Run CI-like checks
        cmd ci {
            /// Run all CI-like checks
            optional --all
        }

        // Generate lexer code
        cmd gen-lex {}

        /// Run Clippy
        cmd lint {}

        /// Check spelling
        cmd spellcheck {}

        /// Check dependencies: unused crates and license compatibility
        cmd check-deps {}

        /// Publish a new release on GitHub and crates.io
        cmd release-crate {
            /// Version to publish, in semver format
            required version: String
            /// Actually perform a release. Dry-run mode is the default
            optional --execute
        }

        /// Publish a new release on Visual Studio Marketplace
        cmd release-ext {
            /// Actually perform a release. Dry-run mode is the default
            optional --execute
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub verbose: bool,
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Ci(Ci),
    GenLex(GenLex),
    Lint(Lint),
    Spellcheck(Spellcheck),
    CheckDeps(CheckDeps),
    ReleaseCrate(ReleaseCrate),
    ReleaseExt(ReleaseExt),
}

#[derive(Debug)]
pub struct Ci {
    pub all: bool,
}

#[derive(Debug)]
pub struct GenLex;

#[derive(Debug)]
pub struct Lint;

#[derive(Debug)]
pub struct Spellcheck;

#[derive(Debug)]
pub struct CheckDeps;

#[derive(Debug)]
pub struct ReleaseCrate {
    pub version: String,

    pub execute: bool,
}

#[derive(Debug)]
pub struct ReleaseExt {
    pub execute: bool,
}

impl Xtask {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
