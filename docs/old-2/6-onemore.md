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
    - shared ownership
    - mutable things:
      - what does `m.borrow()` return? it has to have a dtor, but we want it to return a normal reference!
        - `borrow() -> (guard, &T <must be tied to guard>)`
          - `@location` is not enough :-(
      - if it is a builtin type, we can just give a borrow a magical semantic that creates a witness object tied to it
        - it can have exactly the same "begin/end" points (NLL and all that) that regular borrows have
          - may be too high overhead
  - `&T(view) @location`, `[]T(view) @location`
    - shared reference
  - `uniq &T`, `uniq []T`
  - unify tuples and structs?
  - we want raw pointers
    - they should be mutable, too? then we have `*mut T` vs `*uniq T` for no reason
      - also, we have an overlap with `mutable T`
      - implementing `mutable`, can we just have a "shadow" for all values, where we can at runtime track borrow state?
        - this way `mutable T` has the same layout as `T`, and we can drop it altogether, and make this just a runtime check?
          - that sometimes can be elided when we can prove things statically
      - `shadow state` is a very powerful idea for adding all kind of non-intrusive checks
    - separate "uniq reference" thing, so that we can express `uniq const` (why?) and `uniq mut`?

- data structures
  - unified `type` thing that allows both for struct and enum behavior (see 5-build.md)
  - parametrize structures by context instead of lifetimes (every borrow has an explicit source!)
    - ```
      type foo[x &bar] {
        first  &int @x.q
        second &int @x.p
      }```
  - extends naturally to self-referential structs
  - hm, how do we define an empty enum?
    - we don't, and have a special built-in type instead?

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
  - can also use this for attribute syntax?

- packages/modules/visibility
  - mostly copy Go
    - module as a unit of distribution (dependencies, string-based imports, internal stuff)
    - package as a unit of maintenance (a bunch of files with no encapsulation)
  - visibility: name-based or not?
    - let's just copy Go
      - one problem: public/private status of conformances (and other unnamed things)
        - rust: The visibility of a trait implementation is the intersection of the visibility of the trait and the visibility of the type it's implemented on

- newtypes/coercions
  - want to prevent (Map Int T) -> (Map Age T) (this will break the invariant)
    - haskell uses type roles for this (nominal/phantom/representational). key is nominal
      - but what about hashmap, should it be nominal as well?
        - hash relates to eq; if eq is different than semantics are different; so nominal too
          - so we can just require bounding in type definition; if the parameter is bound -- it is nominal
      - parameters of type classes are nominal by default, to prevent wrong impl from being called
        - well if everything is a type class in fennec, how do we get representational things anywhere?
          - maybe have some built-in traits magical?
  - we may want newtypes for alternative impls
    - in python, we would not be writing this shit?
      - no, we would, in OOP fashion
  - but newtypes is type-heavy programming; can we avoid it?
  - general rule when can coerce:
    - if we could write the less efficient code by hand with the same semantics -- we can coerce!
  - safety invariant: we can coerce when the type is in the "free" position

- error handling
  - errors are values is a good idea
  - want automatic structural augmentation with "logical call stack"
  - don't want rich error types
  - don't want allocations (boxing) for errors?
    - LEAF idea is interesting but requires handling to be synchronous (we know exactly the handling context)
    - maybe use a simple temporary allocator to store the error info?
      - some small memory area will usually be enough as we don't have a lot of inflight errors
      - start allocating in regular heap once small area is full (should be rare)
      - a damn good idea
  - assertions/panics exist but are not reflected in signatures

