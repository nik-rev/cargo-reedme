# httparse

A push library for parsing HTTP/1.x requests and responses.

The focus is on speed and safety. Unsafe code is used to keep parsing fast,
but unsafety is contained in a submodule, with invariants enforced. The
parsing internals use an `Iterator` instead of direct indexing, while
skipping bounds checks.

With Rust 1.27.0 or later, support for SIMD is enabled automatically.
If building an executable to be run on multiple platforms, and thus
not passing `target_feature` or `target_cpu` flags to the compiler,
runtime detection can still detect SSE4.2 or AVX2 support to provide
massive wins.

If compiling for a specific target, remembering to include
`-C target_cpu=native` allows the detection to become compile time checks,
making it *even* faster.