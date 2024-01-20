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
- instant feedback and total observability
- fast interpreter/compiler for development, WASM/C/LLVM for production
- aggressively cut anything non-essential to vision
- aggressively minimize papercuts

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
- `fennec.mod` (like `go.mod`)
  - custom format to be able to associate a language server with it and properly report errors
- make package name always match the "directory" name (except for `v2`)
  - don't require `package foo` on top of every file
- `vendor` always exists, but can be ignored in source control (or not)
  - nudge people to be mindful of dependencies
  - makes modules more self-contained
- is including a `version` field to be independent of source control a bad idea?

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
- assertions are only checked in debug builds
  - a lot of invariants are too expensive to check always
  - don't want people to choose between different assertion kinds
  - since all code must be safe, missing assertions can't result in unsafety
  - when testing, we only check the assertions of module under test
    - faster and mirrors the release build semantics
- otherwise, `Result`

### Value semantics

- explicit copy and move
- all objects trivially moveable
  - what about mutex?
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
- `and`, `or`, `not`?
  - `and`, `or` to avoid confusion with `&` and `|`, but type checker should help with that
  - `not` to make reading code easier (ensuring `!` is used where it should can be hard)
- experiment with making borrow the default?
  - or even require to explicitly indicate every use, so that `a.b.c` is just a path
    - for maximum simplicity and consistency
    - this may help with eliminating `ref` and friends as well in `match`?
- is it possible to unify closures and functions (closure = function with non-empty capture list)?

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
- variable lifetime
  - default should be earliest possible drop, to avoid as many borrow problems as possible
  - special guard (type? construct? trait?) would force the lifetime to extend to the entire scope
    - rust doing this for anything that implements `Drop` is a mistake
  - [Explicit separation between liveness scope and referential scope](https://haibane-tenshi.github.io/rust-reborrowing/), maybe?
  - explicit re-borrow, including "shared that is guaranteed to be single, to exclusive"

### Synchronization

- can we unify channels and condvars in one primitive (state + lock + optional notification)?
  - channel can be seen as a specialization of condvar for `vec[T]` state
  - condvar can be seen as a generalization of channel for "arbitrary" (constrained) state
    - that can be inspected and modified (e.g. for cancellation)
  - [Go proposal to remove sync.Cond](https://github.com/golang/go/issues/21165)

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

- each enum variant must be a separate type (for flow-dependent typing and more)
- for pattern matching, swift-like syntax to refer to variants must be implemented
- fields must have rich metadata that specifies the relationship (part/helper/link/...)
  - we should be able to almost always derive `Eq`, `Debug` etc. consistent with the intended meaning
    - e.g. omit all caches, links
    - we should be able to show meaningful diff as well by default
  - we should be able to derive complex data generation as well?
    - type-based generation is a bad idea, but maybe if we have a rich "alias"/"layer" feature,
      we can create new lightweight interoperable views with desired semantics?

## Random ideas

### Testing

- liberal use of assertions in non-test code
  - allow to specify type invariants
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
  - we should never re-run manually when debugging; just add any required observability
    and everything is happening automatically
    - or simply time-travel back and expect any state without touching the code, that's the goal

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
- main thing for performance is transparency and predictability in what the compiler does
  - have a tree-walking interpreter as a reference
  - have a single destination-driven code generation pipeline, checked against the interpreter
  - have a way to write a portable vector version that is guaranteed to be identical to the scalar one
  - have a way to write a non-portable vector version that is guaranteed to be identical to the portable one
  - that's it; all people need is a sufficiently nice to use macro assembler
- can we have a language pattern that allows to write functions that can be used in 2 modes:
  - either return a new value (convenient), or
  - re-use the storage of the previous value (efficient)? so that it is convenient and transparent to the
    programmer, and is a default way of writing functions? avoid separate `append` class of APIs
- build on top of chunked primitives by default?
  - make `string`, `vec`  and builtin iteration constructs chunked
  - or have a fundamental "chunked stuff" type, maybe with virtual dispatch to get next chunk?
  - this is general enough so that e.g. gap buffers or ropes can be used everywhere instead of strings/vectors

### Ergonomics

- combinator chains should be avoided in favor of native control flow
  - however, can we really create an ergonomic replacement for the closure-argument pattern?
- `string`, `option`, `result`, `error` ergonomics are fundamental
  - `option -> result + unwrap` pattern (`Option::ok_or/ok_or_else` in rust)
- it should be obvious what code to write (like in go)
  - decision paralysis of searching for the fanciest way to express something is bad (rust)
- shadowing for typestate-like things is convenient
- Go-like `Public` vs `private` is very convenient for reading the code
- have a builtin `TODO`/`todo` which can fill any gap?
- consider treating `ctx` (zig-like) as fundamental building block
  - reification of lexical scoping
  - build scoped (structured) arena allocations, scoped (structured) concurrency around it
- `assert(x.is_some())` should change the type of `x` afterwards
- you don't debug a program, you debug a *problem*
  - find a problem by auto-testing, then click on it to start debugging, that's the whole flow
    - automatic problem finding (auto-test) + automatic problem debugging (auto-minify, auto-rerun, time travel)

### UI

- show the text of error in the status bar

### Tooling

- built-in documentation generator (can embed runnable code so that people can play with it)
- auto-render (in the IDE) and export (to HTML) content of e.g. `docs` folder, using `djot`
  - make the documentation built-in and automatic the same way we do it for testing
  - of course, the generated documentation must contain code examples that are runnable using WASM
  - isn't this really close to notebooks, in a sense?

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
