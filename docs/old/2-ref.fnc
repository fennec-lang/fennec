// memory regions are disjoint
// allocating creates an *owning* reference to a region
// RAII or just make it a compile error not to destroy the reference

// "higher RAII" effectively makes the destructor private,
// forcing to transfer ownership to a public function which can destroy the object
// (this way, we can e.g. pass additional information to the destructor)
// but, we can't simply unwind if we pass additional args

// "debug" (safe) pointers vs thin "release" pointers?

// allocated ref is a mutable borrow of the subpart of the region
// or is it just a box? you create a local value and transfer it to a box
// (which ties the lifetime to the memory region)
// box = part(whole)
// but what about creating an array?

// slices: built-in
// regions: built-in

// append: needs a region to reallocate into, must be the same as original
// single global region: no need to track?

// owned slice: fat, tracks region (don't want that, want external handles)
// borrowed slice: thin?
// or, thin pointer and external hashmap that remembers where the pointer is from
// (works bad with arenas?)
