`static` friendly data structures that don’t require dynamic memory allocation

The core principle behind `heapless` is that its data structures are backed by a *static* memory
allocation. For example, you can think of `heapless::Vec` as an alternative version of
`std::Vec` with fixed capacity and that can’t be re-allocated on the fly (e.g. via `push`).

All `heapless` data structures store their memory allocation *inline* and specify their capacity
via their type parameter `N`. This means that you can instantiate a `heapless` data structure on
the stack, in a `static` variable, or even in the heap.

```rust
use heapless::Vec; // fixed capacity `std::Vec`

// on the stack
let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
xs.push(42).unwrap();
assert_eq!(xs.pop(), Some(42));

// in a `static` variable
static mut XS: Vec<u8, 8> = Vec::new();

let xs = unsafe { &mut XS };

xs.push(42);
assert_eq!(xs.pop(), Some(42));

// in the heap (though kind of pointless because no reallocation)
let mut ys: Box<Vec<u8, 8>> = Box::new(Vec::new());
ys.push(42).unwrap();
assert_eq!(ys.pop(), Some(42));
```

Because they have fixed capacity `heapless` data structures don’t implicitly reallocate. This
means that operations like `heapless::Vec.push` are *truly* constant time rather than amortized
constant time with potentially unbounded (depends on the allocator) worst case execution time
(which is bad / unacceptable for hard real time applications).

`heapless` data structures don’t use a memory allocator which means no risk of an uncatchable
Out Of Memory (OOM) condition while performing operations on them. It’s certainly possible to
run out of capacity while growing `heapless` data structures, but the API lets you handle this
possibility by returning a `Result` on operations that may exhaust the capacity of the data
structure.

List of currently implemented data structures:

- [`BinaryHeap`](https://docs.rs/heapless/0.8.0/heapless/binary_heap/struct.BinaryHeap.html) – priority queue
- [`IndexMap`](https://docs.rs/heapless/0.8.0/heapless/indexmap/struct.IndexMap.html) – hash table
- [`IndexSet`](https://docs.rs/heapless/0.8.0/heapless/indexset/struct.IndexSet.html) – hash set
- [`LinearMap`](https://docs.rs/heapless/0.8.0/heapless/linear_map/struct.LinearMap.html)
- [`String`](https://docs.rs/heapless/0.8.0/heapless/string/struct.String.html)
- [`Vec`](https://docs.rs/heapless/0.8.0/heapless/vec/struct.Vec.html)
- [`mpmc::Q*`](https://docs.rs/heapless/0.8.0/heapless/mpmc/) – multiple producer multiple consumer lock-free queue
- [`spsc::Queue`](https://docs.rs/heapless/0.8.0/heapless/spsc/struct.Queue.html) – single producer single consumer lock-free queue

# Optional Features

The `heapless` crate provides the following optional Cargo features:

- `ufmt`: Implement [`ufmt_write::uWrite`] for `String<N>` and `Vec<u8, N>`

[`ufmt_write::uWrite`]: https://docs.rs/ufmt-write/

# Minimum Supported Rust Version (MSRV)

This crate does *not* have a Minimum Supported Rust Version (MSRV) and may make use of language
features and API in the standard library available in the latest stable Rust version.

In other words, changes in the Rust version requirement of this crate are not considered semver
breaking change and may occur in patch version releases.