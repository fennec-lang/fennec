- comments

- top-level is const only, data only (special extension to mark this data-only thing?)
- top-level should be order-independent

- to maintain tower of languages, data language must not depend on real maps/vecs
- but then we need to introduce explicit casts to "real" types?
  - can't have simply `let x = [1, 2, 3]`
  - or we can, but what should be the interface types of stuff?
    - generic is too hard
    - non-generic -- what interface type to choose?
      - want to promote interoperability, need to have good defaults
      - no-std, what about it?
  - `f([1, 2, 3])` should just work
    - so, want a default implicit cast (coercion?)

- store pointers together with allocator reference?
  - should a countainer store an extra pointer?
    - very wasteful! vec/string becomes much bigger
    - but how not to forget the association?
  - how do we guarantee that a region is cleared on scope exit?
    - RAII, but only for regions? inelegant
  - generalize regions to scopes/lifetimes?
    - tie non-memory resources to the lifetimes
    - first-class lifetimes yes or no?

- map means values have to be equality-comparable
  - are all types equality-comparable?

- literals?
  - initializer lists ?
- const = compile time, can only use compile time?
- functions are const (comptime) by default

- any?
- bool
  - an enum?
  - implicitly imported from the `builtin` module
- integer
  - arbitrary precision
  -   hard, let's not
- float
  - arbitrary precision
    - hard, let's not
- tuple
- string
  - multiline
- sequence
  - homogenous?
  - slice?
    - good enough for 99%
- mapping
  - homogenous?
  - slice-of-pairs?
    - bad
    - what should be the default choice?
      - if hash_map, what should be the iteration order?

- records?
  - named-tuples
- enums?
  - bool

- named data, refer to data?
- abstract data vs concrete data? const vs let?
- how do we interact with abstract data from functions?
  - what if the abstract data includes concrete data?
- recursive data, what to do?

let version = "1.23"
let what = (version, version)

