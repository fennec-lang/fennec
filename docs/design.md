# Design

## Versioning

- copy Go + simplify where possible
- `fennec.toml` instead of `go.mod` to have an extensible config (external tools, publishing, ...)
- make package name always match the "directory" name (except for `v2`)
- `vendor` always exists, but can be ignored in source control (or not)
- is including a `version` field to be independent of source control a bad idea?

```toml
fennec = "1.0"
module = "example.org/hello"

[require]
"example.org/util" = "1.2.3"
```
