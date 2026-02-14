Pre-allocated storage for a uniform data type.

`Slab` provides pre-allocated storage for a single data type. If many values
of a single type are being allocated, it can be more efficient to
pre-allocate the necessary storage. Since the size of the type is uniform,
memory fragmentation can be avoided. Storing, clearing, and lookup
operations become very cheap.

While `Slab` may look like other Rust collections, it is not intended to be
used as a general purpose collection. The primary difference between `Slab`
and `Vec` is that `Slab` returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a
value associated with a given key is removed from a slab, that key may be
returned from future calls to `insert`.

# Performance notes

Methods that remove values and return them, such as [`Slab::remove`](https://docs.rs/slab/0.4.12/slab/struct.Slab.html#method.remove) and
[`Slab::try_remove`](https://docs.rs/slab/0.4.12/slab/struct.Slab.html#method.try_remove), might copy the removed values to the stack even if
their return values are unused. For types that don’t have drop glue, the
compiler can usually elide these copies.

# Examples

Basic storing and retrieval.

```rust
let mut slab = Slab::new();

let hello = slab.insert("hello");
let world = slab.insert("world");

assert_eq!(slab[hello], "hello");
assert_eq!(slab[world], "world");

slab[world] = "earth";
assert_eq!(slab[world], "earth");
```

Sometimes it is useful to be able to associate the key with the value being
inserted in the slab. This can be done with the `vacant_entry` API as such:

```rust
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

It is generally a good idea to specify the desired capacity of a slab at
creation time. Note that `Slab` will grow the internal capacity when
attempting to insert a new value once the existing capacity has been reached.
To avoid this, add a check.

```rust
let mut slab = Slab::with_capacity(1024);

// ... use the slab

if slab.len() == slab.capacity() {
    panic!("slab full");
}

slab.insert("the slab is not at capacity yet");
```

# Capacity and reallocation

The capacity of a slab is the amount of space allocated for any future
values that will be inserted in the slab. This is not to be confused with
the *length* of the slab, which specifies the number of actual values
currently being inserted. If a slab’s length is equal to its capacity, the
next value inserted into the slab will require growing the slab by
reallocating.

For example, a slab with capacity 10 and length 0 would be an empty slab
with space for 10 more stored values. Storing 10 or fewer elements into the
slab will not change its capacity or cause reallocation to occur. However,
if the slab length is increased to 11 (due to another `insert`), it will
have to reallocate, which can be slow. For this reason, it is recommended to
use [`Slab::with_capacity`] whenever possible to specify how many values the
slab is expected to store.

# Implementation

`Slab` is backed by a `Vec` of slots. Each slot is either occupied or
vacant. `Slab` maintains a stack of vacant slots using a linked list. To
find a vacant slot, the stack is popped. When a slot is released, it is
pushed onto the stack.

If there are no more available slots in the stack, then `Vec::reserve(1)` is
called and a new slot is created.

[`Slab::with_capacity`]: struct.Slab.html#with_capacity