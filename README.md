<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo reedme 

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

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
