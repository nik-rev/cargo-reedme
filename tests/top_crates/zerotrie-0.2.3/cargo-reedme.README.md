A data structure offering zero-copy storage and retrieval of byte strings, with a focus
on the efficient storage of ASCII strings. Strings are mapped to `usize` values.

[`ZeroTrie`](https://docs.rs/zerotrie/0.2.3/zerotrie/zerotrie/struct.ZeroTrie.html) does not support mutation because doing so would require recomputing the entire
data structure. Instead, it supports conversion to and from [`LiteMap`] and [`BTreeMap`].

There are multiple variants of [`ZeroTrie`](https://docs.rs/zerotrie/0.2.3/zerotrie/zerotrie/struct.ZeroTrie.html) optimized for different use cases.

# Examples

```rust
use zerotrie::ZeroTrie;

let data: &[(&str, usize)] = &[("abc", 11), ("xyz", 22), ("axyb", 33)];

let trie: ZeroTrie<Vec<u8>> = data.iter().copied().collect();

assert_eq!(trie.get("axyb"), Some(33));
assert_eq!(trie.byte_len(), 18);
```

# Internal Structure

To read about the internal structure of [`ZeroTrie`](https://docs.rs/zerotrie/0.2.3/zerotrie/zerotrie/struct.ZeroTrie.html), build the docs with private modules:

```bash
cargo doc --document-private-items --all-features --no-deps --open
```

[`LiteMap`]: https://docs.rs/litemap/latest/litemap/map/struct.LiteMap.html
[`BTreeMap`]: https://doc.rust-lang.org/stable/alloc/collections/btree/map/struct.BTreeMap.html