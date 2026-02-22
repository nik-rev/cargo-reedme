<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo reedme 

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

Features:

- Everything is resolved by rustdoc, so all the links will work

- **All** doc comments work. So, macros in doc comments expanded. These doc comments:

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
