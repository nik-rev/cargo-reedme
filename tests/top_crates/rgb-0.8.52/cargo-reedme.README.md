Basic struct for `RGB` and `RGBA` pixels. Packed, with red first, alpha last.

This crate is intended to be the lowest common denominator for sharing `RGB`/`RGBA` bitmaps between other crates.

The crate includes convenience functions for converting between the struct and bytes,
and overloaded operators that work on all channels at once.

This crate intentionally doesn’t implement color management (due to complexity of the problem),
but the structs can be parametrized to implement this if necessary. Other colorspaces are out of scope.

```rust
let pixel = RGB8 {r:0, g:100, b:255};

let pixel_rgba = pixel.alpha(255);
let pixel = pixel_rgba.rgb();

let pixels = vec![pixel; 100];
/// Can be converted to a type-less slice without copying
let bytes: &[u8] = rgb::bytemuck::cast_slice(&pixels);

use rgb::prelude::*; // Import pixel map trait
let half_bright = pixel.map(|channel| channel / 2);
let doubled = half_bright * 2;
```