# Fennec spec notes

Quick specification of what is *actually implemented*.

## Code organization

- unit of distribution/versioning = module
  - `fennec new` creates new module
- module is identified by path, like in Go
  - last non-version path element must be a package name,
    to eliminate the need for `package foo` at the top of every file
- first path element must be domain-like for any external dependency
  - if it is not domain-like, it is either a module from std or self
  - this is a compromise between clean separation of std
    and requirement to invent random domain names to create new modules
- module root directory is identified by a module manifest (`fennec.mod`)
  - module directory subtrees with manifests are excluded from the "parent" module
- module consists of packages (directories) with source (`.fn`) files
  - package structure = directory structure; packages can be empty
- subdirectories of module root and source files that do not follow
  valid lowercase identifier syntax are ignored (are in "detached" state)
  - in addition to consistency in naming, this allows to not think
    about file system case sensitivity
- when gathering content of a module, symlinks (files and directories) are ignored
- when gathering content of a module, we rely on file modification time
  to detect changes (we assume the change every time if mtime is unavailable)
- maximum source file size is 2^24 (16 megabytes)

## Module manifest (`fennec.mod`) format

```
module "example/hello" // comment
fennec 0.1.0
```
