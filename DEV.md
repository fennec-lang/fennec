# Development

## Publishing a new release on crates.io

Use [cargo-release](https://github.com/crate-ci/cargo-release), e.g. `cargo release 0.1.3`.

## Publishing a new release on Visual Studio Code Marketplace

Run `vsce publish` in the `ide/vscode-fennec` directory.

## Checking licenses of the dependencies

Use [cargo-deny](https://github.com/EmbarkStudios/cargo-deny): `cargo deny check`.
