The enum [`Either`] with variants `Left` and `Right` is a general purpose
sum type with two cases.

[`Either`]: enum.Either.html

**Crate features:**

* `"std"`
  Enabled by default. Disable to make the library `#![no_std]`.

* `"serde"`
  Disabled by default. Enable to `#[derive(Serialize, Deserialize)]` for `Either`
