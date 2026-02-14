Extra iterator adaptors, functions and macros.

To extend [`Iterator`](https://doc.rust-lang.org/stable/core/iter/traits/iterator/trait.Iterator.html) with methods in this crate, import
the [`Itertools`](https://docs.rs/itertools/0.14.0/itertools/trait.Itertools.html) trait:

```rust
use itertools::Itertools;
```

Now, new methods like [`interleave`](https://docs.rs/itertools/0.14.0/itertools/trait.Itertools.html#tymethod.interleave)
are available on all iterators:

```rust
use itertools::Itertools;

let it = (1..3).interleave(vec![-1, -2]);
itertools::assert_equal(it, vec![1, -1, 2, -2]);
```

Most iterator methods are also provided as functions (with the benefit
that they convert parameters using [`IntoIterator`](https://doc.rust-lang.org/stable/core/iter/traits/collect/trait.IntoIterator.html)):

```rust
use itertools::interleave;

for elt in interleave(&[1, 2, 3], &[2, 3, 4]) {
    /* loop body */
}
```

## Crate Features

- `use_std`
  - Enabled by default.
  - Disable to compile itertools using `#![no_std]`. This disables
    any item that depend on allocations (see the `use_alloc` feature)
    and hash maps (like `unique`, `counts`, `into_grouping_map` and more).
- `use_alloc`
  - Enabled by default.
  - Enables any item that depend on allocations (like `chunk_by`,
    `kmerge`, `join` and many more).

## Rust Version

This version of itertools requires Rust 1.63.0 or later.