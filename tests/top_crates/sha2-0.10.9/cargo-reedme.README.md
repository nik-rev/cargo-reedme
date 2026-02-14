An implementation of the [SHA-2][1] cryptographic hash algorithms.

There are 6 standard algorithms specified in the SHA-2 standard: [`Sha224`](https://docs.rs/sha2/0.10.9/sha2/type.Sha224.html),
[`Sha256`](https://docs.rs/sha2/0.10.9/sha2/type.Sha256.html), [`Sha512_224`](https://docs.rs/sha2/0.10.9/sha2/type.Sha512_224.html), [`Sha512_256`](https://docs.rs/sha2/0.10.9/sha2/type.Sha512_256.html), [`Sha384`](https://docs.rs/sha2/0.10.9/sha2/type.Sha384.html), and [`Sha512`](https://docs.rs/sha2/0.10.9/sha2/type.Sha512.html).

Algorithmically, there are only 2 core algorithms: SHA-256 and SHA-512.
All other algorithms are just applications of these with different initial
hash values, and truncated to different digest bit lengths. The first two
algorithms in the list are based on SHA-256, while the last four are based
on SHA-512.

# Usage

## One-shot API

```rust
use hex_literal::hex;
use sha2::{Sha256, Digest};

let result = Sha256::digest(b"hello world");
assert_eq!(result[..], hex!("
    b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
")[..]);
```

## Incremental API

```rust
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

// create a Sha256 object
let mut hasher = Sha256::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let result = hasher.finalize();

assert_eq!(result[..], hex!("
    b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
")[..]);

// same for Sha512
let mut hasher = Sha512::new();
hasher.update(b"hello world");
let result = hasher.finalize();

assert_eq!(result[..], hex!("
    309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
    989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
")[..]);
```

Also see [RustCrypto/hashes][2] readme.

[1]: https://en.wikipedia.org/wiki/SHA-2
[2]: https://github.com/RustCrypto/hashes