A crate to help you go wide.

This crate provides SIMD-compatible data types.

When possible, explicit SIMD is used with all the math operations here. As a
fallback, the fact that all the lengths of a fixed length array are doing
the same thing will often make LLVM notice that it should use SIMD
instructions to complete the task. In the worst case, the code just becomes
totally scalar (though the math is still correct, at least).

## Crate Features

* `std`: This causes the feature to link to `std`.
  * Currently this just improves the performance of `sqrt` when an explicit
    SIMD `sqrt` isn’t available.