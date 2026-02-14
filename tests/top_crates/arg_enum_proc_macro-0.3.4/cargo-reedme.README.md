# arg_enum_proc_macro

This crate consists in a procedural macro derive that provides the
same implementations that clap the [`clap::arg_enum`][1] macro provides:
[`std::fmt::Display`](https://doc.rust-lang.org/stable/core/fmt/trait.Display.html), [`std::str::FromStr`](https://doc.rust-lang.org/stable/core/str/traits/trait.FromStr.html) and a `variants()` function.

By using a procedural macro it allows documenting the enum fields
correctly and avoids the requirement of expanding the macro to use
the structure with [cbindgen](https://crates.io/crates/cbindgen).

[1]: https://docs.rs/clap/2.32.0/clap/macro.arg_enum.html
