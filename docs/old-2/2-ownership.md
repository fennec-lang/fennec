# ownership

- regular, `boxed`, `shared`, `weak` (`unowned`?)
- `mutable`
- easy self-references? `self_cell` etc.
  - my no-lifetimes approach seems simple and easy here?
    - a simple form of dependent types? we parametrize by places

```
type Foo[a &X] struct {  // &X == &(?) X
  x &(a.first) int
  b &(a.second) int
}

func foo[a &X](f Foo[a]) &(f) int {
  return f.x
}

func foo(a &X, f Foo[a]) &(f) int {
  return f.y
}

type SelfRef struct {
  x vector[int]
  b int @x
  c f64 @x
}
```

- make `T` mean `&T` (or `T` for copy types), `mut T` mean `&mut T` (reference semantics by default)
  - replace `T` with `owned T` (or `new T`)
  - don't want to write `owned` in structs?
- functions operate on places (references) by default
  - struct default is `@new`, (or `@own`?) func default is `@auto`
    - in-place placement of `@new` is just an optimization!
- struct store values by default

```
func min[T](a T, b T) T {
  // TODO
}

func min[T](a T, b T) T@(a | b) {
  // TODO
}

func minCopy[T](a T, b T) T @own {
  // TODO
}

func minOut[T](a T, b T, out mut T) {
  // TODO
}
```

- structs define logical layout of places (whole/part etc.)
- functions operate on places, describing what do they do
- `x T @foo`: x = value (reference) name, T = value type, @foo = value place
  - what about `x mut T @foo`

```
func (x Foo @auto) borrowWhatIsRequired() {  // compute precise borrow set automatically; can't be public?

}
```

- maybe `@auto` should be the default? or should it be `@subparts`?
  - expand to e.g. `@(.foo | .bar)`
  - we have a partial reference, which is distinct that a full reference to somewhere
    - `func foo(x bar.auto)`? `func foo(x bar.(x | y))`
  - but regular reference already referes to subparts!
- `x T @foo`: x = reference name, `T` = to what (subparts!) `@foo` = where
- want `x.auto` default for functions? BUT this is bad for public symbol evolution
  - but then we have different meanings for `x T` in public and private stuff?
  - can be explicit about stability, `func` vs `proc` or whatever

