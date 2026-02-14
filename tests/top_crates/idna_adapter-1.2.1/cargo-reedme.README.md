This crate abstracts over a Unicode back end for the [`idna`][1]
crate.

To work around the lack of [`global-features`][2] in Cargo, this
crate allows the top level `Cargo.lock` to choose an alternative
Unicode back end for the `idna` crate by pinning a version of this
crate.

See the [README of the latest version][3] for more details.

[1]: https://docs.rs/crate/idna/latest
[2]: https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618
[3]: https://docs.rs/crate/idna_adapter/latest