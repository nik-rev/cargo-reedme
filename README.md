<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo reedme 

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

Features:

- **Link mapping:** Intra-doc links will be transformed into absolute URLs. These doc comments:

  ```rust
  /// This data structure is [`serde_json::Value`](Value).
  struct Value;
  ```

  Generate the following `README.md`:

  ```markdown
  This data structure is [`serde_json::Value`](https://docs.rs/serde_json/1.0.149/serde_json/enum.Value.html).
  ```

  The generated link format is fully configurable.

- **Workspace support**: Generate `README.md`s for all crates in your workspace with a single command! Supports `--workspace`, `--exclude`, and `--package`

- **Cargo features**: supports `--all-features`, `--features`, and `--no-default-features`

- **Code blocks transformation**: Code blocks will have `rust` language added, and hidden lines (starting with `#`) will be removed:

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

  Generates the following `README.md`:

  ````markdown
  An example program:

  ```rust
  // "hello world" in Rust
  println!("Hello, world!");
  ```
  ````

- **Full doc comments support**: Macros in doc comments get properly expanded. These doc comments:

  ````rust
  //! ```toml
  //! [dependencies]
  #![doc = concat!("derive_aliases = '", env!("CARGO_PKG_VERSION"), "'")]
  //! ```
  ````

  Generate the following `README.md`:

  ````markdown
  ```toml
  [dependencies]
  derive_aliases = '0.4'
  ```
  ````

  Notice that the `concat!` and inner `env!` macro was expanded appropriately.

Inspired by:

- [`cargo-readme`](https://github.com/webern/cargo-readme)
- [`cargo-rdme`](https://github.com/orium/cargo-rdme)

# Configuration

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

Crate-level configuration will take priority over workspace-level

<!-- cargo-reedme: end -->
