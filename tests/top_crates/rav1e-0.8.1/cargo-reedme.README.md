rav1e is an [AV1] video encoder. It is designed to eventually cover all use
cases, though in its current form it is most suitable for cases where
libaom (the reference encoder) is too slow.

## Features

* Intra and inter frames
* 64x64 superblocks
* 4x4 to 64x64 RDO-selected square and 2:1/1:2 rectangular blocks
* DC, H, V, Paeth, smooth, and a subset of directional prediction modes
* DCT, (FLIP-)ADST and identity transforms (up to 64x64, 16x16 and 32x32
  respectively)
* 8-, 10- and 12-bit depth color
* 4:2:0 (full support), 4:2:2 and 4:4:4 (limited) chroma sampling
* Variable speed settings
* Near real-time encoding at high speed levels

## Usage

Encoding is done through the [`Context`] struct. Examples on
[`Context::receive_packet`] show how to create a [`Context`], send frames
into it and receive packets of encoded data.

[AV1]: https://aomediacodec.github.io/av1-spec/av1-spec.pdf
[`Context`]: struct.Context.html
[`Context::receive_packet`]: struct.Context.html#method.receive_packet