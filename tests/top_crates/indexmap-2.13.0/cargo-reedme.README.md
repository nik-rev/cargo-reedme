[`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) is a hash table where the iteration order of the key-value
pairs is independent of the hash values of the keys.

[`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html) is a corresponding hash set using the same implementation and
with similar properties.

### Highlights

[`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html) are drop-in compatible with the std `HashMap`
and `HashSet`, but they also have some features of note:

- The ordering semantics (see their documentation for details)
- Sorting methods and the [IndexMap::pop](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html#method.pop) methods.
- The [`Equivalent`](https://docs.rs/equivalent/latest/equivalent/trait.Equivalent.html) trait, which offers more flexible equality definitions
  between borrowed and owned versions of keys.
- The [map::MutableKeys](https://docs.rs/indexmap/2.13.0/indexmap/map/mutable/trait.MutableKeys.html) trait, which gives opt-in mutable
  access to map keys, and [set::MutableValues](https://docs.rs/indexmap/2.13.0/indexmap/set/mutable/trait.MutableValues.html) for sets.

### Feature Flags

To reduce the amount of compiled code in the crate by default, certain
features are gated behind [feature flags]. These allow you to opt in to (or
out of) functionality. Below is a list of the features available in this
crate.

* `std`: Enables features which require the Rust standard library. For more
  information see the section on [`no_std`].
* `rayon`: Enables parallel iteration and other parallel methods.
* `serde`: Adds implementations for [`Serialize`] and [`Deserialize`]
  to [`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html). Alternative implementations for
  (de)serializing [`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) as an ordered sequence are available in the
  [`map::serde_seq`](https://docs.rs/indexmap/2.13.0/indexmap/map/serde_seq/) module.
* `arbitrary`: Adds implementations for the [`arbitrary::Arbitrary`] trait
  to [`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html).
* `quickcheck`: Adds implementations for the [`quickcheck::Arbitrary`] trait
  to [`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html).
* `borsh` (**deprecated**): Adds implementations for [`BorshSerialize`] and
  [`BorshDeserialize`] to [`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html). Due to a cyclic
  dependency that arose between [`borsh`] and `indexmap`, `borsh v1.5.6`
  added an `indexmap` feature that should be used instead of enabling the
  feature here.

_Note: only the `std` feature is enabled by default._

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[`no_std`]: #no-standard-library-targets
[`Serialize`]: https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html
[`Deserialize`]: https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html
[`BorshSerialize`]: `::borsh::BorshSerialize`
[`BorshDeserialize`]: `::borsh::BorshDeserialize`
[`borsh`]: `:https://docs.rs/borsh/latest/borsh/ser/trait.BorshSerialize.htmltrary`]: https://docs.rhttps://docs.rs/borsh/latest/borsh/de/trait.BorshDeserialize.htmly/trait.Arbihttps://docs.rs/borsh/latest/borsh/l
[`quickcheck::Arbitrary`]: https://docs.rs/quickcheck/latest/quickcheck/arbitrary/trait.Arbitrary.html

### Alternate Hashers

[`IndexMap`](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/2.13.0/indexmap/set/struct.IndexSet.html) have a default hasher type
[std::hash::RandomState](https://doc.rust-lang.org/stable/std/hash/random/struct.RandomState.html),
just like the standard `HashMap` and `HashSet`, which is resistant to
HashDoS attacks but not the most performant. Type aliases can make it easier
to use alternate hashers:

```rust
use fnv::FnvBuildHasher;
use indexmap::{IndexMap, IndexSet};

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;
type FnvIndexSet<T> = IndexSet<T, FnvBuildHasher>;

let std: IndexSet<i32> = (0..100).collect();
let fnv: FnvIndexSet<i32> = (0..100).collect();
assert_eq!(std, fnv);
```

### Rust Version

This version of indexmap requires Rust 1.82 or later.

The indexmap 2.x release series will use a carefully considered version
upgrade policy, where in a later 2.x version, we will raise the minimum
required Rust version.

## No Standard Library Targets

This crate supports being built without `std`, requiring `alloc` instead.
This is chosen by disabling the default “std” cargo feature, by adding
`default-features = false` to your dependency specification.

- Creating maps and sets using [IndexMap::new](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html#method.new) and
  [IndexMap::with_capacity](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html#method.with_capacity) is unavailable without `std`.
  Use methods [`IndexMap::default`], [IndexMap::with_hasher](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html#method.with_hasher),
  [IndexMap::with_capacity_and_hasher](https://docs.rs/indexmap/2.13.0/indexmap/map/struct.IndexMap.html#method.with_capacity_and_hasher) instead.
  A no-std compatible hasher will be needed as well, for example
  from the crate `twox-hash`.
- Macros [`indexmap!`](https://docs.rs/indexmap/2.13.0/indexmap/macro.indexmap.html) and [`indexset!`](https://docs.rs/indexmap/2.13.0/indexmap/macro.indexset.html) are unavailable without `std`. Use
  the macros [`indexmap_with_default!`](https://docs.rs/indexmap/2.13.0/indexmap/macro.indexmap_with_default.html) and [`indexset_with_default!`](https://docs.rs/indexmap/2.13.0/indexmap/macro.indexset_with_default.html) instead.