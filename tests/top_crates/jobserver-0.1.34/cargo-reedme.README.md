An implementation of the GNU make jobserver.

This crate is an implementation, in Rust, of the GNU `make` jobserver for
CLI tools that are interoperating with make or otherwise require some form
of parallelism limiting across process boundaries. This was originally
written for usage in Cargo to both (a) work when `cargo` is invoked from
`make` (using `make`’s jobserver) and (b) work when `cargo` invokes build
scripts, exporting a jobserver implementation for `make` processes to
transitively use.

The jobserver implementation can be found in [detail online][docs] but
basically boils down to a cross-process semaphore. On Unix this is
implemented with the `pipe` syscall and read/write ends of a pipe and on
Windows this is implemented literally with IPC semaphores. Starting from
GNU `make` version 4.4, named pipe becomes the default way in communication
on Unix. This crate also supports that feature in the sense of inheriting
and forwarding the correct environment.

The jobserver protocol in `make` also dictates when tokens are acquired to
run child work, and clients using this crate should take care to implement
such details to ensure correct interoperation with `make` itself.

## Examples

Connect to a jobserver that was set up by `make` or a different process:

```rust
use jobserver::Client;

// See API documentation for why this is `unsafe`
let client = match unsafe { Client::from_env() } {
    Some(client) => client,
    None => panic!("client not configured"),
};
```

Acquire and release token from a jobserver:

```rust
use jobserver::Client;

let client = unsafe { Client::from_env().unwrap() };
let token = client.acquire().unwrap(); // blocks until it is available
drop(token); // releases the token when the work is done
```

Create a new jobserver and configure a child process to have access:

```rust
use std::process::Command;
use jobserver::Client;

let client = Client::new(4).expect("failed to create jobserver");
let mut cmd = Command::new("make");
client.configure(&mut cmd);
```

## Caveats

This crate makes no attempt to release tokens back to a jobserver on
abnormal exit of a process. If a process which acquires a token is killed
with ctrl-c or some similar signal then tokens will not be released and the
jobserver may be in a corrupt state.

Note that this is typically ok as ctrl-c means that an entire build process
is being torn down, but it’s worth being aware of at least!

## Windows caveats

There appear to be two implementations of `make` on Windows. On MSYS2 one
typically comes as `mingw32-make` and the other as `make` itself. I’m not
personally too familiar with what’s going on here, but for jobserver-related
information the `mingw32-make` implementation uses Windows semaphores
whereas the `make` program does not. The `make` program appears to use file
descriptors and I’m not really sure how it works, so this crate is not
compatible with `make` on Windows. It is, however, compatible with
`mingw32-make`.

[docs]: https://make.mad-scientist.net/papers/jobserver-implementation/