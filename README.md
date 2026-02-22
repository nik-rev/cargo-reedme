<!-- cargo-reedme: start -->

<!--
    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo reedme target/debug/cargo-reedme

    for more info: https://github.com/nik-rev/cargo-reedme
-->

# Configuration

You can configure the behavior of `cargo-reedme` via the `[metadata]` table in `Cargo.toml`:

```toml
# project/crates/foo_bar/Cargo.toml

[package.metadata.cargo-reedme]
# ...
```

```toml
# project//Cargo.toml

[workspace.metadata.cargo-reedme]
# ...
```

## `[metadata.cargo-reedme.format]`

By default, generated links will forward to `docs.rs`. This `lib.rs`:

```rust
//! This is an [Example]
```

Will generate the following `README.md`:

```markdown
This is an [Example](https://docs.rs/example/0.1.0/example/struct.Example.html)
```

It’s possible to use a custom format

<!-- cargo-reedme: end -->
