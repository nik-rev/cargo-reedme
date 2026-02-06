 Rust-PHF is a library to generate efficient lookup tables at compile time using
 [perfect hash functions](http://en.wikipedia.org/wiki/Perfect_hash_function).

 It currently uses the
 [CHD algorithm](http://cmph.sourceforge.net/papers/esa09.pdf) and can generate
 a 100,000 entry map in roughly .4 seconds.

 MSRV (minimum supported rust version) is Rust 1.66.

 ## Usage

 PHF data structures can be constructed via either the procedural
 macros in the `phf_macros` crate or code generation supported by the
 `phf_codegen` crate. If you prefer macros, you can easily use them by
 enabling the `macros` feature of the `phf` crate, like:

```toml
 [dependencies]
 phf = { version = "0.13.1", features = ["macros"] }
 ```

 To compile the `phf` crate with a dependency on
 libcore instead of libstd, enabling use in environments where libstd
 will not work, set `default-features = false` for the dependency:

 ```toml
 [dependencies]
 # to use `phf` in `no_std` environments
 phf = { version = "0.13.1", default-features = false }
 ```

 ## Example (with the `macros` feature enabled)

 ```rust
 use phf::phf_map;

 #[derive(Clone)]
 pub enum Keyword {
     Loop,
     Continue,
     Break,
     Fn,
     Extern,
 }

 static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
     "loop" => Keyword::Loop,
     "continue" => Keyword::Continue,
     "break" => Keyword::Break,
     "fn" => Keyword::Fn,
     "extern" => Keyword::Extern,
 };

 pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
     KEYWORDS.get(keyword).cloned()
 }
```

 Alternatively, you can use the [`phf_codegen`] crate to generate PHF datatypes
 in a build script.

 [`phf_codegen`]: https://docs.rs/phf_codegen

 ## Note

 Currently, the macro syntax has some limitations and may not
 work as you want. See [#183] or [#196] for example.

 [#183]: https://github.com/rust-phf/rust-phf/issues/183
 [#196]: https://github.com/rust-phf/rust-phf/issues/196