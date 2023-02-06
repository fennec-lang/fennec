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
  - incompatible with unwinding?
- no overloading
- no bound methods (just use a closure)
- default to strongly const (but no WxA; so have to pessimistically re-read const things)
- `require` + `assert` in prelude, distinguish preconditions and internal invariant violations
- think about `debug` assertions which check complex invariants but are off by default
- no null, no pointer arithmetic & casts, throw on overflow and OOB
- expose throw as a control flow construct
- model complex systems as single thread agents with epoll-like interface; maybe some structured concurrency later
  - explicit choice of what action out of all possible actions to take
- start with core fennec for language, and have minimum things built-in (rely on prelude instead)
- make sure all operators on all types follow the same rules (regularity, generic programming)
- full proper compile-time execution?
- want to have Go-like client-side abstraction, which should not require upfront design

## misc. questions

- can we disallow lifetime parameters on types/traits entirely?
  - maybe the loss will not be that great; need to consider this seriously
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
  - uniform brace-initialization for everything?
- generics
  - `any`/`dyn`/`impl` modes of use?
  - `impl`/`some` in argument vs return position
  - type members / associated types, do we want them?
- single abstraction thing (`interface` / `protocol`)

## values/pointers

- normal, `boxed`, `shared`, `weak` (maybe `unowned` as well?) kinds of values
  - can we generate `hash`, `eq` etc. automatically, only traversing the owned parts?
    - probably no; what about subslice, which shares slice ownership, and only owns the indices?
  - how to borrow from `weak`, do we have to have a built-in `optional`?
    - `as shared` -- a conversion that can fail, like `i64 as i8`
      - probably have to have a built-in optional (not in the core language, though)
      - we return `optional shared T`?
        - if `optional` is generic, then `shared T` can be generic type argument
      - `optional T` = shorthand for `T | nil`?
- `optional T` as `T?`: `shared T?` vs `shared (T?)` or `(shared T)?`
- `mutable` type modifier: dynamic checking of borrows
  - is `shared` always `mutable`?
    - it can be internally mutable! so no

- `&` + `&mut`: can not refer to kinds of values; only to types?
  `& shared T` sounds weird; how is it different from `shared &T`?
  - isn't this impossible, because we can always wrap any kind in a struct, and borrow it?
  - should we be able to `newtype` e.g. `shared T`?
- `&mutable T` is useful; can get `&mut parent.child` from `&parent` when `parent` is `mutable`

- do these things compose? `boxed boxed shared shared`?
  - don't want to
  - but how to limit this in generics? these should not be type constructors?
  - but if we can pass them to functions, and we can write generic functions, ...?
  - can abstract over `T`, but can't abstract over `boxed vs shared`
  - can we `newtype` a `shared T`? looks like not, but we can write a `struct` with single member
- want to be able to write special code when `rc == 1` (maybe a kind of pattern-match?)

## optional/result

- what is the `None`? is it `()`, or `nil`?
- result should a regular thing, probably `T | any error`
  - or should we go with Go-style interface values?
    - can start with "object-safe" interfaces, and go for full swift-style dynamism later
- want a single error type? probably no. but probably don't want go-style `error` interface
  - but we *do* want an opaque type, so that people don't know how to handle it by default
  - LEAF makes `result` a magical `optional` extension which stores error information out of band
- errors compose (wrap + multi-wrap); cast with `is`/`as`; then use methods to query stuff
- LEAF-like design?
  - but with inline handling as well
  - LEAF does not work with tree-shaped errors?
- **errors are values**, and are handled using the regular control flow
  - but some shortcuts can help make things a bit shorter (like the `?` operator)
- want a nice way for error augmentation (like LEAF does with `leaf::on_error`)
  - this is kind of like Zig's `errdefer`?
  - D `scope(exit)`/`scope(success)`/`scope(failure)`

## data types

- anything that is unnamed will match structurally
  - but only with other unnamed stuff (members!)
  - think tuples
- anything that is named will only match nominally (both top-level and members!)
- this nicely generalizes: we always match by name, but we can define "duplicate" things by re-using the empty name
