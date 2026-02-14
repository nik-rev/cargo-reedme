Correct, fast, and configurable [base64][] decoding and encoding. Base64
transports binary data efficiently in contexts where only plain text is
allowed.

[base64]: https://developer.mozilla.org/en-US/docs/Glossary/Base64

# Usage

Use an [`Engine`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html) to decode or encode base64, configured with the base64
alphabet and padding behavior best suited to your application.

## Engine setup

There is more than one way to encode a stream of bytes as “base64”.
Different applications use different encoding
[alphabet::Alphabet](https://docs.rs/base64/0.22.1/base64/alphabet/struct.Alphabet.html) and
[engine::general_purpose::GeneralPurposeConfig](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/struct.GeneralPurposeConfig.html).

### Encoding alphabet

Almost all base64 [alphabet::Alphabet](https://docs.rs/base64/0.22.1/base64/alphabet/struct.Alphabet.html) use `A-Z`, `a-z`, and
`0-9`, which gives nearly 64 characters (26 + 26 + 10 = 62), but they differ
in their choice of their final 2.

Most applications use the [alphabet::STANDARD](https://docs.rs/base64/0.22.1/base64/alphabet/const.STANDARD.html) alphabet specified
in [RFC 4648][rfc-alphabet].  If that’s all you need, you can get started
quickly by using the pre-configured
[engine::general_purpose::STANDARD](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/const.STANDARD.html) engine, which is also available
in the [`prelude`](https://docs.rs/base64/0.22.1/base64/prelude/) module as shown here, if you prefer a minimal `use`
footprint.

```rust
use base64::prelude::*;

assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");
```

[rfc-alphabet]: https://datatracker.ietf.org/doc/html/rfc4648#section-4

Other common alphabets are available in the [`alphabet`](https://docs.rs/base64/0.22.1/base64/alphabet/) module.

#### URL-safe alphabet

The standard alphabet uses `+` and `/` as its two non-alphanumeric tokens,
which cannot be safely used in URL’s without encoding them as `%2B` and
`%2F`.

To avoid that, some applications use a [alphabet::URL_SAFE](https://docs.rs/base64/0.22.1/base64/alphabet/const.URL_SAFE.html),
which uses `-` and `_` instead. To use that alternative alphabet, use the
[engine::general_purpose::URL_SAFE](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/const.URL_SAFE.html) engine. This example doesn’t
use [`prelude`](https://docs.rs/base64/0.22.1/base64/prelude/) to show what a more explicit `use` would look like.

```rust
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

assert_eq!(URL_SAFE.decode(b"-uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(URL_SAFE.encode(b"\xFF\xEC\x20\x55\0"), "_-wgVQA=");
```

### Padding characters

Each base64 character represents 6 bits (2⁶ = 64) of the original binary
data, and every 3 bytes of input binary data will encode to 4 base64
characters (8 bits × 3 = 6 bits × 4 = 24 bits).

When the input is not an even multiple of 3 bytes in length, [canonical][]
base64 encoders insert padding characters at the end, so that the output
length is always a multiple of 4:

[canonical]: https://datatracker.ietf.org/doc/html/rfc4648#section-3.5

```rust
use base64::{engine::general_purpose::STANDARD, Engine as _};

assert_eq!(STANDARD.encode(b""),    "");
assert_eq!(STANDARD.encode(b"f"),   "Zg==");
assert_eq!(STANDARD.encode(b"fo"),  "Zm8=");
assert_eq!(STANDARD.encode(b"foo"), "Zm9v");
```

Canonical encoding ensures that base64 encodings will be exactly the same,
byte-for-byte, regardless of input length. But the `=` padding characters
aren’t necessary for decoding, and they may be omitted by using a
[engine::general_purpose::NO_PAD](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/const.NO_PAD.html) configuration:

```rust
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

assert_eq!(STANDARD_NO_PAD.encode(b""),    "");
assert_eq!(STANDARD_NO_PAD.encode(b"f"),   "Zg");
assert_eq!(STANDARD_NO_PAD.encode(b"fo"),  "Zm8");
assert_eq!(STANDARD_NO_PAD.encode(b"foo"), "Zm9v");
```

The pre-configured `NO_PAD` engines will reject inputs containing padding
`=` characters. To encode without padding and still accept padding while
decoding, create an [engine::general_purpose::GeneralPurpose](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/struct.GeneralPurpose.html) with
that [engine::DecodePaddingMode](https://docs.rs/base64/0.22.1/base64/engine/enum.DecodePaddingMode.html).

```rust
assert_eq!(STANDARD_NO_PAD.decode(b"Zm8="), Err(base64::DecodeError::InvalidPadding));
```

### Further customization

Decoding and encoding behavior can be customized by creating an
[engine::GeneralPurpose](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/struct.GeneralPurpose.html) with an [alphabet::Alphabet](https://docs.rs/base64/0.22.1/base64/alphabet/struct.Alphabet.html) and
[engine::GeneralPurposeConfig](https://docs.rs/base64/0.22.1/base64/engine/general_purpose/struct.GeneralPurposeConfig.html):

```rust
use base64::{engine, alphabet, Engine as _};

// bizarro-world base64: +/ as the first symbols instead of the last
let alphabet =
    alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
    .unwrap();

// a very weird config that encodes with padding but requires no padding when decoding...?
let crazy_config = engine::GeneralPurposeConfig::new()
    .with_decode_allow_trailing_bits(true)
    .with_encode_padding(true)
    .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

let encoded = crazy_engine.encode(b"abc 123");

```

## Memory allocation

The [Engine::decode()](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.decode) and [Engine::encode()](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.encode) engine methods
allocate memory for their results – `decode` returns a `Vec<u8>` and
`encode` returns a `String`. To instead decode or encode into a buffer that
you allocated, use one of the alternative methods:

#### Decoding

| Method                     | Output                        | Allocates memory              |
| -------------------------- | ----------------------------- | ----------------------------- |
| [`Engine::decode`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.decode)         | returns a new `Vec<u8>`       | always                        |
| [`Engine::decode_vec`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.decode_vec)     | appends to provided `Vec<u8>` | if `Vec` lacks capacity       |
| [`Engine::decode_slice`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.decode_slice)   | writes to provided `&[u8]`    | never

#### Encoding

| Method                     | Output                       | Allocates memory               |
| -------------------------- | ---------------------------- | ------------------------------ |
| [`Engine::encode`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.encode)         | returns a new `String`       | always                         |
| [`Engine::encode_string`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.encode_string)  | appends to provided `String` | if `String` lacks capacity     |
| [`Engine::encode_slice`](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.encode_slice)   | writes to provided `&[u8]`   | never                          |

## Input and output

The `base64` crate can [Engine::decode()](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.decode) and
[Engine::encode()](https://docs.rs/base64/0.22.1/base64/engine/trait.Engine.html#tymethod.encode) values in memory, or
[read::DecoderReader](https://docs.rs/base64/0.22.1/base64/read/decoder/struct.DecoderReader.html) and
[write::EncoderWriter](https://docs.rs/base64/0.22.1/base64/write/encoder/struct.EncoderWriter.html) provide streaming decoding and
encoding for any [std::io::Read](https://doc.rust-lang.org/stable/std/io/trait.Read.html) or [std::io::Write](https://doc.rust-lang.org/stable/std/io/trait.Write.html)
byte stream.

#### Decoding

```rust
use base64::{engine::general_purpose::STANDARD, read::DecoderReader};

let mut input = io::stdin();
let mut decoder = DecoderReader::new(&mut input, &STANDARD);
io::copy(&mut decoder, &mut io::stdout())?;
```

#### Encoding

```rust
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};

let mut output = io::stdout();
let mut encoder = EncoderWriter::new(&mut output, &STANDARD);
io::copy(&mut io::stdin(), &mut encoder)?;
```

#### Display

If you only need a base64 representation for implementing the
[std::fmt::Display](https://doc.rust-lang.org/stable/core/fmt/trait.Display.html) trait, use
[display::Base64Display](https://docs.rs/base64/0.22.1/base64/display/struct.Base64Display.html):

```rust
use base64::{display::Base64Display, engine::general_purpose::STANDARD};

let value = Base64Display::new(b"\0\x01\x02\x03", &STANDARD);
assert_eq!("base64: AAECAw==", format!("base64: {}", value));
```

# Panics

If length calculations result in overflowing `usize`, a panic will result.