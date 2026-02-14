# cookie_store
Provides an implementation for storing and retrieving [`Cookie`](https://docs.rs/cookie_store/0.22.0/cookie_store/cookie/struct.Cookie.html)s per the path and domain matching
rules specified in [RFC6265](https://datatracker.ietf.org/doc/html/rfc6265).

## Example
Please refer to the [reqwest_cookie_store](https://crates.io/crates/reqwest_cookie_store) for
an example of using this library along with [reqwest](https://crates.io/crates/reqwest).

## Feature flags
* **`preserve_order`** —  uses `indexmap::IndexMap` in lieu of HashMap internally, so cookies are maintained in insertion/creation order
* **`public_suffix`** *(enabled by default)* —  Add support for public suffix lists, as provided by [publicsuffix](https://crates.io/crates/publicsuffix).
* **`wasm-bindgen`** —  Enables transitive feature `time/wasm-bindgen`; necessary in `wasm` contexts.
* **`log_secure_cookie_values`** —  Enable logging the values of cookies marked ‘secure’, off by default as values may be sensitive

 ### Serialization
* **`serde`** *(enabled by default)* —  Supports generic (format-agnostic) de/serialization for a `CookieStore`. Adds dependencies `serde` and `serde_derive`.
* **`serde_json`** *(enabled by default)* —  Supports de/serialization for a `CookieStore` via the JSON format. Enables feature `serde` and adds depenency `serde_json`.
* **`serde_ron`** —  Supports de/serialization for a `CookieStore` via the RON format. Enables feature `serde` and adds depenency `ron`.