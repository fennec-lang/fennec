# Fennec design notes

## Vision

"A simple interactive programming language for building reliable and efficient software".

### Goals

1. simplicity, in both language and programs that it guides you to write
2. interactivity: instant feedback on any code change
3. reliability, through static checks and automatic testing
4. efficiency: control over data layout and absence of wasteful computation

### Non-goals

- expressivity, in both syntax and semantics
- maximal performance or zero-overhead
- maximal compatibility with existing ecosystems

### Directions

- small language with nimble feel
- lightweight typing
- built by induction to always be auto-testable
- total determinism and purity, I/O only in the shell
- instant feedback
- fast interpreter/compiler for development, WASM/C/LLVM for production
- aggressively cut anything non-essential to vision

## Decisions

### General

- no compiler switches, no warnings and no options
  - must explicitly bind or ignore results
  - must use local variables

### Versioning

- start with `FENNEC_PATH` mode (in `vendor` directory)
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

### Iteration & concurrency

- interior iteration by non-escaping coroutines
  - much simpler and direct expression
  - no complex lifetimes to manage, direct transfer of control
- no `async`/`await`
  - horrible complexity (function coloring) for minimal gain
- structured concurrency only for simplicity and composability
  - same way as we prefer tree-like data, we prefer tree-like control flow
  - how do we prevent active coroutines from escaping, unmovable or second-class types?
    - immovable might be a good idea, allowing self-referential types later
- consider:
  - `function*` in JS
  - only single-thread concurrency, with opt-in parallelism being limited to real threads?
