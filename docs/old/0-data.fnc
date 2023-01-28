const a = 1
const b = 2
const c = (a, b)

// built-in generics: slice and pointer
// or no slice?
// smallvec, smallstring must be encodeable

// all types are implicitly parametrized by lifetimes?
// (all types with references are)

struct Slice[T any] @x {
	start &T @x
	size int
}

pub func (m *Memory) alloc[T any](n int) Slice[T] @m {
}

struct Pair[T] {
	first T
	second T
}

struct Ref[T] @x {
	ref &T @x
}

pub func make_ref[T any](r &T) Ref[T] @r {
}

pub func ref_pair[T any](r1 &T, r2 &T) Pair[&T @r1 @r2] {
	return Pair[&T] {
		first: r1,
		second: r1,
	}
}

pub func make2[T any](m *Memory, s1 Slice[T], s2 Slice[T]) Slice[Slice[T] @s1 @s2] @m {
	let s = m.alloc[Slice[T] @s1 @s2](2)
	s[0] = s1
	s[1] = s2
	return s
}


func main() {
	let x = LocalMem.alloc[int](3)
	let y = LocalMem.alloc[int](4)
	let z = make2(GlobalMem, x, y)
	let 
}


enum Bool {
	True
	False
}

struct Point {
	x int
	y int
}

struct Pair[T any] {
	first T
	second T
}

struct SliceWrap[T any] {
	s []any
	
}

