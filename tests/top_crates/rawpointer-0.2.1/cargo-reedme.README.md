Rawpointer adds extra utility methods to raw pointers `*const T`, `*mut T`
and `NonNull<T>`.

Features include:

- Strided offsets - [`.stride_offset(stride,
  index)`](https://docs.rs/rawpointer/0.2.1/rawpointer/trait.PointerExt.html#tymethod.stride_offset) make it easy to compute
  pointer offsets where the index is unsigned and the stride is signed.

- Offsetting methods in general for `NonNull`, since it does not have these
  from libcore

- Post- and preincrement and post- and predecrement methods

  - For `p++` use [`p.post_inc()`](https://docs.rs/rawpointer/0.2.1/rawpointer/trait.PointerExt.html#tymethod.post_inc).
  - For `++p` use [`p.pre_inc()`](https://docs.rs/rawpointer/0.2.1/rawpointer/trait.PointerExt.html#tymethod.pre_inc).
  - For `p--` use [`p.post_dec()`](https://docs.rs/rawpointer/0.2.1/rawpointer/trait.PointerExt.html#tymethod.post_dec).
  - For `--p` use [`p.pre_dec()`](https://docs.rs/rawpointer/0.2.1/rawpointer/trait.PointerExt.html#tymethod.pre_dec).

```rust
use rawpointer::PointerExt;

unsafe {
    // In this example:
    // Use .post_inc() to iterate and overwrite the first four
    // elements of the array.

    let mut xs = [0; 16];
    let mut ptr = xs.as_mut_ptr();
    let end = ptr.offset(4);
    let mut i = 0;
    while ptr != end {
        *ptr.post_inc() = i;
        i += 1;
    }
    assert_eq!(&xs[..8], &[0, 1, 2, 3, 0, 0, 0, 0]);
}
```

## Safety

See the Rust [core::ptr](https://doc.rust-lang.org/stable/core/ptr/) documentation for more information.

## Rust Version

This version of the crate requires Rust 1.26 or later