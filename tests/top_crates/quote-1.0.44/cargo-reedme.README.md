[![github]](https://github.com/dtolnay/quote) [![crates-io]](https://crates.io/crates/quote) [![docs-rs]](https://docs.rs/quote)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

This crate provides the [`quote!`](https://docs.rs/quote/1.0.44/quote/macro.quote.html) macro for turning Rust syntax tree data
structures into tokens of source code.

Procedural macros in Rust receive a stream of tokens as input, execute
arbitrary Rust code to determine how to manipulate those tokens, and produce
a stream of tokens to hand back to the compiler to compile into the caller’s
crate. Quasi-quoting is a solution to one piece of that — producing
tokens to return to the compiler.

The idea of quasi-quoting is that we write *code* that we treat as *data*.
Within the `quote!` macro, we can write what looks like code to our text
editor or IDE. We get all the benefits of the editor’s brace matching,
syntax highlighting, indentation, and maybe autocompletion. But rather than
compiling that as code into the current crate, we can treat it as data, pass
it around, mutate it, and eventually hand it back to the compiler as tokens
to compile into the macro caller’s crate.

This crate is motivated by the procedural macro use case, but is a
general-purpose Rust quasi-quoting library and is not specific to procedural
macros.

```toml
[dependencies]
quote = "1.0"
```

<br>

# Example

The following quasi-quoted block of code is something you might find in [a]
procedural macro having to do with data structure serialization. The `#var`
syntax performs interpolation of runtime variables into the quoted tokens.
Check out the documentation of the [`quote!`](https://docs.rs/quote/1.0.44/quote/macro.quote.html) macro for more detail about
the syntax. See also the [`quote_spanned!`](https://docs.rs/quote/1.0.44/quote/macro.quote_spanned.html) macro which is important for
implementing hygienic procedural macros.

[a]: https://serde.rs/

```rust
let tokens = quote! {
    struct SerializeWith #generics #where_clause {
        value: &'a #field_ty,
        phantom: core::marker::PhantomData<#item_ty>,
    }

    impl #generics serde::Serialize for SerializeWith #generics #where_clause {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            #path(self.value, serializer)
        }
    }

    SerializeWith {
        value: #value,
        phantom: core::marker::PhantomData::<#item_ty>,
    }
};
```

<br>

# Non-macro code generators

When using `quote` in a build.rs or main.rs and writing the output out to a
file, consider having the code generator pass the tokens through
[prettyplease] before writing. This way if an error occurs in the generated
code it is convenient for a human to read and debug.

[prettyplease]: https://github.com/dtolnay/prettyplease