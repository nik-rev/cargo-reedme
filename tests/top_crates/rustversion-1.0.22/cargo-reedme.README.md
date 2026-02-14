[![github]](https://github.com/dtolnay/rustversion) [![crates-io]](https://crates.io/crates/rustversion) [![docs-rs]](https://docs.rs/rustversion)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

This crate provides macros for conditional compilation according to rustc
compiler version, analogous to [`#[cfg(...)]`][cfg] and
[`#[cfg_attr(...)]`][cfg_attr].

[cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
[cfg_attr]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute

<br>

# Selectors

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::stable]</code></b>
  —<br>
  True on any stable compiler.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::stable(1.34)]</code></b>
  —<br>
  True on exactly the specified stable compiler.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::beta]</code></b>
  —<br>
  True on any beta compiler.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::nightly]</code></b>
  —<br>
  True on any nightly compiler or dev build.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::nightly(2025-01-01)]</code></b>
  —<br>
  True on exactly one nightly.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::since(1.34)]</code></b>
  —<br>
  True on that stable release and any later compiler, including beta and
  nightly.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::since(2025-01-01)]</code></b>
  —<br>
  True on that nightly and all newer ones.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::before(</code></b><i>version or date</i><b><code style="display:inline">)]</code></b>
  —<br>
  Negative of <i>#[rustversion::since(...)]</i>.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::not(</code></b><i>selector</i><b><code style="display:inline">)]</code></b>
  —<br>
  Negative of any selector; for example <i>#[rustversion::not(nightly)]</i>.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::any(</code></b><i>selectors...</i><b><code style="display:inline">)]</code></b>
  —<br>
  True if any of the comma-separated selectors is true; for example
  <i>#[rustversion::any(stable, beta)]</i>.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::all(</code></b><i>selectors...</i><b><code style="display:inline">)]</code></b>
  —<br>
  True if all of the comma-separated selectors are true; for example
  <i>#[rustversion::all(since(1.31), before(1.34))]</i>.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">#[rustversion::attr(</code></b><i>selector</i><b><code style="display:inline">, </code></b><i>attribute</i><b><code style="display:inline">)]</code></b>
  —<br>
  For conditional inclusion of attributes; analogous to
  <code style="display:inline">cfg_attr</code>.
  </p>

- <p style="margin-left:50px;text-indent:-50px">
  <b><code style="display:inline">rustversion::cfg!(</code></b><i>selector</i><b><code style="display:inline">)</code></b>
  —<br>
  An expression form of any of the above attributes; for example
  <i>if rustversion::cfg!(any(stable, beta)) { ... }</i>.
  </p>

<br>

# Use cases

Providing additional trait impls as types are stabilized in the standard library
without breaking compatibility with older compilers; in this case Pin\<P\>
stabilized in [Rust 1.33][pin]:

[pin]: https://blog.rust-lang.org/2019/02/28/Rust-1.33.0.html#pinning

```rust
#[rustversion::since(1.33)]
use std::pin::Pin;

#[rustversion::since(1.33)]
impl<P: MyTrait> MyTrait for Pin<P> {
    /* ... */
}
```

Similar but for language features; the ability to control alignment greater than
1 of packed structs was stabilized in [Rust 1.33][packed].

[packed]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1330-2019-02-28

```rust
#[rustversion::attr(before(1.33), repr(packed))]
#[rustversion::attr(since(1.33), repr(packed(2)))]
struct Six(i16, i32);

fn main() {
    println!("{}", std::mem::align_of::<Six>());
}
```

Augmenting code with `const` as const impls are stabilized in the standard
library. This use of `const` as an attribute is recognized as a special case
by the rustversion::attr macro.

```rust
use std::time::Duration;

#[rustversion::attr(since(1.32), const)]
fn duration_as_days(dur: Duration) -> u64 {
    dur.as_secs() / 60 / 60 / 24
}
```

Emitting Cargo cfg directives from a build script. Note that this requires
listing `rustversion` under `[build-dependencies]` in Cargo.toml, not
`[dependencies]`.

```rust
// build.rs

fn main() {
    if rustversion::cfg!(since(1.36)) {
        println!("cargo:rustc-cfg=no_std");
    }
}
```

```rust
// src/lib.rs

#![cfg_attr(no_std, no_std)]

#[cfg(no_std)]
extern crate alloc;
```

<br>