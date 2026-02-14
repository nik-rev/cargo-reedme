Parsing and inspecting Rust literal tokens.

This library offers functionality to parse Rust literals, i.e. tokens in the
Rust programming language that represent fixed values. The grammar for
those is defined [here][ref].

This kind of functionality already exists in the crate `syn`. However, as
you oftentimes don’t need (nor want) the full power of `syn`, `litrs` was
built. This crate also offers a bit more flexibility compared to `syn`
(only regarding literals, of course).


# Quick start

| **`StringLit::try_from(tt)?.value()`** |
| - |

… where `tt` is a `proc_macro::TokenTree` and where [`StringLit`](https://docs.rs/litrs/1.0.0/litrs/string/struct.StringLit.html) can be
replaced with [`Literal`](https://docs.rs/litrs/1.0.0/litrs/enum.Literal.html) or other types of literals (e.g. [`FloatLit`](https://docs.rs/litrs/1.0.0/litrs/float/struct.FloatLit.html)).
Calling `value()` returns the value that is represented by the literal.

**Mini Example**

```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
     let first_token = input.into_iter().next().unwrap(); // Do proper error handling!
     let string_value = match litrs::StringLit::try_from(first_token) {
         Ok(string_lit) => string_lit.value(),
         Err(e) => return e.to_compile_error(),
     };

     // `string_value` is the string value with all escapes resolved.
     todo!()
}
```

# Overview

The main types of this library are [`Literal`](https://docs.rs/litrs/1.0.0/litrs/enum.Literal.html), representing any kind of
literal, and `*Lit`, like [`StringLit`](https://docs.rs/litrs/1.0.0/litrs/string/struct.StringLit.html) or [`FloatLit`](https://docs.rs/litrs/1.0.0/litrs/float/struct.FloatLit.html), representing a
specific kind of literal.

There are different ways to obtain such a literal type:

- **`parse`**: parses a `&str` or `String` and returns `Result<_,
    ParseError>`. For example: [`Literal::parse`](https://docs.rs/litrs/1.0.0/litrs/enum.Literal.html#method.parse) and
    [`IntegerLit::parse`](https://docs.rs/litrs/1.0.0/litrs/integer/struct.IntegerLit.html#method.parse).

- **`From<proc_macro::Literal> for Literal`**: turns a `Literal` value from
    the `proc_macro` crate into a `Literal` from this crate.

- **`TryFrom<proc_macro::Literal> for *Lit`**: tries to turn a
    `proc_macro::Literal` into a specific literal type of this crate. If
    the input is a literal of a different kind, `Err(InvalidToken)` is
    returned.

- **`TryFrom<proc_macro::TokenTree>`**: attempts to turn a token tree into a
    literal type of this crate. An error is returned if the token tree is
    not a literal, or if you are trying to turn it into a specific kind of
    literal and the token tree is a different kind of literal.

All of the `From` and `TryFrom` conversions also work for reference to
`proc_macro` types. Additionally, if the crate feature `proc-macro2` is
enabled, all these `From` and `TryFrom` impls also exist for the
corresponding `proc_macro2` types.

**Note**: `true` and `false` are `Ident`s when passed to your proc macro.
The `TryFrom<TokenTree>` impls check for those two special idents and
return a [`BoolLit`](https://docs.rs/litrs/1.0.0/litrs/bool/enum.BoolLit.html) appropriately. For that reason, there is also no
`TryFrom<proc_macro::Literal>` impl for [`BoolLit`](https://docs.rs/litrs/1.0.0/litrs/bool/enum.BoolLit.html). The `proc_macro::Literal`
simply cannot represent bool literals.


# Examples

In a proc-macro:

```rust
use std::convert::TryFrom;
use proc_macro::TokenStream;
use litrs::FloatLit;

#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
     let mut input = input.into_iter().collect::<Vec<_>>();
     if input.len() != 1 {
         // Please do proper error handling in your real code!
         panic!("expected exactly one token as input");
     }
     let token = input.remove(0);

     match FloatLit::try_from(token) {
         Ok(float_lit) => { /* do something */ }
         Err(e) => return e.to_compile_error(),
     }

     // Dummy output
     TokenStream::new()
}
```

Parsing from string:

```rust
use litrs::{FloatLit, Literal};

// Parse a specific kind of literal (float in this case):
let float_lit = FloatLit::parse("3.14f32");
assert!(float_lit.is_ok());
assert_eq!(float_lit.unwrap().suffix(), "f32");
assert!(FloatLit::parse("'c'").is_err());

// Parse any kind of literal. After parsing, you can inspect the literal
// and decide what to do in each case.
let lit = Literal::parse("0xff80").expect("failed to parse literal");
match lit {
    Literal::Integer(lit) => { /* ... */ }
    Literal::Float(lit) => { /* ... */ }
    Literal::Bool(lit) => { /* ... */ }
    Literal::Char(lit) => { /* ... */ }
    Literal::String(lit) => { /* ... */ }
    Literal::Byte(lit) => { /* ... */ }
    Literal::ByteString(lit) => { /* ... */ }
    Literal::CString(lit) => { /* ... */ }
    _ => { /* ... */ }
}
```

# SemVer/Versioning guarantees

Some technically breaking changes might be released as a minor/patch version
in some situations, for example:
- Bugs in this library (e.g. behavior different from rustc)
- Rust making breaking changes, likely via new edition

In all cases, releasing these changes as a minor/patch version is only done
if it is expected that breakage is minimal or non-existent.


# Crate features

- `proc-macro2`: adds the dependency `proc_macro2`, a bunch of `From` and
  `TryFrom` impls, and [`InvalidToken::to_compile_error2`].
- `check_suffix`: if enabled, `parse` functions will exactly verify that the
  literal suffix is valid. Adds the dependency `unicode-xid`. If disabled,
  only an approximate check (only in ASCII range) is done. If you are
  writing a proc macro, you don’t need to enable this as the suffix is
  already checked by the compiler.


[ref]: https://doc.rust-lang.org/reference/tokens.html#literals
