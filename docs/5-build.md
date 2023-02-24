# build the language

- `x T @foo`

- types
  - primitive (atomic) types
  - what to do with strings?
    - we want to refer to strings, but we can't have them primitive
      - we can have a builtin type that wraps a slice of bytes
  - types can be grouped (as fields) into structs (always inline)
    - or tuples (= unnamed struct with unnamed fields)
      - 1-element tuple == type itself?
    - or into arrays
      - is a slice a built-in types that generalizes reference to one to reference to several?
      - how is slice related to view types?
  - types can be alternated (as variants) into enums (inline and outline, 2 syntax variants)
    - common parts are shared/reused and reside in "parent" (enum itself)
      - this common part has a name
    - variants are always a distinct types, which can be separately used
    - can mark enum as open (force users to handle the unknown case)
  - describe value layout
- reference builds from empty to some subset of values
  - via view type: type + relative path expression (valid subpart)
- reference `hey T(.x, .y) @foo(.bar | .baz)`
  - `hey` refers to `x and y` of `T` located in `foo.bar` or in `foo.baz`
  - how to refer to some part of the array?
    - array = tuple = struct; an aggregate
    - we should be able to refer to subparts of them


- newtypes with coercions
  - true ADT; coercions are not exported

- context parameter everywhere
  - logger, temporary allocator, what else?

`... hey T(.x, .y) @foo(.bar | .baz) ...`

```
bool f32 f64
i8 i16 i32 i64
u8 u16 u32 u64

// nominal
type amout {
  f64
}

// structural
type amount (
  f64
)

type point {
  x i32
  y i32
}

type color {
  case red  // same thing as `case red()` or `case red{}` (empty payloads)
  case black
}

type color {
  case red = 1
  case black = 2
}

case type color {}
type color.red = 1 {}
type color.black = 2 {}

type option[T] {
  case some{T}
  case none
}

type with_const {
  const hello = "world" // what is the type?
}

// want to show that it can be extended (default to final)
// hmm is T a phantom type here?
case type option[T] {}

// must be the same as inline (only the syntax differs)
// can't be structural if parent is nominal? or structural here refers only to the derived part?
type option[T].some (
  T
)

// should be able to explicitly specify case number, even when the definition is not inline?
// or only for cases without payloads?

type option[T].none ()

// can type be used standalone or not? like `option[T]` without any payload
// no; if there is a need -- you can have an explicit "nothing" thing like `option[T].none`
```

- we want to be able to uniformly parametrize + `where` anything:
  - models
  - regular method groups (`impl` in rust)
    - rust: methods are defined in the context of type/trait/... (`Self` exists?)
      - different blocks can establish different context (require different things for generic parameters)

- rust slices:
  - `[foo]` is a DST; can't operate on it directly
  - `&[foo]` is a reference to DST; can operate on references
  - same for trait objects

- slices
  - can we get away without DSTs?
  - if we have rich references already, can we just have fat references and be done with it?
    - `hey []T(.x, .y) @foo(.bar | .baz)`
      - `[?]` really, with `?` stored inside the fat pointer
    - `hey [3]T(.x, .y) @foo(.bar | .baz)`
      - slice of size 3 or an array of size 3?
  - how to build a vector?
    - easy! it contains an *owning* slice; that's it
