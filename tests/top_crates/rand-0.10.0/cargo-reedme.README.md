Utilities for random number generation

Rand provides utilities to generate random numbers, to convert them to
useful types and distributions, and some randomness-related algorithms.

# Quick Start

```rust
// The prelude import enables methods we use below, specifically
// Rng::random, Rng::sample, SliceRandom::shuffle and IndexedRandom::choose.
use rand::prelude::*;

// Get an RNG:
let mut rng = rand::rng();

// Try printing a random unicode code point (probably a bad idea)!
println!("char: '{}'", rng.random::<char>());
// Try printing a random alphanumeric value instead!
println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);

// Generate and shuffle a sequence:
let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
// And take a random pick (yes, we didn't need to shuffle first!):
let _ = nums.choose(&mut rng);
```

# The Book

For the user guide and further documentation, please read
[The Rust Rand Book](https://rust-random.github.io/book).