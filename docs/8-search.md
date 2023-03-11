# search for simple you powerful enough

- lifetimes/regions are OK only if simple and intuitive
  - but how do we differ from rust?
    - no unsafe, period? focus on the higher level; all unsafe things must be in the stdlib
- without lifetimes, we are unsafe
  - hard to sell
  - hard to run in shared environment? or webassembly solves this problem?
  - hard to know what to audit or what to run under ubsan

- UB
  - use after free 
  - use of invalid reference (reference to changed data)
  - overflow/underflow
    - easy, detecting is cheap
  - race?
    - can specify, go-like?
    - detecting is kind of expensive

- simple lifetimes
  - lifetimes tied to "guard" things that enforce invariants vs lifetimes that freeze stuff
    - "conditional" lifetimes: do they require guards to be frozen?
      - no, this is just and "exist" obligation vs "immutable" and "unshared" obligations
        - immutable/unshared imply exist
      - should "raw" pointers require "exist" thing, too?
        - eh, probably not expressive enough
  - specify full context obligations!
    - can we split any function into sub-functions, are our annotations expressive enough?
  - lifetime = set of regions invalidating which would invalidate it
    - for guard, "imm" is probably good enough; no need to add "exists"
    - we can have an explicit syntax for lifetimes
      - when writing data structures, we can just introduce lifetime variables to capture an "abstract" set
        - when writing functions, we should almost always not need an explicit introduction of lifetime vars
        - hmm, maybe just stay the requirements of *references*, not lifetimes?

```
def Min(a &Matrix, b &Matrix) &[[lifetime(a) & lifetime(b]] Matrix {
}

def Min(a &Matrix, b &Matrix) &[[a & b]] Matrix {
}

def Min(a &Matrix, b &Matrix) [[a & b]] &Matrix  {
}

def Min(a &Matrix, b &Matrix) [[a & b]] &Matrix[[[foo]] &int]  {
}

def Min(a &Matrix, b &Matrix) <a & b> &Matrix[<foo> &int]  { // need an inline-introduce syntax; same for generics
}

def Min(a &Matrix, b &Matrix) <'x> &Matrix[<'y> &int]  
    where 'x = a & b
      and 'y = foo {
}

def Min(a &'T, b &'T) <'x> &'T[<'t> &int]

def Min[x, y, T](a &T, b &T) <x> &T[<t> &int]


type RefPair {
    first &int
    second &int
}

a := vector[RefPair]
```

- introduction of type parameter does not require to lift it to the top
- introduction of lifetime parameter does not require to lift it to the top
- possible?
- translate parameters to "associated types"
  - but parameters are chosen outside, by users
    - they can constrain a parameter, forming a subtype
- why people don't want to add lifetime parameters: they infect all signatures!
  - same for type parameters
    - but we want to discourage people for adding type parameters
    - same for lifetime parameters?
      - references are generic in lifetimes; don't want to punish people from using references

- generic function: it physically exists; it is a polymorphic function
- generic type: it physically exists, is a polymorphic type?

```
type <a> ref_wrapper[T] { // introduction of a lifetime parameter
   first <a> &T                 // reference to a lifetime parameter
}

type double_ref[T] {
  first &T   // introduces first lifetime
  second &T  // introduces second lifetime, just like function arguments
}

// nice definition; but how do we use such a type? we should optimize for use (in other types, in functions)
// MAYBE we can allow "inline" things in "inline" types (like tuples) -- they are quite like function parameters: completely public
// inline types = type expressions; there we can use inline introductions
// type declaration = we have to explicitly write out parameters

type <a> constrained_double_ref[T] {  // strictly less general (more constrained), however some functions might require this?
  first  <a> &T
  second <a> &T
}

// what functions may require contrained lifetimes?
// if we want to append both to a vector? but maybe just make a vector of a more general type?

func append_two_refs_to_vec(v &uniq vector[&int], first &int, second &int) { // ???
}

func append_two_refs_to_vec(v &uniq vector[<'a> &int], first <a> &int, second <a> &int) { // ???
}

func append_two_refs_to_vec(v &uniq vector[<'a> &'T], first <a> &T, second <a> &T)
func append_two_refs_to_vec(v &uniq vector['T], first T, second T)
func <a> append_two_refs_to_vec[T](v &uniq vector[<a> &T], first <a> &T, second <a> &T)
// really bad, we gain nothing (but do we require this for "full" form?
// BUT, here we can specify things at the point of use
// OR can we just synthisize the "full" form automatically? there is MORE information (about the "main" parameter and "derived" ones)


// for data structures, we want to explicitly introduce parameters upfront? this requires lifting reference things outwards;

// we can't "hide" references: when we do, we have to introduce reference parameters
// but when we contain a public reference, we don't have to introduce parameters

type <a, b: a> complex_ref[T] {
}

type <a, b: a> complex_ref[T] {
}

type <a, b> pair_of_refs {
case first: <a> &int
case second: <b> &float
}

// can this thing be inferred from this?

type pair_of_refs {
case first: &int
csae second: &float
}


```




