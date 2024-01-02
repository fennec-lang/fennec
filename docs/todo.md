# TODO

Areas of possible improvement.

## Implementation

- incremental re-lex and re-parse
  - lexing: newline-delimited
  - parsing: token-bounded or (approximately) closest enclosing `{}` block
- consider using a rope for source code? depends on:
  - [logos support](https://github.com/maciejhirsz/logos/issues/222)
  - [crop support](https://github.com/nomad/crop/issues/15)

## Miscellaneous

- add vscode language icon
- look over old docs for interesting ideas for `design.md`
