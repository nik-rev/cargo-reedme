# Overview

This crate provides native rust implementations of image encoding and decoding as well as some
basic image manipulation functions. Additional documentation can currently also be found in the
[README.md file which is most easily viewed on
github](https://github.com/image-rs/image/blob/main/README.md).

There are two core problems for which this library provides solutions: a unified interface for image
encodings and simple generic buffers for their content. It’s possible to use either feature
without the other. The focus is on a small and stable set of common operations that can be
supplemented by other specialized crates. The library also prefers safe solutions with few
dependencies.

# High level API

Load images using [`ImageReader`](https://docs.rs/image/0.25.9/image/io/image_reader_type/struct.ImageReader.html):

```rust
use std::io::Cursor;
use image::ImageReader;

let img = ImageReader::open("myimage.png")?.decode()?;
let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
```

And save them using [`save`] or [`write_to`] methods:

```rust
img.save("empty.jpg")?;

let mut bytes: Vec<u8> = Vec::new();
img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;
```

With default features, the crate includes support for [many common image formats](codecs/index.html#supported-formats).

[`save`]: enum.DynamicImage.html#method.save
[`write_to`]: enum.DynamicImage.html#method.write_to
[`ImageReader`]: struct.Reader.html

# Image buffers

The two main types for storing images:
* [`ImageBuffer`] which holds statically typed image contents.
* [`DynamicImage`] which is an enum over the supported `ImageBuffer` formats
  and supports conversions between them.

As well as a few more specialized options:
* [`GenericImage`] trait for a mutable image buffer.
* [`GenericImageView`] trait for read only references to a `GenericImage`.
* [`flat`] module containing types for interoperability with generic channel
  matrices and foreign interfaces.

[`GenericImageView`]: trait.GenericImageView.html
[`GenericImage`]: trait.GenericImage.html
[`ImageBuffer`]: struct.ImageBuffer.html
[`DynamicImage`]: enum.DynamicImage.html
[`flat`]: flat/index.html

# Low level encoding/decoding API

Implementations of [`ImageEncoder`] provides low level control over encoding:
```rust
let encoder = JpegEncoder::new_with_quality(&mut writer, 95);
img.write_with_encoder(encoder)?;
```
While [`ImageDecoder`] and [`ImageDecoderRect`] give access to more advanced decoding options:

```rust
let decoder = PngDecoder::new(&mut reader)?;
let icc = decoder.icc_profile();
let img = DynamicImage::from_decoder(decoder)?;
```

[`DynamicImage::from_decoder`]: enum.DynamicImage.html#method.from_decoder
[`ImageDecoderRect`]: trait.ImageDecoderRect.html
[`ImageDecoder`]: trait.ImageDecoder.html
[`ImageEncoder`]: trait.ImageEncoder.html