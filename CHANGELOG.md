# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/nik-rev/cargo-reedme/compare/v0.4.0...HEAD

- Documentation improvements

## [v0.4.0] - 2026-02-27

[v0.4.0]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.6...v0.4.0

### Changed

- Increment all headings by 1 level when it makes sense to do so. Use `increment-headings = false` to disable
- Made detection of whether to generate README from `lib.rs` or `main.rs` comments smarter

### Added

- Added an option `target` that allows specifying whether doc comments should be used from `lib.rs` or `main.rs`
- Config: added `features` field, which are `--features` passed to Cargo
- Config: added `all-features` field, which is `--all-features` passed to Cargo
- Config: added `no-default-features` field, which is `--no-default-features` passed to Cargo
- Config: added `rustc-args` field, which are the additional `RUSTFLAGS` to set
- Config: added `rustdoc-args` field, which are the additional `RUSTFLAGS` to set

### Fixed

- Inherit `note` field from the workspace
- Fixed options not being inherited from the workspace config

## [v0.3.6] - 2026-02-24

[v0.3.6]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.5...v0.3.6

- Documentation improvements

## [v0.3.5] - 2026-02-23

[v0.3.5]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.4...v0.3.5

- Documentation improvements

## [v0.3.4] - 2026-02-23

[v0.3.4]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.3...v0.3.4

- Documentation improvements

## [v0.3.3] - 2026-02-23

[v0.3.3]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.2...v0.3.3

- Fix exiting with a code of `1` (failure) when running in `--check` mode even if all is OK

## [v0.3.2] - 2026-02-23

[v0.3.2]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.1...v0.3.2

- Skip first 2 arguments when computing `{args}` interpolation in `note` field

## [v0.3.1] - 2026-02-23

[v0.3.1]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.0...v0.3.1

- Improve error message when Rustdoc JSON fails to deserialize

## [v0.3.0] - 2026-02-23

[v0.3.0]: https://github.com/nik-rev/cargo-reedme/compare/v0.2.0...v0.3.0

- Make the command actually work as a `cargo` subcommand

## [v0.2.0] - 2026-02-23

[v0.2.0]: https://github.com/nik-rev/cargo-reedme/compare/v0.1.0...v0.2.0

- Add config option `note` to configure the inserted note
