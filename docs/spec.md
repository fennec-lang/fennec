# Fennec spec notes

Quick specification of what is *actually implemented*.

## Code organization

- unit of distribution/versioning = module
- module is identified by path, like in Go
  - last non-version path element must be a package name,
    to eliminate the need for `package foo` at the top of every file
- first path element must be domain-like for any external dependency
  - if it is not domain-like, it is either a module from std or self
  - this is a compromise between clean separation of std
    and requirement to invent random domain names to create new modules
- module root directory is identified by a `fennec.toml` file
- `fennec new` creates new module