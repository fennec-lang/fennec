# TODO

some things should be decided later, to not block the development

- newtype
  - need newtype to be able to provide an alternative models
  - need newtype to hide operations on existing type (`int`)
  - problem: need to design a coerce thing for newtype to be ergonomic

- explore "Algebraic Subtyping" approach for type-checking programs with inferred regions which can be unioned and intersected

- code examples:
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
