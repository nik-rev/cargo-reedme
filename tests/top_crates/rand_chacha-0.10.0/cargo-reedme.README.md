The ChaCha random number generators.

These are native Rust implementations of RNGs derived from the
[ChaCha stream ciphers] by D J Bernstein.

## Generators

This crate provides 8-, 12- and 20-round variants of generators via a “core”
implementation (of [`block::Generator`]), each with an associated “RNG” type
(implementing [`Rng`]).

These generators are all deterministic and portable (see [Reproducibility]
in the book), with testing against reference vectors.

## Cryptographic (secure) usage

Where secure unpredictable generators are required, it is suggested to use
[`ChaCha12Rng`](https://docs.rs/rand_chacha/0.10.0/rand_chacha/chacha/struct.ChaCha12Rng.html) or [`ChaCha20Rng`](https://docs.rs/rand_chacha/0.10.0/rand_chacha/chacha/struct.ChaCha20Rng.html) and to seed via
[`SysRng`].

See also the [Security] chapter in the rand book. The crate is provided
“as is”, without any form of guarantee, and without a security audit.

## Seeding (construction)

Generators implement the [`SeedableRng`] trait. Any method may be used,
but note that `seed_from_u64` is not suitable for usage where security is
important. Some suggestions:

1.  With a fresh seed, **direct from the OS** (implies a syscall):
    ```rust
    let rng = ChaCha12Rng::try_from_rng(&mut SysRng).unwrap();
```
2.  **From a master generator.** This could be [`rand::rng`]
    (effectively a fresh seed without the need for a syscall on each usage)
    or a deterministic generator such as [`ChaCha20Rng`](https://docs.rs/rand_chacha/0.10.0/rand_chacha/chacha/struct.ChaCha20Rng.html).
    Beware that should a weak master generator be used, correlations may be
    detectable between the outputs of its child generators.
    ```rust
    let rng = ChaCha12Rng::from_rng(&mut rand::rng());
```

See also [Seeding RNGs] in the book.

## Generation

Generators implement [`Rng`], whose methods may be used directly to
generate unbounded integer or byte values.
```rust
use rand_core::{SeedableRng, Rng};
use rand_chacha::ChaCha12Rng;

let mut rng = ChaCha12Rng::from_seed(Default::default());
let x = rng.next_u64();
assert_eq!(x, 0x53f955076a9af49b);
```

It is often more convenient to use the [`rand::Rng`] trait, which provides
further functionality. See also the [Random Values] chapter in the book.

[ChaCha stream ciphers]: https://cr.yp.to/chacha.html
[Reproducibility]: https://rust-random.github.io/book/crate-reprod.html
[Seeding RNGs]: https://rust-random.github.io/book/guide-seeding.html
[Security]: https://rust-random.github.io/book/guide-rngs.html#security
[Random Values]: https://rust-random.github.io/book/guide-values.html
[`block::Generator`]: https://docs.rs/rand_core/latest/rand_core/block/trait.Generator.html
[`Rng`]: https://docs.rs/rand_core/latest/rand_core/trait.Rng.html
[`SeedableRng`]: https://docs.rs/rand_corhttps://docs.rs/rand_core/latest/rand_core/trait.Rng.htmlcore/seedable_rng/trait.SeedableRng.html
[`SysRng`]: https://docs.rs/rand/latest/rand/rngs/struct.SysRng.html
[`rand::rng`]: https://docs.rs/rand/latest/rand/fn.rng.html
[`rand::Rng`]: https://docs.rs/rand/latest/rand/trait.Rng.html