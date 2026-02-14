# `litemap`

`litemap` is a crate providing [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html), a highly simplistic “flat” key-value map
based off of a single sorted vector.

The main goal of this crate is to provide a map that is good enough for small
sizes, and does not carry the binary size impact of [`HashMap`](https://doc.rust-lang.org/stable/std/collections/hash/map/struct.HashMap.html)
or [`BTreeMap`](alloc::collections::BTreeMap).

If binary size is not a concern, [`std::collections::BTreeMap`](https://doc.rust-lang.org/stable/alloc/collections/btree/map/struct.BTreeMap.html) may be a better choice
for your use case. It behaves very similarly to [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) for less than 12 elements,
and upgrades itself gracefully for larger inputs.

## Performance characteristics

[`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) is a data structure with similar characteristics as [`std::collections::BTreeMap`](https://doc.rust-lang.org/stable/alloc/collections/btree/map/struct.BTreeMap.html) but
with slightly different trade-offs as it’s implemented on top of a flat storage (e.g. Vec).

* [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) iteration is generally faster than `BTreeMap` because of the flat storage.
* [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) can be pre-allocated whereas `BTreeMap` can’t.
* [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) has a smaller memory footprint than `BTreeMap` for small collections (< 20 items).
* Lookup is `O(log(n))` like `BTreeMap`.
* Insertion is generally `O(n)`, but optimized to `O(1)` if the new item sorts greater than the current items. In `BTreeMap` it’s `O(log(n))`.
* Deletion is `O(n)` whereas `BTreeMap` is `O(log(n))`.
* Bulk operations like `from_iter`, `extend` and deserialization have an optimized `O(n)` path
  for inputs that are ordered and `O(n*log(n))` complexity otherwise.

## Pluggable Backends

By default, [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) is backed by a [`Vec`]; however, it can be backed by any appropriate
random-access data store, giving that data store a map-like interface. See the [`store`](https://docs.rs/litemap/0.8.1/litemap/store/)
module for more details.

## Const construction

[`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html) supports const construction from any store that is const-constructible, such as a
static slice, via [`LiteMap::from_sorted_store_unchecked()`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html#method.from_sorted_store_unchecked). This also makes [`LiteMap`](https://docs.rs/litemap/0.8.1/litemap/map/struct.LiteMap.html)
suitable for use with [`databake`]. See [`impl Bake for LiteMap`] for more details.

[`impl Bake for LiteMap`]: ./struct.LiteMap.html#impl-Bake-for-LiteMap<K,+V,+S>
[`Vec`]: alloc::vec::Vec