# fennec

- simple C/Go/Rust inspired core
- no GC, no runtime (or lightweight one)
- aim for Go-like speed, maybe a bit faster
- always live, always runnable, even with syntax or type errors
- complete and total determinism of execution
- always fuzzing (white-box) in the background, update fuzzing corpus (in the source code form) automatically
- no complex proof-like safety systems, all-in on fuzzing to found any problems
- built-in sanitizers (ASAN etc.); fat (unspecified size) pointers/references to assist in checking
- no implicit environment of any kind, capability-secure APIs by default
- generic from day 1 (but very simple generics)
- explicit object lifetime (create/destroy/copy/move), tied to scopes
- no invisible code (destructors); `defer` instead (destructors with effects are possible)
- no overloading
- no bound methods (just use a closure)
- default to strongly const (but no WxA; so have to pessimistically re-read const things)
- `require` + `assert` in prelude, distinguish preconditions and internal invariant violations
- think about `debug` assertions which check complex invariants but are off by default
- no null, no pointer arithmetic & casts, throw on overflow and OOB
- expose throw as a control flow construct
- model complex systems as single thread agents with epoll-like interface; maybe some structured concurrency later
- start with core fennec for language, and have minimum things built-in (rely on prelude instead)
- make sure all operators on all types follow the same rules (regularity, generic programming)

## misc. questions

- built-in flexible arrays and DSTs? with linked size field nearby
  - probably little need, since we can define a vector (or even `smallvec`!)
- have a default global allocator or not? `any` etc. require it, kind of
- do we require explicit control flow for throws?
  - assert can be everywhere, so everything can throw?
- library evolution things? don't make authors commit to something by accident
- ability to have non-exhaustive enums?
  - `@unknown default` case
- warn on unused results, unless function is annotated with `allow unused result`
- swift-like `.foo` implicit name resolution
  - swift type-checks entire statement at once
  - maybe a compromise like `_::foo`? ugly
- swift CoW collections for cheap value semantics everywhere
- how to pass parameters? want to avoid raw pointers etc.
  - but `inout` etc. are not first-class things, is this good or not?
  - good; we are sure no aliasing is possible
- actors == classes? should they exist separately?
- generic (or abstract) code should be discouraged precisely because "Postel's law" leads to bad and fragile systems
  - being strict about what you accept forces uniformity in callers, and in overall system design
- rename shallow copy to `alias`?
- errors (sean parent talk)
  - function domain = set of values satisfying preconditions
  - errors are about postconditions (which can not be satisfied when preconditions hold)
  - errors are recoverable (oom, i/o, cancallation)

## core

- structs, enums
- generics
  - `any`/`dyn`/`impl` modes of use?
  - `impl`/`some` in argument vs return position
  - type members / associated types, do we want them?
- single abstraction thing (`interface` / `protocol`)
