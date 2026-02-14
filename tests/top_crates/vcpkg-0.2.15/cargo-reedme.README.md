A build dependency for Cargo libraries to find libraries in a
[Vcpkg](https://github.com/microsoft/vcpkg) tree

From a Vcpkg package name
this build helper will emit cargo metadata to link it and it’s dependencies
(excluding system libraries, which it does not determine).

The simplest possible usage looks like this :-

```rust
// build.rs
vcpkg::find_package("libssh2").unwrap();
```

The cargo metadata that is emitted can be changed like this :-

```rust
// build.rs
vcpkg::Config::new()
    .emit_includes(true)
    .find_package("zlib").unwrap();
```

If the search was successful all appropriate Cargo metadata will be printed
to stdout.

# Static vs. dynamic linking
## Linux and Mac
At this time, vcpkg has a single triplet on macOS and Linux, which builds
static link versions of libraries. This triplet works well with Rust. It is also possible
to select a custom triplet using the `VCPKGRS_TRIPLET` environment variable.
## Windows
On Windows there are three
configurations that are supported for 64-bit builds and another three for 32-bit.
The default 64-bit configuration is `x64-windows-static-md` which is a
[community supported](https://github.com/microsoft/vcpkg/blob/master/docs/users/triplets.md#community-triplets)
configuration that is a good match for Rust - dynamically linking to the C runtime,
and statically linking to the packages in vcpkg.

Another option is to build a fully static
binary using `RUSTFLAGS=-Ctarget-feature=+crt-static`. This will link to libraries built
with vcpkg triplet `x64-windows-static`.

For dynamic linking, set `VCPKGRS_DYNAMIC=1` in the
environment. This will link to libraries built with vcpkg triplet `x64-windows`. If `VCPKGRS_DYNAMIC` is set, `cargo install` will
generate dynamically linked binaries, in which case you will have to arrange for
dlls from your Vcpkg installation to be available in your path.

# Environment variables

A number of environment variables are available to globally configure which
libraries are selected.

* `VCPKG_ROOT` - Set the directory to look in for a vcpkg installation. If
it is not set, vcpkg will use the user-wide installation if one has been
set up with `vcpkg integrate install`, and check the crate source and target
to see if a vcpkg tree has been created by [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg).

* `VCPKGRS_TRIPLET` - Use this to override vcpkg-rs’ default triplet selection with your own.
This is how to select a custom vcpkg triplet.

* `VCPKGRS_NO_FOO` - if set, vcpkg-rs will not attempt to find the
library named `foo`.

* `VCPKGRS_DISABLE` - if set, vcpkg-rs will not attempt to find any libraries.

* `VCPKGRS_DYNAMIC` - if set, vcpkg-rs will link to DLL builds of ports.
# Related tools
## cargo vcpkg
[`cargo vcpkg`](https://crates.io/crates/cargo-vcpkg) can fetch and build a vcpkg installation of
required packages from scratch. It merges package requirements specified in the `Cargo.toml` of
crates in the dependency tree.  
## vcpkg_cli
There is also a rudimentary companion crate, `vcpkg_cli` that allows testing of environment
and flag combinations.

```Batchfile
C:\src> vcpkg_cli probe -l static mysqlclient
Found library mysqlclient
Include paths:
        C:\src\[..]\vcpkg\installed\x64-windows-static\include
Library paths:
        C:\src\[..]\vcpkg\installed\x64-windows-static\lib
Cargo metadata:
        cargo:rustc-link-search=native=C:\src\[..]\vcpkg\installed\x64-windows-static\lib
        cargo:rustc-link-lib=static=mysqlclient
```