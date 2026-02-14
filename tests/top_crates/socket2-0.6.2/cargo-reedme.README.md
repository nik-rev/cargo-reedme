Utilities for creating and using sockets.

The goal of this crate is to create and use a socket using advanced
configuration options (those that are not available in the types in the
standard library) without using any unsafe code.

This crate provides as direct as possible access to the system’s
functionality for sockets, this means little effort to provide
cross-platform utilities. It is up to the user to know how to use sockets
when using this crate. *If you don’t know how to create a socket using
libc/system calls then this crate is not for you*. Most, if not all,
functions directly relate to the equivalent system call with no error
handling applied, so no handling errors such as [`EINTR`]. As a result using
this crate can be a little wordy, but it should give you maximal flexibility
over configuration of sockets.

[`EINTR`]: std::io::ErrorKind::Interrupted

# Examples

```rust
use std::net::{SocketAddr, TcpListener};
use socket2::{Socket, Domain, Type};

// Create a TCP listener bound to two addresses.
let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;

socket.set_only_v6(false)?;
let address: SocketAddr = "[::1]:12345".parse().unwrap();
socket.bind(&address.into())?;
socket.listen(128)?;

let listener: TcpListener = socket.into();
// ...
```

## Features

This crate has a single feature `all`, which enables all functions even ones
that are not available on all OSs.