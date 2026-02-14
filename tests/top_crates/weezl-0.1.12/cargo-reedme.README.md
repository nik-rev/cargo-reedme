# LZW decoder and encoder

This crates provides an `Encoder` and a `Decoder` in their respective modules. The code words
are written from and to bit byte slices (or streams) where it is possible to write either the
most or least significant bits first. The maximum possible code size is 12 bits, the smallest
available code size is 2 bits.

## Example

These two code blocks show the compression and corresponding decompression. Note that you must
use the same arguments to `Encoder` and `Decoder`, otherwise the decoding might fail or produce
bad results.

```rust
use weezl::{BitOrder, encode::Encoder};

let data = b"Hello, world";
let compressed = Encoder::new(BitOrder::Msb, 9)
    .encode(data)
    .unwrap();
```

```rust
use weezl::{BitOrder, decode::Decoder};

let decompressed = Decoder::new(BitOrder::Msb, 9)
    .decode(&compressed)
    .unwrap();
assert_eq!(decompressed, data);
```

## LZW Details

The de- and encoder expect the LZW stream to start with a clear code and end with an
end code which are defined as follows:

 * `CLEAR_CODE == 1 << min_code_size`
 * `END_CODE   == CLEAR_CODE + 1`

For optimal performance, all buffers and input and output slices should be as large as possible
and at least 2048 bytes long. This extends to input streams which should have similarly sized
buffers. This library uses Rust’s standard allocation interfaces (`Box` and `Vec` to be
precise). Since there are no ways to handle allocation errors it is not recommended to operate
it on 16-bit targets.

## Allocations and standard library

The main algorithm can be used in `no_std` as well, although it requires an allocator. This
restriction might be lifted at a later stage. For this you should deactivate the `std` feature.
The main interfaces stay intact but the `into_stream` combinator is no available.