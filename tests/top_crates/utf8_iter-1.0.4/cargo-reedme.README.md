Provides iteration by `char` over `&[u8]` containing potentially-invalid
UTF-8 such that errors are handled according to the [WHATWG Encoding
Standard](https://encoding.spec.whatwg.org/#utf-8-decoder) (i.e. the same
way as in `String::from_utf8_lossy`).

The trait `Utf8CharsEx` provides the convenience method `chars()` on
byte slices themselves instead of having to use the more verbose
`Utf8Chars::new(slice)`.

```rust
use utf8_iter::Utf8CharsEx;
let data = b"\xFF\xC2\xE2\xE2\x98\xF0\xF0\x9F\xF0\x9F\x92\xE2\x98\x83";
let from_iter: String = data.chars().collect();
let from_std = String::from_utf8_lossy(data);
assert_eq!(from_iter, from_std);
```