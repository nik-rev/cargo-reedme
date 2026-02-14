`addr2line` provides a cross-platform library for retrieving per-address debug information
from files with DWARF debug information. Given an address, it can return the file name,
line number, and function name associated with that address, as well as the inline call
stack leading to that address.

At the lowest level, the library uses a [`Context`](https://docs.rs/addr2line/0.25.1/addr2line/struct.Context.html) to cache parsed information so that
multiple lookups are efficient. To create a `Context`, you first need to open and parse the
file using an object file parser such as [`object`](https://github.com/gimli-rs/object),
create a [`gimli::Dwarf`](https://docs.rs/gimli/latest/gimli/read/dwarf/struct.Dwarf.html), and finally call [`Context::from_dwarf`](https://docs.rs/addr2line/0.25.1/addr2line/struct.Context.html#method.from_dwarf).

Location information is obtained with [`Context::find_location`](https://docs.rs/addr2line/0.25.1/addr2line/struct.Context.html#method.find_location) or
[`Context::find_location_range`](https://docs.rs/addr2line/0.25.1/addr2line/struct.Context.html#method.find_location_range). Function information is obtained with
[`Context::find_frames`](https://docs.rs/addr2line/0.25.1/addr2line/struct.Context.html#method.find_frames), which returns a frame for each inline function. Each frame
contains both name and location.

The library also provides a [`Loader`] which internally memory maps the files,
uses the `object` crate to do the parsing, and creates a `Context`.
The `Context` is not exposed, but the `Loader` provides the same functionality
via [`Loader::find_location`], [`Loader::find_location_range`], and
[`Loader::find_frames`]. The `Loader` also provides [`Loader::find_symbol`]
to use the symbol table instead of DWARF debugging information.
The `Loader` will load Mach-O dSYM files and split DWARF files as needed.

The crate has a CLI wrapper around the library which provides some of
the functionality of the `addr2line` command line tool distributed with
[GNU binutils](https://sourceware.org/binutils/docs/binutils/addr2line.html).