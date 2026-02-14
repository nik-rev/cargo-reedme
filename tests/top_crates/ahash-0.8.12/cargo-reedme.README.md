AHash is a high performance keyed hash function.

It quickly provides a high quality hash where the result is not predictable without knowing the Key.
AHash works with `HashMap` to hash keys, but without allowing for the possibility that an malicious user can
induce a collision.

# How aHash works

When it is available aHash uses the hardware AES instructions to provide a keyed hash function.
When it is not, aHash falls back on a slightly slower alternative algorithm.

Because aHash does not have a fixed standard for its output, it is able to improve over time.
But this also means that different computers or computers using different versions of ahash may observe different
hash values for the same input.
# Basic Usage
AHash provides an implementation of the [Hasher] trait.
To construct a HashMap using aHash as its hasher do the following:
```rust
use ahash::{AHasher, RandomState};
use std::collections::HashMap;

let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
map.insert(12, 34);
```

### Randomness

The above requires a source of randomness to generate keys for the hashmap. By default this obtained from the OS.
It is also possible to have randomness supplied via the `compile-time-rng` flag, or manually.

### If randomness is not available

[AHasher::default()] can be used to hash using fixed keys. This works with
[BuildHasherDefault](https://doc.rust-lang.org/stable/core/hash/struct.BuildHasherDefault.html). For example:

```rust
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
use ahash::AHasher;

let mut m: HashMap<_, _, BuildHasherDefault<AHasher>> = HashMap::default();
```
It is also possible to instantiate [RandomState](https://docs.rs/ahash/0.8.12/ahash/random_state/struct.RandomState.html) directly:

```rust
use ahash::HashMap;
use ahash::RandomState;

let mut m = HashMap::with_hasher(RandomState::with_seed(42));
```
Or for uses besides a hashhmap:
```rust
use std::hash::BuildHasher;
use ahash::RandomState;

let hash_builder = RandomState::with_seed(42);
let hash = hash_builder.hash_one("Some Data");
```
There are several constructors for [RandomState](https://docs.rs/ahash/0.8.12/ahash/random_state/struct.RandomState.html) with different ways to supply seeds.

# Convenience wrappers

For convenience, both new-type wrappers and type aliases are provided.

The new type wrappers are called called `AHashMap` and `AHashSet`.
```rust
use ahash::AHashMap;

let mut map: AHashMap<i32, i32> = AHashMap::new();
map.insert(12, 34);
```
This avoids the need to type “RandomState”. (For convenience `From`, `Into`, and `Deref` are provided).

# Aliases

For even less typing and better interop with existing libraries (such as rayon) which require a `std::collection::HashMap` ,
the type aliases [HashMap](https://docs.rs/ahash/0.8.12/ahash/type.HashMap.html), [HashSet](https://docs.rs/ahash/0.8.12/ahash/type.HashSet.html) are provided.

```rust
use ahash::{HashMap, HashMapExt};

let mut map: HashMap<i32, i32> = HashMap::new();
map.insert(12, 34);
```
Note the import of [HashMapExt](https://docs.rs/ahash/0.8.12/ahash/trait.HashMapExt.html). This is needed for the constructor.