- references:
  - [Storing Data in Control Flow](https://research.swtch.com/pcdata)
  - [Go coroutines](https://research.swtch.com/coro)
  - [Stackful vs stackless coroutines](https://blog.varunramesh.net/posts/stackless-vs-stackful-coroutines/)
  - [We never need stackful coroutines](http://hacksoflife.blogspot.com/2021/06/we-never-needed-stackfull-coroutines.html)
  - [Same fridge problem](https://www.crystalclearsoftware.com/soc/coroutine/coroutine/stackful.html)

### Error handling

- abort on any contract/invariant violation, including assertions
  - avoid throw-while-throwing, exception safety, etc.
  - runtime should re-instantiate and re-wire component if desirable
  - runtime will manage all external resources, leaks are not a problem
- otherwise, `Result`

### Value semantics

- explicit copy and move
- all objects trivially moveable
  - how do we forbid interior references?
  - interior pointer already impossible because pointers only point to heap
- RAII with deterministic destruction as an important design tool

### Macros

- of course no

## Research areas

### Syntax

- design with extensibility and forward compatibility in mind
  - reserve syntax for raw identifiers
- Go-like uncluttered feel
  - copy Go semicolon [rules](https://go.dev/ref/spec#Semicolons)?
- ideas
  - `{ .field: init }` / `{ .field = init }` initialization
  - `[]` for generics
  - swift-style `.Variant` matching
  - `.field0` for tuple access? do we need tuples at all?
- UFCS?
  - "packages can't add public functions" problem
  - research extension methods as well
  - probably no need to bother when we have traits
    - but rust-like "invisible" use (we don't know its effects) is a bit too much magic
  - while traits help with chaining, shadowing lets write convenient (and debuggable) pipelines
    - re-bind the same name at each line, e.g. `it`
- reserve `_0`, `_1`, `_2` etc.?

### References and pointers

- owned values + A^M references as a default
  - consider having `&` + `&uniq` as more descriptive names
- for aliasing, use pointers or indices
  - pointer + dereference = index + subscript
  - prefer indices as more local
  - can we unify indices and pointers? people should write code 1 time
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

### Strings / byte slices / vectors

- want:
  - UTF-8 by convention
  - one main type for mutable/immutable, string/byteslice
    - maybe a separate mutable type like `StringBuilder` as well
  - should this be the same type as `vec[u8]`?
  - no more than 16 bytes
    - `vec[vec[vec[T]]]`
    - `vec[string]` is common, but storing most stings inline is nice, too
  - SSO
  - extremely rare atomic ops and no refcount ops in the common cases (traversal, getter, pass as argument)
    - defer touching the refcount while we hold the anchoring reference
    - biased reference counting for multi-threaded components
    - can limit reference counts by 32 bits and trap on overflow
      - your program is likely wrong if you have more than 4 billion references to something
  - store compile-time-constant strings efficiently
  - can we fit strings into 16 bytes by requiring them to be immutable?
    - `vec[u8]` will be 24 and will be mutable?
    - will they return different kinds of slices?
- consider
  - should slice be the same type or not?
  - make them the only DST, thus builtin
  - cooperate with allocator on growth strategy (like `fbvector`)
  - limit the maximum size (of any collection) to 2^32?
    - what about `mmap`?
    - what the signature of `func hash(string)` be?
- progression
  - `thin_vec` (1 word) -> boxed slice (2 words) -> vec (3 words)
- references:
  - [Rust bytes](https://docs.rs/bytes/latest/bytes/)
  - [Rust bstr](https://blog.burntsushi.net/bstr/)
  - [Rust ecow](https://github.com/typst/ecow)
    - 16 bytes, cheap clone
  - [Rust imstr](https://github.com/xfbs/imstr)
    - cheap clone, cheap slice
  - [Rust tendril](https://github.com/servo/tendril)
    - slices to same type, 2^32 bytes maximum
  - [Dotnet design overview](https://github.com/dotnet/designs/blob/main/accepted/2021/utf8/validation.md)

### Maps / sets

- references:
  - [Rust indexmap](https://github.com/bluss/indexmap)

### Structs

- it should be possible to have public fields that are mutable only by methods
  - constructor + methods enforce all required invariants
  - convenient access to non-mutating interface is preserved
  - `pub` (default, can't mutate directly) vs `pub mut`
- no tuples; make structs convenient for all use cases
  - unnamed fields
  - anonymous struct with named or unnamed field (func return type)
  - immediately initialized anonymous struct
- have only single data type, which can express both sum and product types

  ```
  type Point {
          x int
          y int
          z type {
                  // ...
          }
  }

  type Result {
  case Ok:
  case Err:
  }

  type ErrorCode {
  case EAccess = 5:                   // const tag
          message = "access denied"   // const field
  case EReadOnly = 6:
          message = "read-only file"
  }
  ```

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
    - log entries as a timeline?
- only re-run corpus / re-run + explore a bit / re-run + explore more / full fuzzing modes
- green checkmark meaning current corpus is passing
- tests (usually property-based) as interface members
- inline values UI
  - loops: remember first + last
- external tests go to the `/test/` subdirectory (because we can't place them in the same as in Go)
- test auto-run
  - for every function check that if preconditions hold, postconditions hold as well
  - for simple tests, this is enough
  - for more complex scenarios, you can write a special function just for the test scenario
    - imperative rapid-like `repeat` etc.
  - should we do the same for all constructor + method combinations?
    - probably yes, otherwise there is no incentive to write methods compared to functions

### Fuzzing

- automatic structural fuzzing of everything is our only and main trick
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
- shells should guarantee that we can always use structural fuzzing

### Metaprogramming

- since we can run arbitrary code at compile time, this should be easy?

### FFI

- go all-in on WASM components?
  - built-in shells should be components
  - later, allow people to compose components themselves

### Optimizations

- can use special reference count that means "static" and avoid any ops
- can use biased reference counting
- can use small object headers that will transform to big ones (side tables) lazily
- can omit all atomic ops in a component if we know it is single-threaded
- guarantee that you can always read a full word if the address is aligned for faster copying?

### Ergonomics

- combinator chains should be avoided in favor of native control flow
- `string`, `option`, `result`, `error` ergonomics are fundamental
  - `option -> result + unwrap` pattern (`Option::ok_or/ok_or_else` in rust)
- it should be obvious what code to write (like in go)
  - decision paralysis of searching for the fanciest way to express something is bad (rust)
- shadowing for typestate-like things is convenient
- Go-like `Public` vs `private` is very convenient for reading the code

### Tooling

- built-in documentation generator (can embed runnable code so that people can play with it)

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
  - just make it a builtin?
- lexer/parser on top of generators
- generic `min`
- tree + tree algorithms and traversals
- slice (a container with required context?)
  - maybe builtin?

## TODO (small things)

### Miscellaneous

- logo (project + github social previews + opengraph + vscode language icon + vscode extension icon)
- look over old docs for interesting ideas for this document
