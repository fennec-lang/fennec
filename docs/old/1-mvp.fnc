// types are parametrized by regions
// regions are proofs of validity and existence
// shared xor mutable is good

// all types are byte-moveable

// region is a slice!
// like a real physical slice[t]
// T can have fields, OR have variants
// region is an addressable place (which may conditionally exist!)
// conditionally: enums OR flow sensitivity

// lifetimes are wrong, but:
// what is a lifetime, relative to a region?
// how do we abstract over regions?
// are lifetimes sets of "same duration" regions?

// you can broaden the regions: from part to whole
// you can OR regions: here OR there
// you can AND regions

// MVS: whole-part fundamental (always), + yield to access subpart
// semantic uniformity required for generic programming
// references as not first-class (limiting and sad)

struct foo(arena, subarena)[T] {
	ptr1 &(arena & subarena)T
	ptr2 &(arena | subarena)T
}

// functions can not be parametrised by lifetimes
// maybe no lifetime parameters for types as well? definitions don't constraint lifetimes of references

struct foo[T] {
    ptr1 &T
    ptr2 &T
}

// how does a user use `foo` without knowing its definition?
// will we have to add some more contraints to functions instead?

// we should be allowed to write ref[T] that is as good as &T
// how does a user know that ref[T] has an associated lifetime?

// just make everything in the type public for inspection / type checking (even private members)
// so no "encapsulation" from the type checker or lifetime checker
// what about mutability bits? do we "propagate" them magically or do we duplicate code?
// probably can't propagate when not every non-mut type can be transformed to mut type

struct complex {
    c container[int]
    p &(container) int
}

struct external_dep {
   nodes node_storage(arena)
   // todo: list of nodes
}

// but everything is parametrised by an allocation arena?
// so everything non-inline is a reference type, parametrised by the thing that contains the data
// so every time we have 2 non-inline types, we have to question do they point to the same thing or not
// and how do we express a vector from one region, containing vectors from another region?
// how do we express a pair of vectors, both containing things from 3rd (same) region?

// we have to fundamentally define whole/part relationship, and also *disjoint* nature of wholes (!)

// how do we constrain generic parameters of types in respect to lifetimes?
// we can't do this in the function (method) signatures

struct foo['arena, 'subarena, T] {
	ptr1 &'(arena & subarena)T
	ptr2 &(arena | subarena)T
}

&(arena)T == ref(arena)[T]

&(arena) mut T
foo(arena)[foo(arena)[int]] // can't distinguish from a function call in parser?
