32-bit hashing algorithms

# Why?

Because 32-bit architectures are a thing (e.g. ARM Cortex-M) and you don’t want your hashing
function to pull in a bunch of slow 64-bit compiler intrinsics (software implementations of
64-bit operations).

# Relationship to `core::hash`

This crate extends [`core::hash`] with a 32-bit version of `Hasher`, which extends
`core::hash::Hasher`. It requires that the hasher only performs 32-bit operations when computing
the hash, and adds [`finish32`] to get the hasher’s result as a `u32`. The standard `finish`
method should just zero-extend this result.

Since it extends `core::hash::Hasher`, `Hasher` can be used with any type which implements the
standard `Hash` trait.

This crate also adds a version of `BuildHasherDefault` with a const constructor, to work around
the `core` version’s lack of one.

[`core::hash`]: https://doc.rust-lang.org/std/hash/index.html
[`finish32`]: https://docs.rs/hash32/0.3.1/hash32/trait.Hasher.html#tymethod.finish32

# Hashers

This crate provides implementations of the following 32-bit hashing algorithms:

- [Fowler-Noll-Vo](struct.FnvHasher.html)
- [MurmurHash3](struct.Murmur3Hasher.html)

# Generic code

In generic code, the trait bound `H: core::hash::Hasher` accepts *both* 64-bit hashers like
`std::collections::hash_map::DefaultHasher`; and 32-bit hashers like the ones defined in this
crate (`hash32::FnvHasher` and `hash32::Murmur3Hasher`)

The trait bound `H: hash32::Hasher` is *more* restrictive as it only accepts 32-bit hashers.

The `BuildHasherDefault<H>` type implements the `core::hash::BuildHasher` trait so it can
construct both 32-bit and 64-bit hashers. To constrain the type to only produce 32-bit hasher
you can add the trait bound `H::Hasher: hash32::Hasher`

# MSRV

This crate is guaranteed to compile on latest stable Rust. It *might* compile on older
versions but that may change in any new patch release.