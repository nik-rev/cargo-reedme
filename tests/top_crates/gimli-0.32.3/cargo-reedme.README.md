`gimli` is a library for reading and writing the
[DWARF debugging format](https://dwarfstd.org/).

See the [read](./read/index.html) and [write](./write/index.html) modules
for examples and API documentation.

## Cargo Features

Cargo features that can be enabled with `gimli`:

* `std`: Enabled by default. Use the `std` library. Disabling this feature
  allows using `gimli` in embedded environments that do not have access to
  `std`. Note that even when `std` is disabled, `gimli` still requires an
  implementation of the `alloc` crate.

* `read`: Enabled by default. Enables the `read` module. Use of `std` is
  optional.

* `write`: Enabled by default. Enables the `write` module. Always uses
  the `std` library.