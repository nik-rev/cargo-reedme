# Rustls - a modern TLS library

Rustls is a TLS library that aims to provide a good level of cryptographic security,
requires no configuration to achieve that security, and provides no unsafe features or
obsolete cryptography by default.

Rustls implements TLS1.2 and TLS1.3 for both clients and servers. See [the full
list of protocol features](https://docs.rs/rustls/0.23.36/rustls/manual/_04_features/).

### Platform support

While Rustls itself is platform independent, by default it uses [`aws-lc-rs`] for implementing
the cryptography in TLS.  See [the aws-lc-rs FAQ][aws-lc-rs-platforms-faq] for more details of the
platform/architecture support constraints in aws-lc-rs.

[`ring`] is also available via the `ring` crate feature: see
[the supported `ring` target platforms][ring-target-platforms].

By providing a custom instance of the [`crypto::CryptoProvider`] struct, you
can replace all cryptography dependencies of rustls.  This is a route to being portable
to a wider set of architectures and environments, or compliance requirements.  See the
[`crypto::CryptoProvider`] documentation for more details.

Specifying `default-features = false` when depending on rustls will remove the implicit
dependency on aws-lc-rs.

Rustls requires Rust 1.71 or later. It has an optional dependency on zlib-rs which requires 1.75 or later.

[ring-target-platforms]: https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64
[`crypto::CryptoProvider`]: https://docs.rs/rustls/0.23.36/rustls/cryphttps://docs.rs/rustls/0.23.36/rustls/cryphttps://docs.rs/rustls/0.23.36/rustls/cryphttps://docs.rs/rustls/0.23.36/rustls/crypto/struct.CryptoProvider.html
[`ring`]: https://crates.io/crates/ring
[aws-lc-rs-platforms-faq]: https://aws.github.io/aws-lc-rs/faq.html#can-i-run-aws-lc-rs-on-x-platform-or-architecture
[`aws-lc-rs`]: https://crates.io/crates/aws-lc-rs

### Cryptography providers

Since Rustls 0.22 it has been possible to choose the provider of the cryptographic primitives
that Rustls uses. This may be appealing if you have specific platform, compliance or feature
requirements that aren’t met by the default provider, [`aws-lc-rs`].

Users that wish to customize the provider in use can do so when constructing `ClientConfig`
and `ServerConfig` instances using the `with_crypto_provider` method on the respective config
builder types. See the [`crypto::CryptoProvider`] documentation for more details.

#### Built-in providers

Rustls ships with two built-in providers controlled by associated crate features:

  * [`aws-lc-rs`] - enabled by default, available with the `aws_lc_rs` crate feature enabled.
  * [`ring`] - available with the `ring` crate feature enabled.

See the documentation for [`crypto::CryptoProvider`] for details on how providers are
selected.

#### Third-party providers

The community has also started developing third-party providers for Rustls:

  * [`boring-rustls-provider`] - a work-in-progress provider that uses [`boringssl`] for
    cryptography.
  * [`rustls-graviola`] - a provider that uses [`graviola`] for cryptography.
  * [`rustls-mbedtls-provider`] - a provider that uses [`mbedtls`] for cryptography.
  * [`rustls-openssl`] - a provider that uses [OpenSSL] for cryptography.
  * [`rustls-rustcrypto`] - an experimental provider that uses the crypto primitives
    from [`RustCrypto`] for cryptography.
  * [`rustls-symcrypt`] - a provider that uses Microsoft’s [SymCrypt] library.
  * [`rustls-wolfcrypt-provider`] - a work-in-progress provider that uses [`wolfCrypt`] for cryptography.

[`rustls-graviola`]: https://crates.io/crates/rustls-graviola
[`graviola`]: https://github.com/ctz/graviola
[`rustls-mbedtls-provider`]: https://github.com/fortanix/rustls-mbedtls-provider
[`mbedtls`]: https://github.com/Mbed-TLS/mbedtls
[`rustls-openssl`]: https://github.com/tofay/rustls-openssl
[OpenSSL]: https://openssl-library.org/
[`rustls-symcrypt`]: https://github.com/microsoft/rustls-symcrypt
[SymCrypt]: https://github.com/microsoft/SymCrypt
[`boring-rustls-provider`]: https://github.com/janrueth/boring-rustls-provider
[`boringssl`]: https://github.com/google/boringssl
[`rustls-rustcrypto`]: https://github.com/RustCrypto/rustls-rustcrypto
[`RustCrypto`]: https://github.com/RustCrypto
[`rustls-wolfcrypt-provider`]: https://github.com/wolfSSL/rustls-wolfcrypt-provider
[`wolfCrypt`]: https://www.wolfssl.com/products/wolfcrypt

#### Custom provider

We also provide a simple example of writing your own provider in the [custom provider example].
This example implements a minimal provider using parts of the [`RustCrypto`] ecosystem.

See the [Making a custom CryptoProvider] section of the documentation for more information
on this topic.

[custom provider example]: https://github.com/rustls/rustls/tree/main/provider-example/
[`RustCrypto`]: https://github.com/RustCrypto
[Making a custom CryptoProvider]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#making-a-custom-cryptoprovider

## Design overview

Rustls is a low-level library. If your goal is to make HTTPS connections you may prefer
to use a library built on top of Rustls like [hyper] or [ureq].

[hyper]: https://crates.io/crates/hyper
[ureq]: https://crates.io/crates/ureq

### Rustls does not take care of network IO
It doesn’t make or accept TCP connections, or do DNS, or read or write files.

Our [examples] directory contains demos that show how to handle I/O using the
[`stream::Stream`](https://docs.rs/rustls/0.23.36/rustls/stream/struct.Stream.html) helper, as well as more complex asynchronous I/O using [`mio`].
If you’re already using Tokio for an async runtime you may prefer to use [`tokio-rustls`] instead
of interacting with rustls directly.

[examples]: https://github.com/rustls/rustls/tree/main/examples
[`tokio-rustls`]: https://github.com/rustls/tokio-rustls

### Rustls provides encrypted pipes
These are the [`ServerConnection`](https://docs.rs/rustls/0.23.36/rustls/server/server_conn/connection/struct.ServerConnection.html) and [`ClientConnection`](https://docs.rs/rustls/0.23.36/rustls/client/client_conn/connection/struct.ClientConnection.html) types.  You supply raw TLS traffic
on the left (via the [`read_tls()`] and [`write_tls()`] methods) and then read/write the
plaintext on the right:

[`read_tls()`]: Connection::read_tls
[`write_tls()`]: Connection::read_tls

```text
         TLS                                   Plaintext
         ===                                   =========
    read_tls()      +-----------------------+      reader() as io::Read
                    |                       |
          +--------->   ClientConnection    +--------->
                    |          or           |
          <---------+   ServerConnection    <---------+
                    |                       |
    write_tls()     +-----------------------+      writer() as io::Write
```

### Rustls takes care of server certificate verification
You do not need to provide anything other than a set of root certificates to trust.
Certificate verification cannot be turned off or disabled in the main API.

## Getting started
This is the minimum you need to do to make a TLS client connection.

First we load some root certificates.  These are used to authenticate the server.
The simplest way is to depend on the [`webpki_roots`] crate which contains
the Mozilla set of root certificates.

```rust
let root_store = rustls::RootCertStore::from_iter(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned(),
);
```

[`webpki_roots`]: https://crates.io/crates/webpki-roots

Next, we make a `ClientConfig`.  You’re likely to make one of these per process,
and use it for all connections made by that process.

```rust
let config = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();
```

Now we can make a connection.  You need to provide the server’s hostname so we
know what to expect to find in the server’s certificate.

```rust
let rc_config = Arc::new(config);
let example_com = "example.com".try_into().unwrap();
let mut client = rustls::ClientConnection::new(rc_config, example_com);
```

Now you should do appropriate IO for the `client` object.  If `client.wants_read()` yields
true, you should call `client.read_tls()` when the underlying connection has data.
Likewise, if `client.wants_write()` yields true, you should call `client.write_tls()`
when the underlying connection is able to send data.  You should continue doing this
as long as the connection is valid.

The return types of `read_tls()` and `write_tls()` only tell you if the IO worked.  No
parsing or processing of the TLS messages is done.  After each `read_tls()` you should
therefore call `client.process_new_packets()` which parses and processes the messages.
Any error returned from `process_new_packets` is fatal to the connection, and will tell you
why.  For example, if the server’s certificate is expired `process_new_packets` will
return `Err(InvalidCertificate(Expired))`.  From this point on,
`process_new_packets` will not do any new work and will return that error continually.

You can extract newly received data by calling `client.reader()` (which implements the
`io::Read` trait).  You can send data to the peer by calling `client.writer()` (which
implements `io::Write` trait).  Note that `client.writer().write()` buffers data you
send if the TLS connection is not yet established: this is useful for writing (say) a
HTTP request, but this is buffered so avoid large amounts of data.

The following code uses a fictional socket IO API for illustration, and does not handle
errors.

```rust
use std::io;
use rustls::Connection;

client.writer().write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut socket = connect("example.com", 443);
loop {
  if client.wants_read() && socket.ready_for_read() {
    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();

    let mut plaintext = Vec::new();
    client.reader().read_to_end(&mut plaintext).unwrap();
    io::stdout().write(&plaintext).unwrap();
  }

  if client.wants_write() && socket.ready_for_write() {
    client.write_tls(&mut socket).unwrap();
  }

  socket.wait_for_something_to_happen();
}
```

# Examples

You can find several client and server examples of varying complexity in the [examples]
directory, including [`tlsserver-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsserver-mio.rs)
and [`tlsclient-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs)
\- full worked examples using [`mio`].

[`mio`]: https://docs.rs/mio/latest/mio/

# Manual

The [rustls manual](https://docs.rs/rustls/0.23.36/rustls/manual/) explains design decisions and includes how-to guidance.

# Crate features
Here’s a list of what features are exposed by the rustls crate and what
they mean.

- `std` (enabled by default): enable the high-level (buffered) Connection API and other functionality
  which relies on the `std` library.

- `aws_lc_rs` (enabled by default): makes the rustls crate depend on the [`aws-lc-rs`] crate.
  Use `rustls::crypto::aws_lc_rs::default_provider().install_default()` to
  use it as the default `CryptoProvider`, or provide it explicitly
  when making a `ClientConfig` or `ServerConfig`.

  Note that aws-lc-rs has additional build-time dependencies like cmake.
  See [the documentation](https://aws.github.io/aws-lc-rs/requirements/index.html) for details.

- `ring`: makes the rustls crate depend on the *ring* crate for cryptography.
  Use `rustls::crypto::ring::default_provider().install_default()` to
  use it as the default `CryptoProvider`, or provide it explicitly
  when making a `ClientConfig` or `ServerConfig`.

- `fips`: enable support for FIPS140-3-approved cryptography, via the [`aws-lc-rs`] crate.
  This feature enables the `aws_lc_rs` crate feature, which makes the rustls crate depend
  on [aws-lc-rs](https://github.com/aws/aws-lc-rs).  It also changes the default
  for [`ServerConfig::require_ems`](https://docs.rs/rustls/0.23.36/rustls/server/server_conn/struct.ServerConfig.html#structfield.require_ems) and [`ClientConfig::require_ems`](https://docs.rs/rustls/0.23.36/rustls/client/client_conn/struct.ClientConfig.html#structfield.require_ems).

  See [manual::_06_fips](https://docs.rs/rustls/0.23.36/rustls/manual/_06_fips/) for more details.

- `prefer-post-quantum` (enabled by default): for the [`aws-lc-rs`]-backed provider,
  prioritizes post-quantum secure key exchange by default (using X25519MLKEM768).
  This feature merely alters the order of `rustls::crypto::aws_lc_rs::DEFAULT_KX_GROUPS`.
  See [the manual][x25519mlkem768-manual] for more details.

- `custom-provider`: disables implicit use of built-in providers (`aws-lc-rs` or `ring`). This forces
  applications to manually install one, for instance, when using a custom `CryptoProvider`.

- `tls12` (enabled by default): enable support for TLS version 1.2. Note that, due to the
  additive nature of Cargo features and because it is enabled by default, other crates
  in your dependency graph could re-enable it for your application. If you want to disable
  TLS 1.2 for security reasons, consider explicitly enabling TLS 1.3 only in the config
  builder API.

- `logging` (enabled by default): make the rustls crate depend on the `log` crate.
  rustls outputs interesting protocol-level messages at `trace!` and `debug!` level,
  and protocol-level errors at `warn!` and `error!` level.  The log messages do not
  contain secret key data, and so are safe to archive without affecting session security.

- `read_buf`: when building with Rust Nightly, adds support for the unstable
  `std::io::ReadBuf` and related APIs. This reduces costs from initializing
  buffers. Will do nothing on non-Nightly releases.

- `brotli`: uses the `brotli` crate for RFC8879 certificate compression support.

- `zlib`: uses the `zlib-rs` crate for RFC8879 certificate compression support.

[x25519mlkem768-manual]: https://docs.rs/rustls/0.23.36/rustls/manual/_05_defaults/