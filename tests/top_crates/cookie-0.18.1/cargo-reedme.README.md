HTTP cookie parsing and cookie jar management.

This crates provides the [`Cookie`](https://docs.rs/cookie/0.18.1/cookie/struct.Cookie.html) type, representing an HTTP cookie, and
the [`CookieJar`](https://docs.rs/cookie/0.18.1/cookie/jar/struct.CookieJar.html) type, which manages a collection of cookies for session
management, recording changes as they are made, and optional automatic
cookie encryption and signing.

# Usage

Add the following to the `[dependencies]` section of your `Cargo.toml`:

```toml
cookie = "0.18"
```

# Features

This crate exposes several features, all of which are disabled by default:

* **`percent-encode`**

  Enables _percent encoding and decoding_ of names and values in cookies.

  When this feature is enabled, the [`Cookie::encoded()`](https://docs.rs/cookie/0.18.1/cookie/struct.Cookie.html#method.encoded) and
  [`Cookie::parse_encoded()`](https://docs.rs/cookie/0.18.1/cookie/struct.Cookie.html#method.parse_encoded) methods are available. The `encoded` method
  returns a wrapper around a `Cookie` whose `Display` implementation
  percent-encodes the name and value of the cookie. The `parse_encoded`
  method percent-decodes the name and value of a `Cookie` during parsing.

* **`signed`**

  Enables _signed_ cookies via [`CookieJar::signed()`].

  When this feature is enabled, the [`CookieJar::signed()`] method,
  [`SignedJar`] type, and [`Key`] type are available. The jar acts as “child
  jar”; operations on the jar automatically sign and verify cookies as they
  are added and retrieved from the parent jar.

* **`private`**

  Enables _private_ (authenticated, encrypted) cookies via
  [`CookieJar::private()`].

  When this feature is enabled, the [`CookieJar::private()`] method,
  [`PrivateJar`] type, and [`Key`] type are available. The jar acts as “child
  jar”; operations on the jar automatically encrypt and decrypt/authenticate
  cookies as they are added and retrieved from the parent jar.

* **`key-expansion`**

  Enables _key expansion_ or _key derivation_ via [`Key::derive_from()`].

  When this feature is enabled, and either `signed` or `private` are _also_
  enabled, the [`Key::derive_from()`] method is available. The method can be
  used to derive a `Key` structure appropriate for use with signed and
  private jars from cryptographically valid key material that is shorter in
  length than the full key.

* **`secure`**

  A meta-feature that simultaneously enables `signed`, `private`, and
  `key-expansion`.

You can enable features via `Cargo.toml`:

```toml
[dependencies.cookie]
features = ["secure", "percent-encode"]
```