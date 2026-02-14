Small vectors in various sizes. These store a certain number of elements inline, and fall back
to the heap for larger allocations.  This can be a useful optimization for improving cache
locality and reducing allocator traffic for workloads that fit within the inline buffer.

## `no_std` support

By default, `smallvec` does not depend on `std`.  However, the optional
`write` feature implements the `std::io::Write` trait for vectors of `u8`.
When this feature is enabled, `smallvec` depends on `std`.

## Optional features

### `serde`

When this optional dependency is enabled, `SmallVec` implements the `serde::Serialize` and
`serde::Deserialize` traits.

### `write`

When this feature is enabled, `SmallVec<[u8; _]>` implements the `std::io::Write` trait.
This feature is not compatible with `#![no_std]` programs.

### `union`

**This feature requires Rust 1.49.**

When the `union` feature is enabled `smallvec` will track its state (inline or spilled)
without the use of an enum tag, reducing the size of the `smallvec` by one machine word.
This means that there is potentially no space overhead compared to `Vec`.
Note that `smallvec` can still be larger than `Vec` if the inline buffer is larger than two
machine words.

To use this feature add `features = ["union"]` in the `smallvec` section of Cargo.toml.
Note that this feature requires Rust 1.49.

Tracking issue: [rust-lang/rust#55149](https://github.com/rust-lang/rust/issues/55149)

### `const_generics`

**This feature requires Rust 1.51.**

When this feature is enabled, `SmallVec` works with any arrays of any size, not just a fixed
list of sizes.

### `const_new`

**This feature requires Rust 1.51.**

This feature exposes the functions [`SmallVec::new_const`](https://docs.rs/smallvec/1.15.1/smallvec/struct.SmallVec.html#method.new_const), [`SmallVec::from_const`](https://docs.rs/smallvec/1.15.1/smallvec/struct.SmallVec.html#method.from_const), and [`smallvec_inline`](https://docs.rs/smallvec/1.15.1/smallvec/macro.smallvec_inline.html) which enables the `SmallVec` to be initialized from a const context.
For details, see the
[Rust Reference](https://doc.rust-lang.org/reference/const_eval.html#const-functions).

### `drain_filter`

**This feature is unstable.** It may change to match the unstable `drain_filter` method in libstd.

Enables the `drain_filter` method, which produces an iterator that calls a user-provided
closure to determine which elements of the vector to remove and yield from the iterator.

### `drain_keep_rest`

**This feature is unstable.** It may change to match the unstable `drain_keep_rest` method in libstd.

Enables the `DrainFilter::keep_rest` method.

### `specialization`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

When this feature is enabled, `SmallVec::from(slice)` has improved performance for slices
of `Copy` types.  (Without this feature, you can use `SmallVec::from_slice` to get optimal
performance for `Copy` types.)

Tracking issue: [rust-lang/rust#31844](https://github.com/rust-lang/rust/issues/31844)

### `may_dangle`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

This feature makes the Rust compiler less strict about use of vectors that contain borrowed
references. For details, see the
[Rustonomicon](https://doc.rust-lang.org/1.42.0/nomicon/dropck.html#an-escape-hatch).

Tracking issue: [rust-lang/rust#34761](https://github.com/rust-lang/rust/issues/34761)