# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - 2026-02-01
The crate was moved from the rand repo ([rand#1658]) to the rngs repo ([#92]).

### Changed
- Bump MSRV to 1.85 and edition to 2024 ([rand#1671])
- Update to `rand_core` v0.10 ([rand#1712], [#96])
- Remove feature `os_rng` ([rand#1674])

[rand#1658]: https://github.com/rust-random/rand/pull/1658
[rand#1671]: https://github.com/rust-random/rand/pull/1671
[rand#1674]: https://github.com/rust-random/rand/pull/1674
[rand#1712]: https://github.com/rust-random/rand/pull/1712
[#92]: https://github.com/rust-random/rngs/pull/92
[#96]: https://github.com/rust-random/rngs/pull/96

## [0.9.0] - 2025-01-27
### Dependencies and features
- Update to `rand_core` v0.9.0 ([rand#1558])
- Feature `std` now implies feature `rand_core/std` ([rand#1153])
- Rename feature `serde1` to `serde` ([rand#1477])
- Rename feature `getrandom` to `os_rng` ([rand#1537])

### Other changes
- Remove usage of `unsafe` in `fn generate` ([rand#1181]) then optimise for AVX2 (~4-7%) ([rand#1192])
- Revise crate docs ([rand#1454])

[rand#1558]: https://github.com/rust-random/rand/pull/1558
[rand#1537]: https://github.com/rust-random/rand/pull/1537
[rand#1477]: https://github.com/rust-random/rand/pull/1477
[rand#1454]: https://github.com/rust-random/rand/pull/1454
[rand#1192]: https://github.com/rust-random/rand/pull/1192
[rand#1181]: https://github.com/rust-random/rand/pull/1181
[rand#1153]: https://github.com/rust-random/rand/pull/1153

## [0.3.1] - 2021-06-09
- add getters corresponding to existing setters: `get_seed`, `get_stream` ([rand#1124])
- add serde support, gated by the `serde1` feature ([rand#1124])
- ensure expected layout via `repr(transparent)` ([rand#1120])

[rand#1124]: https://github.com/rust-random/rand/pull/1124
[rand#1120]: https://github.com/rust-random/rand/pull/1120

## [0.3.0] - 2020-12-08
- Bump `rand_core` version to 0.6.0
- Bump MSRV to 1.36 ([rand#1011])
- Remove usage of deprecated feature "simd" of `ppv-lite86` ([rand#979]), then revert
  this change ([rand#1023]) since SIMD is only enabled by default from `ppv-lite86 v0.2.10`
- impl PartialEq+Eq for ChaChaXRng and ChaChaXCore ([rand#979])
- Fix panic on block counter wrap that was occurring in debug builds ([rand#980])

[rand#1020]: https://github.com/rust-random/rand/pull/1020
[rand#1011]: https://github.com/rust-random/rand/pull/1011
[rand#980]: https://github.com/rust-random/rand/pull/980
[rand#979]: https://github.com/rust-random/rand/pull/979

## [0.2.2] - 2020-03-09
- Integrate `c2-chacha`, reducing dependency count ([rand#931])
- Add CryptoRng to ChaChaXCore ([rand#944])

[rand#944]: https://github.com/rust-random/rand/pull/944
[rand#931]: https://github.com/rust-random/rand/pull/931

## [0.2.1] - 2019-07-22
- Force enable the `simd` feature of `c2-chacha` ([rand#845])

[rand#845]: https://github.com/rust-random/rand/pull/845

## [0.2.0] - 2019-06-06
- Rewrite based on the much faster `c2-chacha` crate ([rand#789])

[rand#789]: https://github.com/rust-random/rand/pull/789

## [0.1.1] - 2019-01-04
- Disable `i128` and `u128` if the `target_os` is `emscripten` ([rand#671]: work-around Emscripten limitation)
- Update readme and doc links

[rand#671]: https://github.com/rust-random/rand/pull/671

## [0.1.0] - 2018-10-17
- Pulled out of the Rand crate

[0.9.0]: https://github.com/rust-random/rand/compare/rand_chacha-0.3.1...0.9.0
[0.3.1]: https://github.com/rust-random/rand/compare/rand_chacha-0.3.0...rand_chacha-0.3.1
[0.3.0]: https://github.com/rust-random/rand/compare/rand_chacha-0.2.2...rand_chacha-0.3.0
[0.2.2]: https://github.com/rust-random/rand/compare/rand_chacha-0.2.1...rand_chacha-0.2.2
[0.2.1]: https://github.com/rust-random/rand/compare/rand_chacha-0.2.0...rand_chacha-0.2.1
[0.2.0]: https://github.com/rust-random/rand/compare/rand_chacha-0.1.1...rand_chacha-0.2.0
[0.1.1]: https://github.com/rust-random/rand/compare/rand_chacha-0.1.0...rand_chacha-0.1.1
[0.1.0]: https://github.com/rust-random/rand/compare/a55ba3feb49062ea8dec75c034d796f6e3f763ae...rand_chacha-0.1.0
