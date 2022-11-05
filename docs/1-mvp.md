# MVP

- start by what is definitely required
- no warnings
- deep const by default, no null

- comments
- literals
  - fixed-size for now
- type definitions
  - struct
  - enum
- function definitions
- generics

- visibility model?
  - go-style: can't go `like_this`, have to go `likeThis` :-(

- later
  - typedef, newtype
  - top-level data
  - typed annotations (= linked data)
  - data for root file (start with empty marker file)

## Syntax

- easy to parse
- redundancy to restart the parsing
- a bit of punctuation
- hot whitespace-sensitive (like go, maybe?)
