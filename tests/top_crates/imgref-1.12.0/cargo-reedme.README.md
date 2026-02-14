In graphics code it’s very common to pass `width` and `height` along with a `Vec` of pixels,
all as separate arguments. This is tedious, and can lead to errors.

This crate is a simple struct that adds dimensions to the underlying buffer. This makes it easier to correctly keep track
of the image size and allows passing images with just one function argument instead three or four.

Additionally, it has a concept of a `stride`, which allows defining sub-regions of images without copying,
as well as handling padding (e.g. buffers for video frames may require to be a multiple of 8, regardless of logical image size).

For convenience, there are iterators over rows or all pixels of a (sub)image and
pixel-based indexing directly with `img[(x,y)]` (where `x`/`y` can be `u32` as well as `usize`).

`Img<Container>` type has aliases for common uses:

* Owned: `ImgVec<T>` → `Img<Vec<T>>`  (use it in `struct`s and return types)
* Reference: `ImgRef<T>` → `Img<&[T]>` (use it in function arguments)
* Mutable reference: `ImgRefMut<T>` → `Img<&mut [T]>`

It is assumed that the container is [one element per pixel](https://crates.io/crates/rgb/), e.g. `Vec<RGBA>`,
and _not_ a `Vec<u8>` where 4 `u8` elements are interpreted as one pixel.


 ```rust
 use imgref::*;

 fn main() {
     let img = Img::new(vec![0; 1000], 50, 20); // 1000 pixels of a 50×20 image

     let new_image = some_image_processing_function(img.as_ref()); // Use imgvec.as_ref() instead of &imgvec for better efficiency

     println!("New size is {}×{}", new_image.width(), new_image.height());
     println!("And the top left pixel is {:?}", new_image[(0u32,0u32)]);

     let first_row_slice = &new_image[0];

     for row in new_image.rows() {
         // …
     }
     for px in new_image.pixels() {
         // …
     }

     // slice (x, y, width, height) by reference - no copy!
     let fragment = img.sub_image(5, 5, 15, 15);

     //
     let (vec, width, height) = fragment.to_contiguous_buf();
 }
```