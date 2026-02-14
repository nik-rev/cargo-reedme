```rust
use ravif::*;
let res = Encoder::new()
    .with_quality(70.)
    .with_speed(4)
    .encode_rgba(Img::new(pixels, width, height))?;
std::fs::write("hello.avif", res.avif_file);
```