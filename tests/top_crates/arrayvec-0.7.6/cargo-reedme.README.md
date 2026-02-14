**arrayvec** provides the types [`ArrayVec`](https://docs.rs/arrayvec/0.7.6/arrayvec/arrayvec/struct.ArrayVec.html) and [`ArrayString`](https://docs.rs/arrayvec/0.7.6/arrayvec/array_string/struct.ArrayString.html): 
array-backed vector and string types, which store their contents inline.

The arrayvec package has the following cargo features:

- `std`
  - Optional, enabled by default
  - Use libstd; disable to use `no_std` instead.

- `serde`
  - Optional
  - Enable serialization for ArrayVec and ArrayString using serde 1.x

- `zeroize`
  - Optional
  - Implement `Zeroize` for ArrayVec and ArrayString

## Rust Version

This version of arrayvec requires Rust 1.51 or later.
