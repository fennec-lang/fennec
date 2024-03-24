# Rust coding style

- no unsafe code
- `lib.rs` should consist mostly of re-exports
  - navigating to code in `lib.rs` is awkward because there are a lot of files named `lib.rs`
- avoid `..Default::default()` everywhere instead of constructors
- errors, in general, should not include function arguments
  - that way, function caller has more freedom in the way he presents the error

## Common types

- if something exists in `fennec_common::types`, use it
  - in particular, `types::HashMap` and `types::HashSet` aliases
- use `anyhow::Error`
- use `parking_lot` primitives instead of `std`
- try to avoid using `std::sync::Arc`
