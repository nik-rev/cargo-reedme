Simple library for getting the version information of a `rustc`
compiler.

This can be used by build scripts or other tools dealing with Rust sources
to make decisions based on the version of the compiler.

It calls `$RUSTC --version -v` and parses the output, falling
back to `rustc` if `$RUSTC` is not set.

# Example

```rust
// This could be a cargo build script

use rustc_version::{version, version_meta, Channel, Version};

// Assert we haven't travelled back in time
assert!(version().unwrap().major >= 1);

// Set cfg flags depending on release channel
match version_meta().unwrap().channel {
    Channel::Stable => {
        println!("cargo:rustc-cfg=RUSTC_IS_STABLE");
    }
    Channel::Beta => {
        println!("cargo:rustc-cfg=RUSTC_IS_BETA");
    }
    Channel::Nightly => {
        println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY");
    }
    Channel::Dev => {
        println!("cargo:rustc-cfg=RUSTC_IS_DEV");
    }
}

// Check for a minimum version
if version().unwrap() >= Version::parse("1.4.0").unwrap() {
    println!("cargo:rustc-cfg=compiler_has_important_bugfix");
}
```