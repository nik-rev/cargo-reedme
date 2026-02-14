Rust binding to the [zstd library][zstd].

This crate provides:

* An [encoder](stream/write/struct.Encoder.html) to compress data using zstd
  and send the output to another write.
* A [decoder](stream/read/struct.Decoder.html) to read input data from a `Read`
  and decompress it.
* Convenient functions for common tasks.

# Example

```rust
use std::io;

// Uncompress input and print the result.
zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
```

[zstd]: https://github.com/facebook/zstd