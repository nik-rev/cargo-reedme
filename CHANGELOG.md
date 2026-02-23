# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/nik-rev/cargo-reedme/compare/v0.3.3...HEAD

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
