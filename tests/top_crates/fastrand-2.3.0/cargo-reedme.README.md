A simple and fast random number generator.

The implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast
generator but **not** cryptographically secure.

# Examples

Flip a coin:

```rust
if fastrand::bool() {
    println!("heads");
} else {
    println!("tails");
}
```

Generate a random `i32`:

```rust
let num = fastrand::i32(..);
```

Choose a random element in an array:

```rust
let v = vec![1, 2, 3, 4, 5];
let i = fastrand::usize(..v.len());
let elem = v[i];
```

Sample values from an array with `O(n)` complexity (`n` is the length of array):

```rust
fastrand::choose_multiple([1, 4, 5], 2);
fastrand::choose_multiple(0..20, 12);
```


Shuffle an array:

```rust
let mut v = vec![1, 2, 3, 4, 5];
fastrand::shuffle(&mut v);
```

Generate a random [`Vec`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html) or [`String`]:

```rust
use std::iter::repeat_with;

let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10).collect();
let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
```

To get reproducible results on every run, initialize the generator with a seed:

```rust
// Pick an arbitrary number as seed.
fastrand::seed(7);

// Now this prints the same number on every run:
println!("{}", fastrand::u32(..));
```

To be more efficient, create a new [`Rng`](https://docs.rs/fastrand/2.3.0/fastrand/struct.Rng.html) instance instead of using the thread-local
generator:

```rust
use std::iter::repeat_with;

let mut rng = fastrand::Rng::new();
let mut bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
```

This crate aims to expose a core set of useful randomness primitives. For more niche algorithms,
consider using the [`fastrand-contrib`] crate alongside this one.

# Features

- `std` (enabled by default): Enables the `std` library. This is required for the global
  generator and global entropy. Without this feature, [`Rng`](https://docs.rs/fastrand/2.3.0/fastrand/struct.Rng.html) can only be instantiated using
  the [`with_seed`](https://docs.rs/fastrand/2.3.0/fastrand/struct.Rng.html#method.with_seed) method.
- `js`: Assumes that WebAssembly targets are being run in a JavaScript environment. See the
  [WebAssembly Notes](#webassembly-notes) section for more information.

# WebAssembly Notes

For non-WASI WASM targets, there is additional sublety to consider when utilizing the global RNG.
By default, `std` targets will use entropy sources in the standard library to seed the global RNG.
However, these sources are not available by default on WASM targets outside of WASI.

If the `js` feature is enabled, this crate will assume that it is running in a JavaScript
environment. At this point, the [`getrandom`] crate will be used in order to access the available
entropy sources and seed the global RNG. If the `js` feature is not enabled, the global RNG will
use a predefined seed.

[`fastrand-contrib`]: https://crates.io/crates/fastrand-contrib
[`getrandom`]: https://crates.io/crates/getrandom