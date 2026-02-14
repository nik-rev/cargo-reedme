# `object`

The `object` crate provides a unified interface to working with object files
across platforms. It supports reading relocatable object files and executable files,
and writing relocatable object files and some executable files.

## Raw struct definitions

Raw structs are defined for: [ELF](https://docs.rs/object/0.37.3/object/elf/), [Mach-O](https://docs.rs/object/0.37.3/object/macho/), [PE/COFF](https://docs.rs/object/0.37.3/object/pe/),
[XCOFF](https://docs.rs/object/0.37.3/object/xcoff/), [archive](https://docs.rs/object/0.37.3/object/archive/).
Types and traits for zerocopy support are defined in the [`pod`](https://docs.rs/object/0.37.3/object/pod/) and [`endian`](https://docs.rs/object/0.37.3/object/endian/) modules.

## Unified read API

The [`read`](https://docs.rs/object/0.37.3/object/read/) module provides a unified read API using the [`read::Object`](https://docs.rs/object/0.37.3/object/read/traits/trait.Object.html) trait.
There is an implementation of this trait for [`read::File`](https://docs.rs/object/0.37.3/object/read/any/enum.File.html), which allows reading any
file format, as well as implementations for each file format.

## Low level read API

The [`read#modules`](https://docs.rs/object/0.37.3/object/read/##modules`) submodules define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

## Unified write API

The [`mod@write`] module provides a unified write API for relocatable object files
using [`write::Object`]. This does not support writing executable files.

## Low level write API

The [`mod@write#modules`] submodules define helpers for writing the raw structs.

## Build API

The [`mod@build`] submodules define helpers for building object files, either from
scratch or by modifying existing files.

## Shared definitions

The crate provides a number of definitions that are used by both the read and write
APIs. These are defined at the top level module, but none of these are the main entry
points of the crate.