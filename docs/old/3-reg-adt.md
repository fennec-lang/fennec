- allocator is a data structure, complex and almost magical
- why don't we have it built-in
- then rust solution of having a list with a `vec<node>` works great:
  we just use a `region<node>` instead, and that's it?
- allocator has some big global state, maybe that's why?
- but we can maybe magically make all our allocators point to the
  state of the global one? so no per-instance overhead

- probably have to use 2 key ideas: ghost-cell like thing to flexibly separate permissions and this region stuff
- but maybe ghost-cell is just a hack, and regions are good enough?
- or isn't a ghost-cell just a mutable borrow of our region, which was implicit in rust but is explicit here?

```rust
struct Node<T> {
  value: T,
  children: Range<usize>,
}
struct Tree<T> {
   nodes: Vec<Node<T>>,
}
```

- but how do we delete an element from a region? we have to prove that no pointer exists
- or we just don't delete stuff! region lives as a whole and dies as a whole?
- if we reuse data in the region, is this memory-safety or not?
  - looks like no, our program got confused

```rust
new_key_type! {
    pub struct ListKey;
}

#[derive(Copy, Clone)]
struct Node<T> {
    value: T,
    prev: ListKey,
    next: ListKey,
}

pub struct List<T> {
    sm: SlotMap<ListKey, Node<T>>,
    head: ListKey,
    tail: ListKey,
}
```
