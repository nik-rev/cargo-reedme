Mio is a fast, low-level I/O library for Rust focusing on non-blocking APIs
and event notification for building high performance I/O apps with as little
overhead as possible over the OS abstractions.

# Usage

Using Mio starts by creating a [`Poll`], which reads events from the OS and
puts them into [`Events`]. You can handle I/O events from the OS with it.

For more detail, see [`Poll`].

[`Poll`]: ../mio/struct.Poll.html
[`Events`]: ../mio/event/struct.Events.html

## Examples

Examples can be found in the `examples` directory of the source code, or [on
GitHub].

[on GitHub]: https://github.com/tokio-rs/mio/tree/master/examples

## Guide

A getting started guide is available in the [`guide`](https://docs.rs/mio/1.1.1/mio/guide/) module.

## Available features

The available features are described in the [`features`](https://docs.rs/mio/1.1.1/mio/features/) module.