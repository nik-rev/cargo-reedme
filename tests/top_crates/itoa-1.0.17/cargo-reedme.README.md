[![github]](https://github.com/dtolnay/itoa) [![crates-io]](https://crates.io/crates/itoa) [![docs-rs]](https://docs.rs/itoa)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

This crate provides a fast conversion of integer primitives to decimal
strings. The implementation comes straight from [libcore] but avoids the
performance penalty of going through [`core::fmt::Formatter`](https://doc.rust-lang.org/stable/core/fmt/struct.Formatter.html).

See also [`zmij`] for printing floating point primitives.

[libcore]: https://github.com/rust-lang/rust/blob/1.92.0/library/core/src/fmt/num.rs#L190-L253
[`zmij`]: https://github.com/dtolnay/zmij

# Example

```rust
fn main() {
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    assert_eq!(printed, "128");
}
```

# Performance

The [itoa-benchmark] compares this library and other Rust integer formatting
implementations across a range of integer sizes. The vertical axis in this
chart shows nanoseconds taken by a single execution of
`itoa::Buffer::new().format(value)` so a lower result indicates a faster
library.

[itoa-benchmark]: https://github.com/dtolnay/itoa-benchmark

![performance](https://raw.githubusercontent.com/dtolnay/itoa/master/itoa-benchmark.png)