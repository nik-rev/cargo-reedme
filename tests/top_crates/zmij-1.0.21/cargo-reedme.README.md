[![github]](https://github.com/dtolnay/zmij) [![crates-io]](https://crates.io/crates/zmij) [![docs-rs]](https://docs.rs/zmij)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A double-to-string conversion algorithm based on [Schubfach] and [yy].

This Rust implementation is a line-by-line port of Victor Zverovich’s
implementation in C++, <https://github.com/vitaut/zmij>.

[Schubfach]: https://fmt.dev/papers/Schubfach4.pdf
[yy]: https://github.com/ibireme/c_numconv_benchmark/blob/master/vendor/yy_double/yy_double.c

<br>

# Example

```rust
fn main() {
    let mut buffer = zmij::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

<br>

## Performance

The [dtoa-benchmark] compares this library and other Rust floating point
formatting implementations across a range of precisions. The vertical axis
in this chart shows nanoseconds taken by a single execution of
`zmij::Buffer::new().format_finite(value)` so a lower result indicates a
faster library.

[dtoa-benchmark]: https://github.com/dtolnay/dtoa-benchmark

![performance](https://raw.githubusercontent.com/dtolnay/zmij/master/dtoa-benchmark.png)