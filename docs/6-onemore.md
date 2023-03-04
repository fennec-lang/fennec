# thoughts

- dynamic nature first, but with enough static information to enable efficient compilation
  - and predictable performance, so not JIT/GC
- if we excel at live stuff, we are automatically the best GUI language as well
  - so lean heavy into it; provide visualizations for all built-in and standard data structures
  - every program is GUI program

- misc
  - explicit copy + move (to disincentivise)
  - calling convention for `&T`: can do the copy, and then "take the refenrence" internally
    - goal: make `foo(&T)` the default and efficient calling convention, even for generics
  - provide different built-in assertions
    - one should mean "the input is bad" (fuzzer should avoid it)
    - the other means "invariant is broken" (fuzzer should search for it)
  - cloud-based fuzzer corpus sharing
  - should we provide built-in generators, to simplify writing the iterators?

- types
  - T (== `own T`), (T, T), `[2]T`, `[?]T` (built-in vector, DST)
    - require explicit `foo(own T)` in signatures
    - maaybe do `foo(T)` == `foo(&T)`?
  - `shared T`, `weak T`, `boxed T`, `mutable T`
  - `&T(view) @location`, `[]T(view) @location`
  - `uniq &T`, `uniq []T`

- data structures
  - unified `type` thing that allows both for struct and enum behavior (see 5-build.md)
  - parametrize structures by context instead of lifetimes (every borrow has an explicit source!)
    - ```
      type foo[x &bar] {
        first  &int @x.q
        second &int @x.p
      }```
  - extends naturally to self-referential structs

- generics
  - can always use: move/swap, borrow
  - most important traits: eq, cmp, hash, copy

- polymorphism
  - want only single way to write polymorphic code
    - Trait == set of types
  - `any Trait` + `some Trait`?
  - **vtable is fundamental**
    - start with vtable (dict-passing style); consider stenciling an optimization
      - only perform stenciling after we have ran non-stencil code for a bit
      - smooth progression: syntax checks, type checks, test runs with non-stencil code, test runs of real code
      - but guarantee that we perform full stenciling and only use dict passing where we have to
  - generic function exists as an entity, it is not some "template" that produces functions
    - that means that we of course can have traits with generic functions
  - non-`[]` syntax with "auto" types is the main syntax; `[]` is core-level thing to which we desugar to
    - right as we have smooth upgrades from enum to enum w/data and from struct to set of structs with common fields
    - we have a smooth upgrade path from function on concrete types to function on abstract types

- data-only subset
  - want guaranteed purity
  - want to use for config files like go.mod
  - want to use to include input in the tests source code!
  - want similarity to JSON?

- packages/modules/visibility
  - mostly copy Go
    - module as a unit of distribution (dependencies, string-based imports, internal stuff)
    - package as a unit of maintenance (a bunch of files with no encapsulation)
  - visibility: name-based or not?
    - let's just copy Go

- error handling
  - errors are values is a good idea
  - want automatic structural augmentation with "logical call stack"
  - don't want rich error types
  - don't want allocations (boxing) for errors?
    - LEAF idea is interesting but requires handling to be synchronous (we know exactly the handling context)
    - maybe use a simple temporary allocator to store the error info?
      - some small memory area will usually be enough as we don't have a lot of inflight errors
      - start allocating in regular heap once small area is full (should be rare)
  - assertions/panics exist but are not reflected in signatures

- TODO:
  - need to create a basic library that defines several important data structures and algorithms
    - min/max
    - filter/map
    - rng
    - hashing
    - option
    - vector
      - iterator?
    - list
      - iterator?
    - treemap/hashmap
      - iterator?
    - basic STL-like algorithms:
      - find
      - partition
      - sort
