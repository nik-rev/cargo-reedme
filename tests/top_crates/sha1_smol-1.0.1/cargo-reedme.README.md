A minimal implementation of SHA1 for rust.

This implementation supports no_std which is the default mode.  The
following features are available and can be optionally enabled:

* ``serde``: when enabled the `Digest` type can be serialized.
* ``std``: when enabled errors from this library implement `std::error::Error`
  and the `hexdigest` shortcut becomes available.

## Example

```rust
let mut m = sha1_smol::Sha1::new();
m.update(b"Hello World!");
assert_eq!(m.digest().to_string(),
           "2ef7bde608ce5404e97d5f042f95f89f1c232871");
```

The sha1 object can be updated multiple times.  If you only need to use
it once you can also use shortcuts (requires std):

```rust
assert_eq!(sha1_smol::Sha1::from("Hello World!").hexdigest(),
           "2ef7bde608ce5404e97d5f042f95f89f1c232871");
```