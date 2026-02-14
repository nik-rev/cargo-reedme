# UniCase

UniCase provides a way of specifying strings that are case-insensitive.

UniCase supports full [Unicode case
folding](https://www.w3.org/International/wiki/Case_folding). It can also
utilize faster ASCII case comparisons, if both strings are ASCII.

Using the `UniCase::new()` constructor will check the string to see if it
is all ASCII. When a `UniCase` is compared against another, if both are
ASCII, it will use the faster comparison.

There also exists the `Ascii` type in this crate, which will always assume
to use the ASCII case comparisons, if the encoding is already known.

## Example

```rust
use unicase::UniCase;

let a = UniCase::new("Maße");
let b = UniCase::new("MASSE");
let c = UniCase::new("mase");

assert_eq!(a, b);
assert!(b != c);
```

## Ascii

```rust
use unicase::Ascii;

let a = Ascii::new("foobar");
let b = Ascii::new("FoObAr");

assert_eq!(a, b);
```