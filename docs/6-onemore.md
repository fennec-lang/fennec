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
