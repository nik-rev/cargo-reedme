# RustCrypto: ChaCha20

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]
[![HAZMAT][hazmat-image]][hazmat-link]

Pure Rust implementation of the [ChaCha20 Stream Cipher][1].

<img src="https://raw.githubusercontent.com/RustCrypto/meta/master/img/stream-ciphers/chacha20.png" width="300px">

## About

[ChaCha20][1] is a [stream cipher][2] which is designed to support
high-performance software implementations.

It improves upon the previous [Salsa20][3] stream cipher with increased
per-round diffusion at no cost to performance.

This crate also contains an implementation of [XChaCha20][4]: a variant
of ChaCha20 with an extended 192-bit (24-byte) nonce, gated under the
`chacha20` Cargo feature (on-by-default).

## Implementations

This crate contains the following implementations of ChaCha20, all of which
work on stable Rust with the following `RUSTFLAGS`:

- `x86` / `x86_64`
  - `avx2`: (~1.4cpb) `-Ctarget-cpu=haswell -Ctarget-feature=+avx2`
  - `sse2`: (~1.6cpb) `-Ctarget-feature=+sse2` (on by default on x86 CPUs)
  - `avx512`: `-Ctarget-feature=+avx512f,+avx512vl --cfg chacha20_avx512` requires Rust 1.89+
- `aarch64`
  - `neon` (~2-3x faster than `soft`) requires the `neon` feature enabled
- Portable
  - `soft`: (~5 cpb on x86/x86_64)

NOTE: cpb = cycles per byte (smaller is better)

## Security

### ⚠️ Warning: [Hazmat!][hazmat-link]

This crate does not ensure ciphertexts are authentic (i.e. by using a MAC to
verify ciphertext integrity), which can lead to serious vulnerabilities
if used incorrectly!

To avoid this, use an [AEAD][5] mode based on ChaCha20, i.e. [ChaCha20Poly1305][6].
See the [RustCrypto/AEADs][7] repository for more information.

USE AT YOUR OWN RISK!

### Notes

This crate has received one [security audit by NCC Group][8], with no significant
findings. We would like to thank [MobileCoin][9] for funding the audit.

All implementations contained in the crate (along with the underlying ChaCha20
stream cipher itself) are designed to execute in constant time.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/chacha20.svg
[crate-link]: https://crates.io/crates/chacha20
[docs-image]: https://docs.rs/chacha20/badge.svg
[docs-link]: https://docs.rs/chacha20/
[build-image]: https://github.com/RustCrypto/stream-ciphers/actions/workflows/chacha20.yml/badge.svg
[build-link]: https://github.com/RustCrypto/stream-ciphers/actions/workflows/chacha20.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/260049-stream-ciphers
[hazmat-image]: https://img.shields.io/badge/crypto-hazmat%E2%9A%A0-red.svg
[hazmat-link]: https://github.com/RustCrypto/meta/blob/master/HAZMAT.md

[//]: # (footnotes)

[1]: https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant
[2]: https://en.wikipedia.org/wiki/Stream_cipher
[3]: https://en.wikipedia.org/wiki/Salsa20
[4]: https://tools.ietf.org/html/draft-arciszewski-xchacha-02
[5]: https://en.wikipedia.org/wiki/Authenticated_encryption
[6]: https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305
[7]: https://github.com/RustCrypto/AEADs
[8]: https://web.archive.org/web/20240108154854/https://research.nccgroup.com/wp-content/uploads/2020/02/NCC_Group_MobileCoin_RustCrypto_AESGCM_ChaCha20Poly1305_Implementation_Review_2020-02-12_v1.0.pdf
[9]: https://www.mobilecoin.com/
# Usage

Cipher functionality is accessed using traits from re-exported [`cipher`](https://docs.rs/cipher/latest/cipher/) crate, or as a set
of random number generator types ending in `*Rng` which implement traits from the [`rand_core`](https://docs.rs/rand_core/latest/rand_core/)
crate.

This crate contains the following variants of the ChaCha20 core algorithm:

- [`ChaCha20`](https://docs.rs/chacha20/0.10.0/chacha20/chacha/type.ChaCha20.html): standard IETF variant with 96-bit nonce
- [`ChaCha8`](https://docs.rs/chacha20/0.10.0/chacha20/chacha/type.ChaCha8.html) / [`ChaCha12`](https://docs.rs/chacha20/0.10.0/chacha20/chacha/type.ChaCha12.html): reduced round variants of ChaCha20
- [`XChaCha20`](https://docs.rs/chacha20/0.10.0/chacha20/xchacha/type.XChaCha20.html): 192-bit extended nonce variant
- [`XChaCha8`](https://docs.rs/chacha20/0.10.0/chacha20/xchacha/type.XChaCha8.html) / [`XChaCha12`](https://docs.rs/chacha20/0.10.0/chacha20/xchacha/type.XChaCha12.html): reduced round variants of XChaCha20
- [`ChaCha20Legacy`](https://docs.rs/chacha20/0.10.0/chacha20/legacy/type.ChaCha20Legacy.html): “djb” variant with 64-bit nonce.
**WARNING:** This implementation internally uses 32-bit counter,
while the original implementation uses 64-bit counter. In other words,
it does not allow encryption of more than 256 GiB of data.

## Example
 ```rust
use chacha20::ChaCha20;
// Import relevant traits
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use hex_literal::hex;

let key = [0x42; 32];
let nonce = [0x24; 12];
let plaintext = hex!("00010203 04050607 08090A0B 0C0D0E0F");
let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");

// Key and IV must be references to the `Array` type.
// Here we use the `Into` trait to convert arrays into it.
let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

let mut buffer = plaintext.clone();

// apply keystream (encrypt)
cipher.apply_keystream(&mut buffer);
assert_eq!(buffer, ciphertext);

let ciphertext = buffer.clone();

// ChaCha ciphers support seeking
cipher.seek(0u32);

// decrypt ciphertext by applying keystream again
cipher.apply_keystream(&mut buffer);
assert_eq!(buffer, plaintext);

// stream ciphers can be used with streaming messages
cipher.seek(0u32);
for chunk in buffer.chunks_mut(3) {
    cipher.apply_keystream(chunk);
}
assert_eq!(buffer, ciphertext);
```

# Configuration Flags

You can modify crate using the following configuration flags:

- `chacha20_backend="avx2"`: force AVX2 backend on x86/x86_64 targets.
  Requires enabled AVX2 target feature. Ignored on non-x86(_64) targets.
- `chacha20_backend=“avx512”: force AVX-512 backend on x86/x86_64 targets.
  Requires enabled AVX-512 target feature (MSRV 1.89). Ignored on non-x86(_64) targets.
- `chacha20_backend="soft"`: force software backend.
- `chacha20_backend="sse2"`: force SSE2 backend on x86/x86_64 targets.
  Requires enabled SSE2 target feature. Ignored on non-x86(-64) targets.

The flags can be enabled using `RUSTFLAGS` environmental variable
(e.g. `RUSTFLAGS='--cfg chacha20_backend="avx2"'`) or by modifying `.cargo/config.toml`:

```toml
# In .cargo/config.toml
[build]
rustflags = ['--cfg', 'chacha20_backend="avx2"']
```

## AVX-512 support

To use the MSRV 1.89 AVX-512 support, you must enable it using: `--cfg chacha20_avx512`.

[ChaCha]: https://tools.ietf.org/html/rfc8439
[Salsa]: https://en.wikipedia.org/wiki/Salsa20
[`chacha20poly1305`]: https://docs.rs/chacha20poly1305