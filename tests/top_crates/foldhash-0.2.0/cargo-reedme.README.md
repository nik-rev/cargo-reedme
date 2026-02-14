This crate provides foldhash, a fast, non-cryptographic, minimally
DoS-resistant hashing algorithm designed for computational uses such as
hashmaps, bloom filters, count sketching, etc.

When should you **not** use foldhash:

- You are afraid of people studying your long-running program’s behavior
  to reverse engineer its internal random state and using this knowledge to
  create many colliding inputs for computational complexity attacks.

- You expect foldhash to have a consistent output across versions or
  platforms, such as for persistent file formats or communication protocols.
   
- You are relying on foldhash’s properties for any kind of security.
  Foldhash is **not appropriate for any cryptographic purpose**.

Foldhash has two variants, one optimized for speed which is ideal for data
structures such as hash maps and bloom filters, and one optimized for
statistical quality which is ideal for algorithms such as
[HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog) and
[MinHash](https://en.wikipedia.org/wiki/MinHash).

Foldhash can be used in a `#![no_std]` environment by disabling its default
`"std"` feature.

# Usage

The easiest way to use this crate with the standard library [`HashMap`](https://docs.rs/foldhash/0.2.0/foldhash/convenience/type.HashMap.html) or
[`HashSet`](https://docs.rs/foldhash/0.2.0/foldhash/convenience/type.HashSet.html) is to import them from `foldhash` instead, along with the
extension traits to make [`HashMap::new`] and [`HashMap::with_capacity`]
work out-of-the-box:

```rust
use foldhash::{HashMap, HashMapExt};

let mut hm = HashMap::new();
hm.insert(42, "hello");
```

You can also avoid the convenience types and do it manually by initializing
a [`RandomState`](https://docs.rs/foldhash/0.2.0/foldhash/fast/struct.RandomState.html), for example if you are using a different hash map
implementation like [`hashbrown`](https://docs.rs/hashbrown/):

```rust
use hashbrown::HashMap;
use foldhash::fast::RandomState;

let mut hm = HashMap::with_hasher(RandomState::default());
hm.insert("foo", "bar");
```

The above methods are the recommended way to use foldhash, which will
automatically generate a randomly generated hasher instance for you. If you
absolutely must have determinism you can use [`FixedState`](https://docs.rs/foldhash/0.2.0/foldhash/fast/struct.FixedState.html)
instead, but note that this makes you trivially vulnerable to HashDoS
attacks and might lead to quadratic runtime when moving data from one
hashmap/set into another:

```rust
use std::collections::HashSet;
use foldhash::fast::FixedState;

let mut hm = HashSet::with_hasher(FixedState::with_seed(42));
hm.insert([1, 10, 100]);
```

If you rely on statistical properties of the hash for the correctness of
your algorithm, such as in [HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog),
it is suggested to use the [`RandomState`](https://docs.rs/foldhash/0.2.0/foldhash/quality/struct.RandomState.html)
or [`FixedState`](https://docs.rs/foldhash/0.2.0/foldhash/quality/struct.FixedState.html) from the [`quality`](https://docs.rs/foldhash/0.2.0/foldhash/quality/) module instead
of the [`fast`](https://docs.rs/foldhash/0.2.0/foldhash/fast/) module. The latter is optimized purely for speed in hash
tables and has known statistical imperfections.

Finally, you can also directly use the [`RandomState`](https://docs.rs/foldhash/0.2.0/foldhash/quality/struct.RandomState.html)
or [`FixedState`](https://docs.rs/foldhash/0.2.0/foldhash/quality/struct.FixedState.html) to manually hash items using the
[`BuildHasher`](https://doc.rust-lang.org/stable/core/hash/trait.BuildHasher.html) trait:
```rust
use std::hash::BuildHasher;
use foldhash::quality::RandomState;

let random_state = RandomState::default();
let hash = random_state.hash_one("hello world");
```

## Seeding

Foldhash relies on a single 8-byte per-hasher seed which should be ideally
be different from each instance to instance, and also a larger
[`SharedSeed`](https://docs.rs/foldhash/0.2.0/foldhash/seed/struct.SharedSeed.html) which may be shared by many different instances.

To reduce overhead, this [`SharedSeed`](https://docs.rs/foldhash/0.2.0/foldhash/seed/struct.SharedSeed.html) is typically initialized once and
stored. To prevent each hashmap unnecessarily containing a reference to this
value there are three kinds of [`BuildHasher`](https://doc.rust-lang.org/stable/core/hash/trait.BuildHasher.html)s
foldhash provides (both for [`fast`](https://docs.rs/foldhash/0.2.0/foldhash/fast/) and [`quality`](https://docs.rs/foldhash/0.2.0/foldhash/quality/)):

1. [`RandomState`](https://docs.rs/foldhash/0.2.0/foldhash/fast/struct.RandomState.html), which always generates a
   random per-hasher seed and implicitly stores a reference to [`SharedSeed::global_random`](https://docs.rs/foldhash/0.2.0/foldhash/seed/struct.SharedSeed.html#method.global_random).
2. [`FixedState`](https://docs.rs/foldhash/0.2.0/foldhash/fast/struct.FixedState.html), which by default uses a fixed
   per-hasher seed and implicitly stores a reference to [`SharedSeed::global_fixed`](https://docs.rs/foldhash/0.2.0/foldhash/seed/struct.SharedSeed.html#method.global_fixed).
3. [`SeedableRandomState`](https://docs.rs/foldhash/0.2.0/foldhash/fast/struct.SeedableRandomState.html), which works like
   [`RandomState`](https://docs.rs/foldhash/0.2.0/foldhash/fast/struct.RandomState.html) by default but can be seeded in any manner.
   This state must include an explicit reference to a [`SharedSeed`](https://docs.rs/foldhash/0.2.0/foldhash/seed/struct.SharedSeed.html), and thus
   this struct is 16 bytes as opposed to just 8 bytes for the previous two.

## Features

This crate has the following features:
- `nightly`, this feature improves string hashing performance
slightly using the nightly-only Rust feature
[`hasher_prefixfree_extras`](https://github.com/rust-lang/rust/issues/96762),
- `std`, this enabled-by-default feature offers convenient aliases for `std`
containers, but can be turned off for `#![no_std]` crates.