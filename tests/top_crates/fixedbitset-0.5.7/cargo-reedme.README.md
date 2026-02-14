`FixedBitSet` is a simple fixed size set of bits.

### Crate features

- `std` (default feature)  
  Disabling this feature disables using std and instead uses crate alloc.

### SIMD Acceleration
`fixedbitset` is written with SIMD in mind. The backing store and set operations will use aligned SIMD data types and instructions when compiling
for compatible target platforms. The use of SIMD generally enables better performance in many set and batch operations (i.e. intersection/union/inserting a range).

 When SIMD is not available on the target, the crate will gracefully fallback to a default implementation.  It is intended to add support for other SIMD architectures
once they appear in stable Rust.

Currently only SSE2/AVX/AVX2 on x86/x86_64 and wasm32 SIMD are supported as this is what stable Rust supports.