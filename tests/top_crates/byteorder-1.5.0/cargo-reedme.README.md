This crate provides convenience methods for encoding and decoding numbers in
either [big-endian or little-endian order].

The organization of the crate is pretty simple. A trait, [`ByteOrder`], specifies
byte conversion methods for each type of number in Rust (sans numbers that have
a platform dependent size like `usize` and `isize`). Two types, [`BigEndian`]
and [`LittleEndian`] implement these methods. Finally, [`ReadBytesExt`] and
[`WriteBytesExt`] provide convenience methods available to all types that
implement [`Read`] and [`Write`].

An alias, [`NetworkEndian`], for [`BigEndian`] is provided to help improve
code clarity.

An additional alias, [`NativeEndian`], is provided for the endianness of the
local platform. This is convenient when serializing data for use and
conversions are not desired.

# Examples

Read unsigned 16 bit big-endian integers from a [`Read`] type:

```rust
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
```

Write unsigned 16 bit little-endian integers to a [`Write`] type:

```rust
use byteorder::{LittleEndian, WriteBytesExt};

let mut wtr = vec![];
wtr.write_u16::<LittleEndian>(517).unwrap();
wtr.write_u16::<LittleEndian>(768).unwrap();
assert_eq!(wtr, vec![5, 2, 0, 3]);
```

# Optional Features

This crate optionally provides support for 128 bit values (`i128` and `u128`)
when built with the `i128` feature enabled.

This crate can also be used without the standard library.

# Alternatives

Note that as of Rust 1.32, the standard numeric types provide built-in methods
like `to_le_bytes` and `from_le_bytes`, which support some of the same use
cases.

[big-endian or little-endian order]: https://en.wikipedia.org/wiki/Endianness
[`ByteOrder`]: trait.ByteOrder.html
[`BigEndian`]: enum.BigEndian.html
[`LittleEndian`]: enum.LittleEndian.html
[`ReadBytesExt`]: trait.ReadBytesExt.html
[`WriteBytesExt`]: trait.WriteBytesExt.html
[`NetworkEndian`]: type.NetworkEndian.html
[`NativeEndian`]: type.NativeEndian.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html