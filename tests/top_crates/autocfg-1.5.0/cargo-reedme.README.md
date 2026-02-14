A Rust library for build scripts to automatically configure code based on
compiler support.  Code snippets are dynamically tested to see if the `rustc`
will accept them, rather than hard-coding specific version support.


## Usage

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
autocfg = "1"
```

Then use it in your `build.rs` script to detect compiler features.  For
example, to test for 128-bit integer support, it might look like:

```rust
extern crate autocfg;

fn main() {
    let ac = autocfg::new();
    ac.emit_has_type("i128");

    // (optional) We don't need to rerun for anything external.
    autocfg::rerun_path("build.rs");
}
```

If the type test succeeds, this will write a `cargo:rustc-cfg=has_i128` line
for Cargo, which translates to Rust arguments `--cfg has_i128`.  Then in the
rest of your Rust code, you can add `#[cfg(has_i128)]` conditions on code that
should only be used when the compiler supports it.

## Caution

Many of the probing methods of `AutoCfg` document the particular template they
use, **subject to change**. The inputs are not validated to make sure they are
semantically correct for their expected use, so it’s _possible_ to escape and
inject something unintended. However, such abuse is unsupported and will not
be considered when making changes to the templates.