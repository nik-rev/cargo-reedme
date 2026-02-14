A library for [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
to compile a set of C/C++/assembly/CUDA files into a static archive for Cargo
to link into the crate being built. This crate does not compile code itself;
it calls out to the default compiler for the platform. This crate will
automatically detect situations such as cross compilation and
[various environment variables](#external-configuration-via-environment-variables) and will build code appropriately.

# Example

First, you’ll want to both add a build script for your crate (`build.rs`) and
also add this crate to your `Cargo.toml` via:

```toml
[build-dependencies]
cc = "1.0"
```

Next up, you’ll want to write a build script like so:

```rust
// build.rs
cc::Build::new()
    .file("foo.c")
    .file("bar.c")
    .compile("foo");
```

And that’s it! Running `cargo build` should take care of the rest and your Rust
application will now have the C files `foo.c` and `bar.c` compiled into a file
named `libfoo.a`. If the C files contain

```c
void foo_function(void) { ... }
```

and

```c
int32_t bar_function(int32_t x) { ... }
```

you can call them from Rust by declaring them in
your Rust code like so:

```rust
extern "C" {
    fn foo_function();
    fn bar_function(x: i32) -> i32;
}

pub fn call() {
    unsafe {
        foo_function();
        bar_function(42);
    }
}

fn main() {
    call();
}
```

See [the Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html) for more details.

# External configuration via environment variables

To control the programs and flags used for building, the builder can set a
number of different environment variables.

* `CFLAGS` - a series of space separated flags passed to compilers. Note that
  individual flags cannot currently contain spaces, so doing
  something like: `-L=foo\ bar` is not possible.
* `CC` - the actual C compiler used. Note that this supports passing a known
  wrapper via `sccache cc`. This compiler must understand the `-c` flag. For
  certain `TARGET`s, it also is assumed to know about other flags (most
  common is `-fPIC`).
  ccache, distcc, sccache, icecc, cachepot and buildcache are supported,
  for sccache, simply set `CC` to `sccache cc`.
  For other custom `CC` wrapper, just set `CC_KNOWN_WRAPPER_CUSTOM`
  to the custom wrapper used in `CC`.
* `AR` - the `ar` (archiver) executable to use to build the static library.
* `CRATE_CC_NO_DEFAULTS` - the default compiler flags may cause conflicts in
  some cross compiling scenarios. Setting this variable
  will disable the generation of default compiler
  flags.
* `CC_ENABLE_DEBUG_OUTPUT` - if set, compiler command invocations and exit codes will
  be logged to stdout. This is useful for debugging build script issues, but can be
  overly verbose for normal use.
* `CC_SHELL_ESCAPED_FLAGS` - if set, `*FLAGS` will be parsed as if they were shell
  arguments (similar to `make` and `cmake`) rather than splitting them on each space.
  For example, with `CFLAGS='a "b c"'`, the compiler will be invoked with 2 arguments -
  `a` and `b c` - rather than 3: `a`, `"b` and `c"`.
* `CXX...` - see [C++ Support](#c-support).
* `CC_FORCE_DISABLE` - If set, `cc` will never run any [`Command`](https://doc.rust-lang.org/stable/std/process/struct.Command.html)s, and methods that
  would return an [`Error`](https://docs.rs/cc/1.2.56/cc/struct.Error.html). This is intended for use by third-party build systems
  which want to be absolutely sure that they are in control of building all
  dependencies. Note that operations that return [`Tool`](https://docs.rs/cc/1.2.56/cc/tool/struct.Tool.html)s such as
  [`Build::get_compiler`](https://docs.rs/cc/1.2.56/cc/struct.Build.html#method.get_compiler) may produce less accurate results as in some cases `cc` runs
  commands in order to locate compilers. Additionally, this does nothing to prevent
  users from running [`Tool::to_command`](https://docs.rs/cc/1.2.56/cc/tool/struct.Tool.html#method.to_command) and executing the [`Command`](https://doc.rust-lang.org/stable/std/process/struct.Command.html) themselves.
* `RUSTC_WRAPPER` - If set, the specified command will be prefixed to the compiler
  command. This is useful for projects that want to use
  [sccache](https://github.com/mozilla/sccache),
  [buildcache](https://gitlab.com/bits-n-bites/buildcache), or
  [cachepot](https://github.com/paritytech/cachepot).

Furthermore, projects using this crate may specify custom environment variables
to be inspected, for example via the `Build::try_flags_from_environment`
function. Consult the project’s own documentation or its use of the `cc` crate
for any additional variables it may use.

Each of these variables can also be supplied with certain prefixes and suffixes,
in the following prioritized order:

  1. `<var>_<target>` - for example, `CC_x86_64-unknown-linux-gnu` or `CC_thumbv8m.main-none-eabi`
  2. `<var>_<target_with_underscores>` - for example, `CC_x86_64_unknown_linux_gnu` or `CC_thumbv8m_main_none_eabi` (both periods and underscores are replaced)
  3. `<build-kind>_<var>` - for example, `HOST_CC` or `TARGET_CFLAGS`
  4. `<var>` - a plain `CC`, `AR` as above.

If none of these variables exist, cc-rs uses built-in defaults.

In addition to the above optional environment variables, `cc-rs` has some
functions with hard requirements on some variables supplied by [cargo’s
build-script driver][cargo] that it has the `TARGET`, `OUT_DIR`, `OPT_LEVEL`,
and `HOST` variables.

[cargo]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#inputs-to-the-build-script

# Optional features

## Parallel

Currently cc-rs supports parallel compilation (think `make -jN`) but this
feature is turned off by default. To enable cc-rs to compile C/C++ in parallel,
you can change your dependency to:

```toml
[build-dependencies]
cc = { version = "1.0", features = ["parallel"] }
```

By default cc-rs will limit parallelism to `$NUM_JOBS`, or if not present it
will limit it to the number of cpus on the machine. If you are using cargo,
use `-jN` option of `build`, `test` and `run` commands as `$NUM_JOBS`
is supplied by cargo.

# Compile-time Requirements

To work properly this crate needs access to a C compiler when the build script
is being run. This crate does not ship a C compiler with it. The compiler
required varies per platform, but there are three broad categories:

* Unix platforms require `cc` to be the C compiler. This can be found by
  installing cc/clang on Linux distributions and Xcode on macOS, for example.
* Windows platforms targeting MSVC (e.g. your target name ends in `-msvc`)
  require Visual Studio to be installed. `cc-rs` attempts to locate it, and
  if it fails, `cl.exe` is expected to be available in `PATH`. This can be
  set up by running the appropriate developer tools shell.
* Windows platforms targeting MinGW (e.g. your target name ends in `-gnu`)
  require `cc` to be available in `PATH`. We recommend the
  [MinGW-w64](https://www.mingw-w64.org/) distribution.
  You may also acquire it via
  [MSYS2](https://www.msys2.org/), as explained [here][msys2-help].  Make sure
  to install the appropriate architecture corresponding to your installation of
  rustc. GCC from older [MinGW](http://www.mingw.org/) project is compatible
  only with 32-bit rust compiler.

[msys2-help]: https://github.com/rust-lang/rust/blob/master/INSTALL.md#building-on-windows

# C++ support

`cc-rs` supports C++ libraries compilation by using the `cpp` method on
`Build`:

```rust
cc::Build::new()
    .cpp(true) // Switch to C++ library compilation.
    .file("foo.cpp")
    .compile("foo");
```

For C++ libraries, the `CXX` and `CXXFLAGS` environment variables are used instead of `CC` and `CFLAGS`.

The C++ standard library may be linked to the crate target. By default it’s `libc++` for macOS, FreeBSD, and OpenBSD, `libc++_shared` for Android, nothing for MSVC, and `libstdc++` for anything else. It can be changed in one of two ways:

1. by using the `cpp_link_stdlib` method on `Build`:
```rust
cc::Build::new()
    .cpp(true)
    .file("foo.cpp")
    .cpp_link_stdlib("stdc++") // use libstdc++
    .compile("foo");
```
2. by setting the `CXXSTDLIB` environment variable.

In particular, for Android you may want to [use `c++_static` if you have at most one shared library](https://developer.android.com/ndk/guides/cpp-support).

Remember that C++ does name mangling so `extern "C"` might be required to enable Rust linker to find your functions.

# CUDA C++ support

`cc-rs` also supports compiling CUDA C++ libraries by using the `cuda` method
on `Build`:

```rust
cc::Build::new()
    // Switch to CUDA C++ library compilation using NVCC.
    .cuda(true)
    .cudart("static")
    // Generate code for Maxwell (GTX 970, 980, 980 Ti, Titan X).
    .flag("-gencode").flag("arch=compute_52,code=sm_52")
    // Generate code for Maxwell (Jetson TX1).
    .flag("-gencode").flag("arch=compute_53,code=sm_53")
    // Generate code for Pascal (GTX 1070, 1080, 1080 Ti, Titan Xp).
    .flag("-gencode").flag("arch=compute_61,code=sm_61")
    // Generate code for Pascal (Tesla P100).
    .flag("-gencode").flag("arch=compute_60,code=sm_60")
    // Generate code for Pascal (Jetson TX2).
    .flag("-gencode").flag("arch=compute_62,code=sm_62")
    // Generate code in parallel
    .flag("-t0")
    .file("bar.cu")
    .compile("bar");
```

# Speed up compilation with sccache

`cc-rs` does not handle incremental compilation like `make` or `ninja`. It
always compiles the all sources, no matter if they have changed or not.
This would be time-consuming in large projects. To save compilation time,
you can use [sccache](https://github.com/mozilla/sccache) by setting
environment variable `RUSTC_WRAPPER=sccache`, which will use cached `.o`
files if the sources are unchanged.