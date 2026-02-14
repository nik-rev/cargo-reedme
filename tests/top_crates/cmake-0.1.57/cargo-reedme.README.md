A build dependency for running `cmake` to build a native library

This crate provides some necessary boilerplate and shim support for running
the system `cmake` command to build a native library. It will add
appropriate cflags for building code to link into Rust, handle cross
compilation, and use the necessary generator for the platform being
targeted.

The builder-style configuration allows for various variables and such to be
passed down into the build as well.

## Installation

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
cmake = "0.1"
```

## Examples

```rust
use cmake;

// Builds the project in the directory located in `libfoo`, installing it
// into $OUT_DIR
let dst = cmake::build("libfoo");

println!("cargo:rustc-link-search=native={}", dst.display());
println!("cargo:rustc-link-lib=static=foo");
```

```rust
use cmake::Config;

let dst = Config::new("libfoo")
                 .define("FOO", "BAR")
                 .cflag("-foo")
                 .build();
println!("cargo:rustc-link-search=native={}", dst.display());
println!("cargo:rustc-link-lib=static=foo");
```