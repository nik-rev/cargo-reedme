This tiny crate checks that the running or installed `rustc` meets some
version requirements. The version is queried by calling the Rust compiler
with `--version`. The path to the compiler is determined first via the
`RUSTC` environment variable. If it is not set, then `rustc` is used. If
that fails, no determination is made, and calls return `None`.

# Examples

**Note:** Please see [feature detection] for a note on enabling unstable
features based on detection via this crate.

[feature detection]: crate#feature-detection

* Set a `cfg` flag in `build.rs` if the running compiler was determined to
  be at least version `1.13.0`:

  ```rust
  extern crate version_check as rustc;

  if rustc::is_min_version("1.13.0").unwrap_or(false) {
      println!("cargo:rustc-cfg=question_mark_operator");
  }
```

  See [`is_max_version`](https://docs.rs/version_check/0.9.5/version_check/fn.is_max_version.html) or [`is_exact_version`](https://docs.rs/version_check/0.9.5/version_check/fn.is_exact_version.html) to check if the compiler
  is _at most_ or _exactly_ a certain version.
  <br /><br />

* Check that the running compiler was released on or after `2018-12-18`:

  ```rust
  extern crate version_check as rustc;

  match rustc::is_min_date("2018-12-18") {
      Some(true) => "Yep! It's recent!",
      Some(false) => "No, it's older.",
      None => "Couldn't determine the rustc version."
  };
```

  See [`is_max_date`](https://docs.rs/version_check/0.9.5/version_check/fn.is_max_date.html) or [`is_exact_date`](https://docs.rs/version_check/0.9.5/version_check/fn.is_exact_date.html) to check if the compiler was
  released _prior to_ or _exactly on_ a certain date.
  <br /><br />

* Check that the running compiler supports feature flags:

  ```rust
  extern crate version_check as rustc;

  match rustc::is_feature_flaggable() {
      Some(true) => "Yes! It's a dev or nightly release!",
      Some(false) => "No, it's stable or beta.",
      None => "Couldn't determine the rustc version."
  };
```

  Please see the note on [feature detection].
  <br /><br />

* Check that the running compiler supports a specific feature:

  ```rust
  extern crate version_check as rustc;

  if let Some(true) = rustc::supports_feature("doc_cfg") {
     println!("cargo:rustc-cfg=has_doc_cfg");
  }
```

  Please see the note on [feature detection].
  <br /><br />

* Check that the running compiler is on the stable channel:

  ```rust
  extern crate version_check as rustc;

  match rustc::Channel::read() {
      Some(c) if c.is_stable() => format!("Yes! It's stable."),
      Some(c) => format!("No, the channel {} is not stable.", c),
      None => format!("Couldn't determine the rustc version.")
  };
```

To interact with the version, release date, and release channel as structs,
use [`Version`](https://docs.rs/version_check/0.9.5/version_check/version/struct.Version.html), [`Date`](https://docs.rs/version_check/0.9.5/version_check/date/struct.Date.html), and [`Channel`](https://docs.rs/version_check/0.9.5/version_check/channel/struct.Channel.html), respectively. The [`triple()`](https://docs.rs/version_check/0.9.5/version_check/fn.triple.html)
function returns all three values efficiently.

# Feature Detection

While this crate can be used to determine if the current compiler supports
an unstable feature, no crate can determine whether that feature will work
in a way that you expect ad infinitum. If the feature changes in an
incompatible way, then your crate, as well as all of its transitive
dependents, will fail to build. As a result, great care should be taken when
enabling nightly features even when they're supported by the compiler.

One common mitigation used in practice is to make using unstable features
transitively opt-in via a crate feature or `cfg` so that broken builds only
affect those that explicitly asked for the feature. Another complementary
approach is to probe `rustc` at build-time by asking it to compile a small
but exemplary program that determines whether the feature works as expected,
enabling the feature only if the probe succeeds. Finally, eschewing these
recommendations, you should track the `nightly` channel closely to minimize
the total impact of a nightly breakages.

# Alternatives

This crate is dead simple with no dependencies. If you need something more
and don't care about panicking if the version cannot be obtained, or if you
don't mind adding dependencies, see
[rustc_version](https://crates.io/crates/rustc_version).