# `cargo-reedme`

<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo reedme

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

[![crates.io](https://img.shields.io/crates/v/cargo-reedme?style=flat-square&logo=rust)](https://crates.io/crates/cargo-reedme)
[![docs.rs](https://img.shields.io/docsrs/cargo-reedme?style=flat-square&logo=docs.rs)](https://docs.rs/cargo-reedme)
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-1.93-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/cargo-reedme)](https://github.com/nik-rev/cargo-reedme)

Generate `README.md` from documentation comments in `lib.rs` or `main.rs`

- [Example](#example)
- [Installation](#installation)
- [Features](#features)

# Example

The following documentation in `lib.rs`:

````rust
//! This crate provides a procedural macro [`docstr!`] for
//! ergonomically creating multi-line string literals.
//!
//! ```toml
//! [dependencies]
#![doc = concat!("docstr = '", env!("CARGO_PKG_VERSION"), "'")]
//! ```
//!
//! # Usage
//!
//! ```
//! # use docstr::docstr;
//! #
//! let hello_world_in_c: &'static str = docstr!(
//!     /// #include <stdio.h>
//!     ///
//!     /// int main(int argc, char **argv) {
//!     ///     printf("hello world\n");
//!     ///     return 0;
//!     /// }
//! );
//!
//! assert_eq!(hello_world_in_c, r#"#include <stdio.h>
//!
//! int main(int argc, char **argv) {
//!     printf("hello world\n");
//!     return 0;
//! }"#)
//! ```
````

Generates the following `README.md` when running `cargo reedme`:

````markdown
This crate provides a procedural macro [`docstr!`](https://docs.rs/docstr/0.4.6/docstr/macro.docstr.html) for
ergonomically creating multi-line string literals.

```toml
[dependencies]
docstr = '0.4.6'
```

# Usage

```rust
let hello_world_in_c: &'static str = docstr!(
    /// #include <stdio.h>
    ///
    /// int main(int argc, char **argv) {
    ///     printf("hello world\n");
    ///     return 0;
    /// }
);

assert_eq!(hello_world_in_c, r#"#include <stdio.h>

int main(int argc, char **argv) {
    printf("hello world\n");
    return 0;
}"#)
```
````

# Installation

```sh
cargo install cargo-reedme
```

# Features

- [Generate `README.md` from documentation comments in `lib.rs`](#generate-readmemd-from-documentation-comments-in-librs)
- [Intra-doc link resolution](#intra-doc-link-resolution)
- [Code blocks transformation](#code-blocks-transformation)
- [Macro expansion](#macro-expansion)
- [Check mode](#check-mode)
- [Workspace support](#workspace-support)
- [Cargo features](#cargo-features)
- [Informational note](#informational-note)
- [Config](#config)
- [Use programatically from scripts](#use-programatically-from-scripts)

## Generate `README.md` from documentation comments in `lib.rs`

Running `cargo reedme` will take your doc comments and generate a README from them. This `src/lib.rs`:

```rust
//! Hello, world!
```

Generates the following `README.md`:

```markdown
<!-- cargo-reedme: start -->

Hello, world!

<!-- cargo-reedme: end -->
```

If the `README.md` file already exists, it must have a `<!-- cargo-reedme -->` somewhere inside of it. If the `README.md` contains the following:

```markdown
# my_crate

<!-- cargo-reedme -->
```

Running `cargo reedme` will replace that `<!-- cargo-reedme -->` with documentation from `lib.rs`:

```markdown
# my_crate

<!-- cargo-reedme: start -->

Hello, world!

<!-- cargo-reedme: end -->
```

Further invocations of `cargo reedme` update the inserted region

## Intra-doc link resolution

Intra-doc links will be transformed into absolute URLs:

```rust
//! This data structure is [`serde_json::Value`](Value).
struct Value;
```

The above generate the following `README.md`:

```markdown
This data structure is [`serde_json::Value`](https://docs.rs/serde_json/1.0.149/serde_json/enum.Value.html).
```

The generated link format is fully configurable.

## Code blocks transformation

Code blocks will have `rust` language added, and hidden lines (starting with `#`) will be removed:

````rust
//! An example program:
//!
//! ```
//! # fn main() {
//! // "hello world" in Rust
//! println!("Hello, world!");
//! # }
//! ```
````

The above generates the following `README.md`:

````markdown
An example program:

```rust
// "hello world" in Rust
println!("Hello, world!");
```
````

## Macro expansion

Macros in doc comments get properly expanded:

````rust
//! ```toml
//! [dependencies]
#![doc = concat!("derive_aliases = '", env!("CARGO_PKG_VERSION"), "'")]
//! ```
````

The above generates the following `README.md`:

````markdown
```toml
[dependencies]
derive_aliases = '0.4'
```
````

Notice that the `concat!` and inner `env!` macro was expanded appropriately.

## Check mode

Run `cargo-reedme` as part of your CI pipeline!

The `--check` flag is used to make sure PRs keep the `README.md` up to date with `lib.rs` doc comments.

An example workflow that runs `cargo reedme --check` on every commit and PR:

```yaml
# .github/workflows/cargo-reedme.yaml
name: cargo-reedme
on:
  pull_request:
  push:
    branches:
      - main

jobs:
  cargo-reedme:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - run: cargo install --locked cargo-reedme

      - run: cargo-reedme --check
```

On failure, the program exits with a non-zero exit code and prints a colorful diff between the **expected** and **actual** `README.md` files

## Workspace support

Generate `README.md`s for all crates in your workspace with a single command!

Supports `--workspace`, `--exclude`, and `--package`

## Cargo features

Supports `--all-features`, `--features`, and `--no-default-features`

## Informational note

When `cargo reedme` generates your `README.md` file, it will insert a comment that explains how this section was generated:

```markdown
# my_crate

<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo reedme

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

Your documentation

<!-- cargo-reedme: end -->
```

This note can be configured.

## Config

You can configure the behavior of `cargo-reedme` via the crate-level `[package.metadata]` table in `Cargo.toml`:

```toml
# project/crates/foo_bar/Cargo.toml

[package.metadata.cargo-reedme]
# ... your settings go here ...
```

…or the workspace-level `[workspace.metadata]` table

```toml
# project/Cargo.toml

[workspace.metadata.cargo-reedme]
# ... your settings go here ...
```

The default config is this:

```toml
#! This is the default configuration for `cargo-reedme`
#!
#! These fields can be overridden in `[package.metadata.cargo-reedme]` or `[workspace.metadata.cargo-reedme]`

# The base URL for every generated URL in the README
#
# https://docs.rs/serde_json/1.0.149/serde_json/enum.Value.html
# ^^^^^^^^^^^^^^^
base-url = "https://docs.rs"

# The note that appears at the beginning of the generated section.
#
# When running with `--check`, the note can differ. 2 README files are considered the same
# regardless of the difference between them is their "note" section.
#
# Available values for interpolation:
#
# - `args`: Command-line arguments received
note = """
Do not edit this region by hand
===============================

This region was generated from Rust documentation comments by `cargo-reedme` using this command:

    cargo reedme {args}

for more info: https://github.com/nik-rev/cargo-reedme
"""
```

Fun fact: This README itself is generated by `cargo reedme`. The above TOML is added from the [`default_config.toml`](https://github.com/nik-rev/cargo-reedme/blob/main/default_config.toml) file like this:

```rust
#![doc = include_str!("../default_config.toml")]
```

## Use programmatically from scripts

The `--json` flag can be used to have `cargo-reedme` do all computation but not write any files, so you can
do with that data as you please.

You can also use `cargo-reedme` as a crate. 95% of the `cargo-reedme`’s logic lives in a single, pure function [`cargo_reedme::resolve`](https://docs.rs/cargo-reedme/0.1.0/cargo_reedme/fn.resolve.html)
which does no IO. It has the following signature:

```rust
pub fn resolve(world: &World) -> Result<Output>
```

You can take a look at `main.rs` to see how this function is called

# Inspired by

- [`cargo-readme`](https://github.com/webern/cargo-readme)
- [`cargo-rdme`](https://github.com/orium/cargo-rdme)

<!-- cargo-reedme: end -->
