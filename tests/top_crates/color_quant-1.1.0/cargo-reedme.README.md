# Color quantization library

This library provides a color quantizer based on the [NEUQUANT](http://members.ozemail.com.au/~dekker/NEUQUANT.HTML)

Original literature: Dekker, A. H. (1994). Kohonen neural networks for
optimal colour quantization. *Network: Computation in Neural Systems*, 5(3), 351-367.
[doi: 10.1088/0954-898X_5_3_003](https://doi.org/10.1088/0954-898X_5_3_003)

See also <https://scientificgems.wordpress.com/stuff/neuquant-fast-high-quality-image-quantization/>

## Usage

```rust
let data = vec![0; 40];
let nq = color_quant::NeuQuant::new(10, 256, &data);
let indixes: Vec<u8> = data.chunks(4).map(|pix| nq.index_of(pix) as u8).collect();
let color_map = nq.color_map_rgba();
```