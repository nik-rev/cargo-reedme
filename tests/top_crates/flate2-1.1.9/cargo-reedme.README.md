A DEFLATE-based stream compression/decompression library

This library provides support for compression and decompression of
DEFLATE-based streams:

* the DEFLATE format itself
* the zlib format
* gzip

These three formats are all closely related and largely only differ in their
headers/footers. This crate has three types in each submodule for dealing
with these three formats.

# Implementation

In addition to supporting three formats, this crate supports several different
backends, controlled through this crate’s *features flags*:

* `default`, or `rust_backend` - this implementation currently uses the `miniz_oxide`
  crate which is a port of `miniz.c` to Rust. This feature does not
  require a C compiler, and only uses safe Rust code.

  Note that the `rust_backend` feature may at some point be switched to use `zlib-rs`,
  and that `miniz_oxide` should be used explicitly if this is not desired.

* `zlib-rs` - this implementation utilizes the `zlib-rs` crate, a Rust rewrite of zlib.
  This backend is the fastest, at the cost of some `unsafe` Rust code.

Several backends implemented in C are also available.
These are useful in case you are already using a specific C implementation
and need the result of compression to be bit-identical.
See the crate’s README for details on the available C backends.

The `zlib-rs` backend typically outperforms all the C implementations.

# Feature Flags
Activate the `document-features` cargo feature to see feature docs here

## Ambiguous feature selection

As Cargo features are additive, while backends are not, there is an order in which backends
become active if multiple are selected.

* zlib-ng
* zlib-rs
* cloudflare_zlib
* miniz_oxide

# Organization

This crate consists of three main modules: `bufread`, `read`, and `write`. Each module
implements DEFLATE, zlib, and gzip for [`std::io::BufRead`](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html) input types, [`std::io::Read`](https://doc.rust-lang.org/stable/std/io/trait.Read.html) input
types, and [`std::io::Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html) output types respectively.

Use the [`mod@bufread`](https://docs.rs/flate2/1.1.9/flate2/bufread/) implementations if you can provide a `BufRead` type for the input.
The `&[u8]` slice type implements the `BufRead` trait.

The [`mod@read`](https://docs.rs/flate2/1.1.9/flate2/read/) implementations conveniently wrap a `Read` type in a `BufRead` implementation.
However, the `read` implementations may
[read past the end of the input data](https://github.com/rust-lang/flate2-rs/issues/338),
making the `Read` type useless for subsequent reads of the input. If you need to re-use the
`Read` type, wrap it in a [`std::io::BufReader`](https://doc.rust-lang.org/stable/std/io/buffered/bufreader/struct.BufReader.html), use the `bufread` implementations,
and perform subsequent reads on the `BufReader`.

The [`mod@write`](https://docs.rs/flate2/1.1.9/flate2/write/) implementations are most useful when there is no way to create a `BufRead`
type, notably when reading async iterators (streams).

```rust
use futures::{Stream, StreamExt};
use std::io::{Result, Write as _};

async fn decompress_gzip_stream<S, I>(stream: S) -> Result<Vec<u8>>
where
    S: Stream<Item = I>,
    I: AsRef<[u8]>
{
    let mut stream = std::pin::pin!(stream);
    let mut w = Vec::<u8>::new();
    let mut decoder = flate2::write::GzDecoder::new(w);
    while let Some(input) = stream.next().await {
        decoder.write_all(input.as_ref())?;
    }
    decoder.finish()
}
```


Note that types which operate over a specific trait often implement the mirroring trait as well.
For example a `bufread::DeflateDecoder<T>` *also* implements the
[`Write`] trait if `T: Write`. That is, the “dual trait” is forwarded directly
to the underlying object if available.

# About multi-member Gzip files

While most `gzip` files one encounters will have a single *member* that can be read
with the [`GzDecoder`], there may be some files which have multiple members.

A [`GzDecoder`] will only read the first member of gzip data, which may unexpectedly
provide partial results when a multi-member gzip file is encountered. `GzDecoder` is appropriate
for data that is designed to be read as single members from a multi-member file. `bufread::GzDecoder`
and `write::GzDecoder` also allow non-gzip data following gzip data to be handled.

The [`MultiGzDecoder`] on the other hand will decode all members of a `gzip` file
into one consecutive stream of bytes, which hides the underlying *members* entirely.
If a file contains non-gzip data after the gzip data, MultiGzDecoder will
emit an error after decoding the gzip data. This behavior matches the `gzip`,
`gunzip`, and `zcat` command line tools.

[`Bufread`]: std::io::BufRead
[`BufReader`]: std::io::BufReader
[`Read`]: std::io::Read
[`Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
[`GzDecoder`]: https://docs.rs/flate2/1.1.9/flate2/gz/bufread/strhttps://docs.rs/flate2/1.1.9/flate2/gz/bufread/struct.GzDecoder.html
[`MultiGzDecoder`]: https://docs.rs/flate2/1.1.9/flate2/gz/bufread/struct.MultiGzDecoder.html