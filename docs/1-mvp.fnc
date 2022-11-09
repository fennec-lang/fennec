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
