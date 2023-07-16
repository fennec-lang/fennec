# Design

## Vision

"An interactive programming language for building reliable and efficient software".

### Goals

1. interactivity: instant feedback on any code change
2. safety: fully safe with either static or dynamic enforcement
3. efficiency: control over data layout and absence of wasteful computation
4. simplicity, in both language and programs that it guides you to write

### Non-goals

- expressivity, in both syntax and semantics
- maximal performance or zero-overhead

### Directions

- small language with nimble feel
- lightweight typing
- built by induction to always be auto-testable
- total determinism and purity, I/O only in the shell
- instant feedback
- fast interpreter/compiler for development, WASM/C/LLVM for production
- aggressively cut anything non-essential to vision

## Decisions

### Versioning

- copy Go + simplify where possible
  - avoid central registry
  - don't spend too much time on a solved problem
- `fennec.toml` instead of `go.mod`
  - extensible config (external tools, publishing, ...), avoid NIH
- make package name always match the "directory" name (except for `v2`)
  - don't require `package foo` on top of every file
- `vendor` always exists, but can be ignored in source control (or not)
  - nudge people to be mindful of dependencies
  - makes modules more self-contained
- is including a `version` field to be independent of source control a bad idea?

```toml
fennec = "1.0"
module = "example.org/hello"

[require]
"example.org/util" = "1.2.3"
```

### Safety

- full memory and thread safety is a must
- DRF-SC is a must
  - makes testing possible
- `unsafe` only possible in the standard library
  - nudges people towards standardizing all unsafe idioms in one place
  - guarantees total safety of every package and by induction every program

### Iteration

- interior iteration by non-escaping coroutines
  - much simpler and direct expression
  - no complex lifetimes to manage, direct transfer of control

### Concurrency

- express concurrency directly as control flow, not data
  - ["Storing Data in Control Flow"](https://research.swtch.com/pcdata)
- no `async`/`await`
  - horrible complexity for minimal gain
- structured concurrency only for simplicity and composability
  - same way as we prefer tree-like data, we prefer tree-like control flow

### Error handling

- abort on any contract/invariant violation, including assertions
  - avoid throw-while-throwing, exception safety, etc.
  - runtime should re-instantiate and re-wire component if desirable
  - runtime will manage all external resources, leaks are not a problem
- otherwise, `Result`

### Value semantics

- explicit copy and move
- RAII with deterministic destruction as an important design tool
  - actual memory management can be either GC or refcounting

### Macros

- of course no

## Research areas

### References and pointers

- owned values + A^M references as a default
- for aliasing, use pointers or indices
  - pointer + dereference = index + subscript
  - prefer indices as more local
- pointers
  - pointer requires value to be mutex-wrapped to dereference
    - if you only want to compare and not dereference, mutex can be optional?
  - pointers have `unowned`-like semantics
    - Swift-like or [`BackupRefPtr`](https://docs.google.com/document/d/1m0c63vXXLyGtIGBi9v6YFANum7-IRC3-dmiYBCWqkMk/edit)-like implementation
  - you can only form pointers to heap allocations
  - one implicit global allocator
    - to avoid having everything generic over the allocator
    - any useful functionality for allocation should go to the allocator everybody uses
- indices
  - index = pointer that requires a container to dereference
  - container + index should replace custom allocators (arenas etc.)
  - indices requires you to have a proper reference to the container
  - index can have different level of paranoia about use-after-free
    - raw int / `slotmap`-style generations / refcounts?
  - index looks like a great fit for the "context-parametrized struct" idea
  - experiment with indices being typed (pointer-like)
    - [might need](https://matklad.github.io/2023/05/02/implicits-for-mvs.html)
      implicits or dictionary-passing
- mutexes
  - should be able to make them non-atomic for single-threaded components

## Random ideas

### Testing

- liberal use of assertions in non-test code
- always test automatically
  - no crashes
  - no leaks
  - no hangs
- general way to specify properties, superset of testing and REPL
  - execute -> click to freeze the results to get a test -> generalize to property-based test
- aspect-like properties?
  - fully automatic running of random programs; no code necessary to drive
  - properties that bind to runs that satisfy certain conditions and assert certain things
    - sometimes we can cheat an use property condition as a generation instruction
    - this unifies simple properties for functions and complex properties for stateful
      interactions (that can be interleaved with other stateful interactions)
- omniscient debugging with an editor as the only (and main) UI
  - smart output
  - smart focus
  - text-driven target choice?
  - hyperlinks to jump back / forward
- only re-run corpus / re-run + explore a bit / re-run + explore more / full fuzzing modes
- green checkmark meaning current corpus is passing

### Fuzzing

- text encoding of corpus to be able to re-run it without a DB
- uncovered code as a warning
- explicit arbitrary choice
  - constants in code and in test scenarios
  - choice of actions in test scenarios
- statement to signal progress to a fuzzer to help guide the search
  - ability to return partially ordered data structure as a progress measure?
- ability to guide a fuzzer through a condition by providing an example
- magical var-size primitives to make enumeration possible
- sage-like "stray off the usual path" exploration model
- simplex-like / hurst-exponent-like modeling of correlated rare events
- simulated annealing to guide search

### Metaprogramming

- since we can run arbitrary code at compile time, this should be easy?

### FFI

- go all-in on WASM components?
  - built-in shells should be components
  - later, allow people to compose components themselves

## Architecture

TODO

## Roadmap

TODO

### Language features

- first steps:
  - booleans, integers, floating point
  - if, loop
  - functions
  - packages, modules, visibility control
- ability to fuzz components in isolation
- ability to compose separately fuzzed components

### Challenges

- iterator `merge`/`zip`
- generic `min`
- tree + tree algorithms and traversals
- slice (a container with required context?)
  - maybe builtin?

## TODO (small things)

### Miscellaneous

- logo (project + github social previews + opengraph + vscode language icon + vscode extension icon)
- look over old docs for interesting ideas for this document

### VSCode

- remove hello command
- setup testing with CI
- decide what declarative features we have to provide
