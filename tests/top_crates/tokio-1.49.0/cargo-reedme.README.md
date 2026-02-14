A runtime for writing reliable network applications without compromising speed.

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous
applications with the Rust programming language. At a high level, it
provides a few major components:

* Tools for [working with asynchronous tasks][tasks], including
  [synchronization primitives and channels][sync] and [timeouts, sleeps, and
  intervals][time].
* APIs for [performing asynchronous I/O][io], including [TCP and UDP][net] sockets,
  [filesystem][fs] operations, and [process] and [signal] management.
* A [runtime] for executing asynchronous code, including a task scheduler,
  an I/O driver backed by the operating system’s event queue (`epoll`, `kqueue`,
  `IOCP`, etc…), and a high performance timer.

Guide level documentation is found on the [website].

[tasks]: #working-with-tasks
[sync]: https://docs.rs/tokio/1.49.0/tokio/sync/
[time]: https://docs.rs/tokio/1.49.0/tokio/time/
[io]: #asynchronous-io
[net]: https://docs.rs/tokio/1.49.0/tokio/net/
[fs]: https://docs.rs/tokio/1.49.0/tokio/fs/
[process]: https://docs.rs/tokio/1.49.0/tokio/process/
[signal]: https://docs.rs/tokio/1.49.0/tokio/signal/
[fs]: crate::fs
[runtime]: https://docs.rs/tokio/1.49.0/tokio/runtime/
[website]: https://tokio.rs/tokio/tutorial

# A Tour of Tokio

Tokio consists of a number of modules that provide a range of functionality
essential for implementing asynchronous applications in Rust. In this
section, we will take a brief tour of Tokio, summarizing the major APIs and
their uses.

The easiest way to get started is to enable all features. Do this by
enabling the `full` feature flag:

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring applications

Tokio is great for writing applications and most users in this case shouldn’t
worry too much about what features they should pick. If you’re unsure, we suggest
going with `full` to ensure that you don’t run into any road blocks while you’re
building your application.

#### Example

This example shows the quickest way to get started with Tokio.

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring libraries

As a library author your goal should be to provide the lightest weight crate
that is based on Tokio. To achieve this you should ensure that you only enable
the features you need. This allows users to pick up your crate without having
to enable unnecessary features.

#### Example

This example shows how you may want to import features for a library that just
needs to `tokio::spawn` and use a `TcpStream`.

```toml
tokio = { version = "1", features = ["rt", "net"] }
```

## Working With Tasks

Asynchronous programs in Rust are based around lightweight, non-blocking
units of execution called [_tasks_][tasks]. The [`tokio::task`] module provides
important tools for working with tasks:

* The [`spawn`] function and [`JoinHandle`] type, for scheduling a new task
  on the Tokio runtime and awaiting the output of a spawned task, respectively,
* Functions for [running blocking operations][blocking] in an asynchronous
  task context.

The [`tokio::task`] module is present only when the “rt” feature flag
is enabled.

[tasks]: task/index.html#what-are-tasks
[`tokio::task`]: https://docs.rs/tokio/1.49.0/tokio/task/
[`spawn`]: https://docs.rs/tokio/1.49.0/tokio/task/spawn/fn.spawn.html
[`JoinHahttps://docs.rs/tokio/1.49.0/tokio/task/ps://docs.rs/tokio/1.49.0/tokio/runtime/task/join/struct.JoinHandle.html
[blocking]: task/index.html#blocking-and-yielding

The [`tokio::sync`] module contains synchronization primitives to use when
needing to communicate or share data. These include:

* channels ([`oneshot`], [`mpsc`], [`watch`], and [`broadcast`]), for sending values
  between tasks,
* a non-blocking [`Mutex`], for controlling access to a shared, mutable
  value,
* an asynchronous [`Barrier`] type, for multiple tasks to synchronize before
  beginning a computation.

The `tokio::sync` module is present only when the “sync” feature flag is
enabled.

[`tokio::sync`]: https://docs.rs/tokio/1.49.0/tokio/sync/
[`Mutex`]: crate::sync::Mutex
[`Barrier`]: crate::sync::Barrier
[`oneshot`]: https://docs.rs/tokio/1.49.0/tokio/sync/oneshohttps://docs.rs/tokio/1.49.0/tokio/sync/mutex/struct.Mutex.html://docs.rs/tokhttps://docs.rs/tokio/1.49.0/tokio/sync/barrier/struct.Barrier.html/mpsc/
[`watch`]: https://docs.rs/tokio/1.49.0/tokio/sync/watch/
[`broadcast`]: https://docs.rs/tokio/1.49.0/tokio/sync/broadcast/

The [`tokio::time`] module provides utilities for tracking time and
scheduling work. This includes functions for setting [timeouts][timeout] for
tasks, [sleeping][sleep] work to run in the future, or [rhttps://docs.rs/tokio/1.49.0/tokio/sync/oneshot/n at an
interval][interval].

In order to use `tokio::time`, the “time” feature flag must be enabled.

[`tokio::time`]: https://docs.rs/tokio/1.49.0/tokio/time/
[sleep]: crate::time::sleep()
[interval]: crate::thttps://docs.rs/tokio/1.49.0/tokio/time/sleep/fn.sleep.htmleout]: https:https://docs.rs/tokio/1.49.0/tokio/time/interval/fn.interval.htmltokio/time/timeout/fn.timeout.html

Finally, Tokio provides a _runtime_ for executing asynchronous tasks. Most
applications can use the [`#[tokio::main]`][main] macro to run their code on the
Tokio runtime. However, this macro provides only basic configuration options. As
an alternative, the [`tokio::runtime`] module provides more powerful APIs for configuring
and managing runtimes. You should use that module if the `#[tokio::main]` macro doesn’t
provide the functionality you need.

Using the runtime requires the “rt” or “rt-multi-thread” feature flags, to
enable the current-thread [single-threaded scheduler][rt] and the [multi-thread
scheduler][rt-multi-thread], respectively. See the [`runtime` module
documentation][rt-features] for details. In addition, the “macros” feature
flag enables the `#[tokio::main]` and `#[tokio::test]` attributes.

[main]: attr.main.html
[`tokio::runtime`]: https://docs.rs/tokio/1.49.0/tokio/runtime/
[`Builder`]: crate::runtime::Builder
[`Runtime`]: crate::runtimehttps://docs.rs/tokio/1.49.0/tokio/runtime/builder/struct.Builder.html/index.html#current-thread-scheduler
[rt-multi-thread]: runtime/index.html#multi-thread-scheduler
[rt-features]: runtime/index.html#runtime-scheduler

## CPU-bound tasks and blocking code

Tokio is able to concurrently run many tasks on a few threads by repeatedly
swapping the currently running task on each thread. However, this kind of
swapping can only happen at `.await` points, so code that spends a long time
without reaching an `.await` will prevent other tasks from running. To
combat this, Tokio provides two kinds of threads: Core threads and blocking threads.

The core threads are where all asynchronous code runs, and Tokio will by default
spawn one for each CPU core. You can use the environment variable `TOKIO_WORKER_THREADS`
to override the default value.

The blocking threads are spawned on demand, can be used to run blocking code
that would otherwise block other tasks from running and are kept alive when
not used for a certain amount of time which can be configured with [`thread_keep_alive`].
Since it is not possible for Tokio to swap out blocking tasks, like it
can do with asynchronous code, the upper limit on the number of blocking
threads is very large. These limits can be configured on the [`Builder`].

To spawn a blocking task, you should use the [`spawn_blocking`] function.

[`Builder`]: crate::runtime::Builder
[`spawn_blocking`]: crate::task::spawn_blocking()
[`thread_keep_alive`]https://docs.rs/tokio/1.49.0/tokio/task/blocking/fn.spawn_blocking.html0/tokio/runtime/builder/struct.Builder.html#method.thread_keep_alive

```rust
#[tokio::main]
async fn main() {
    // This is running on a core thread.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}
```

If your code is CPU-bound and you wish to limit the number of threads used
to run it, you should use a separate thread pool dedicated to CPU bound tasks.
For example, you could consider using the [rayon] library for CPU-bound
tasks. It is also possible to create an extra Tokio runtime dedicated to
CPU-bound tasks, but if you do this, you should be careful that the extra
runtime runs _only_ CPU-bound tasks, as IO-bound tasks on that runtime
will behave poorly.

Hint: If using rayon, you can use a [`oneshot`] channel to send the result back
to Tokio when the rayon task finishes.

[rayon]: https://docs.rs/rayon
[`oneshot`]: crate::sync::oneshot

## Asynchronous IO

As well as scheduling and running tasks, Tokio provides everything you need
to perform input and output asynchronously.

The [`tokio::io`] module provides Tokio’s asynchronous core I/O primitives,
the [`AsyncRead`], [`AsyncWrite`], and [`AsyncBufRead`] traits. In addition,
when the “io-util” feature flag is enabled, it also provides combinators and
functions for working with these traits, forming as an asynchronous
counterpart to [`std::io`].

Tokio also includes APIs for performing various kinds of I/O and interacting
with the operating system asynchronously. These include:

* [`tokio::net`], which contains non-blocking versions of [TCP], [UDP], and
  [Unix Domain Sockets][UDS] (enabled by the “net” feature flag),
* [`tokio::fs`], similar to [`std::fs`] but for performing filesystem I/O
  asynchronously (enabled by the “fs” feature flag),
* [`tokio::signal`], for asynchronously handling Unix and Windows OS signals
  (enabled by the “signal” feature flag),
* [`tokio::process`], for spawning and managing child processes (enabled by
  the “process” feature flag).

[`tokio::io`]: https://docs.rs/tokio/1.49.0/tokio/io/
[`AsyncRead`]: https://docs.rs/tokio/1.49.0/tokio/io/async_read/trait.AsyncRead.html
[`AsyncWrite`]: https://docs.rs/tokio/1.49.0/tokio/io/async_write/trait.AsyncWrite.html
[`AsyncBufRead`]: https://docs.rs/tokio/1.49.0/tokio/io/async_buf_read/trait.AsyncBufRead.html
[`std::io`]: https://doc.rust-lang.org/stable/std/io/
[`tokio::net`]: https://docs.rs/tokio/1.49.0/tokio/net/
[TCP]: https://docs.rs/tokio/1.49.0/tokio/net/tcp/
[UDP]: https://docs.rs/tokio/1.49.0/tokio/net/udp/struct.UdpSocket.html
[UDS]: https://docs.rs/tokio/1.49.0/tokio/net/unix/
[`tokio::fs`]: https://docs.rs/tokio/1.49.0/tokio/fs/
[`std::fs`]: https://doc.rust-lang.org/stable/std/fs/
[`tokio::signal`]: https://docs.rs/tokio/1.49.0/tokio/signal/
[`tokio::process`]: https://docs.rs/tokio/1.49.0/tokio/process/

# Examples

A simple TCP echo server:

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

# Feature flags

Tokio uses a set of [feature flags] to reduce the amount of compiled code. It
is possible to just enable certain features over others. By default, Tokio
does not enable any features but allows one to enable a subset for their use
case. Below is a list of the available feature flags. You may also notice
above each function, struct and trait there is listed one or more feature flags
that are required for that item to be used. If you are new to Tokio it is
recommended that you use the `full` feature flag which will enable all public APIs.
Beware though that this will pull in many extra dependencies that you may not
need.

- `full`: Enables all features listed below except `test-util` and `tracing`.
- `rt`: Enables `tokio::spawn`, the current-thread scheduler,
  and non-scheduler utilities.
- `rt-multi-thread`: Enables the heavier, multi-threaded, work-stealing scheduler.
- `io-util`: Enables the IO based `Ext` traits.
- `io-std`: Enable `Stdout`, `Stdin` and `Stderr` types.
- `net`: Enables `tokio::net` types such as `TcpStream`, `UnixStream` and
  `UdpSocket`, as well as (on Unix-like systems) `AsyncFd` and (on
  FreeBSD) `PollAio`.
- `time`: Enables `tokio::time` types and allows the schedulers to enable
  the built in timer.
- `process`: Enables `tokio::process` types.
- `macros`: Enables `#[tokio::main]` and `#[tokio::test]` macros.
- `sync`: Enables all `tokio::sync` types.
- `signal`: Enables all `tokio::signal` types.
- `fs`: Enables `tokio::fs` types.
- `test-util`: Enables testing based infrastructure for the Tokio runtime.
- `parking_lot`: As a potential optimization, use the [`parking_lot`](https://docs.rs/parking_lot/latest/parking_lot/) crate’s
  synchronization primitives internally. Also, this
  dependency is necessary to construct some of our primitives
  in a `const` context. `MSRV` may increase according to the
  [`parking_lot`](https://docs.rs/parking_lot/latest/parking_lot/) release in use.

_Note: `AsyncRead` and `AsyncWrite` traits do not require any features and are
always available._

## Unstable features

Some feature flags are only available when specifying the `tokio_unstable` flag:

- `tracing`: Enables tracing events.

Likewise, some parts of the API are only available with the same flag:

- [`task::Builder`]
- Some methods on [`task::JoinSet`](https://docs.rs/tokio/1.49.0/tokio/task/join_set/struct.JoinSet.html)
- [`runtime::RuntimeMetrics`](https://docs.rs/tokio/1.49.0/tokio/runtime/metrics/runtime/struct.RuntimeMetrics.html)
- [`runtime::Builder::on_task_spawn`]
- [`runtime::Builder::on_task_terminate`]
- [`runtime::Builder::unhandled_panic`]
- [`runtime::TaskMeta`](https://docs.rs/tokio/1.49.0/tokio/runtime/task_hooks/struct.TaskMeta.html)

This flag enables **unstable** features. The public API of these features
may break in 1.x releases. To enable these features, the `--cfg
tokio_unstable` argument must be passed to `rustc` when compiling. This
serves to explicitly opt-in to features which may break semver conventions,
since Cargo [does not yet directly support such opt-ins][unstable features].

You can specify it in your project’s `.cargo/config.toml` file:

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

<div class="warning">
The <code>[build]</code> section does <strong>not</strong> go in a
<code>Cargo.toml</code> file. Instead it must be placed in the Cargo config
file <code>.cargo/config.toml</code>.
</div>

Alternatively, you can specify it with an environment variable:

```sh
## Many *nix shells:
export RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

```powershell
## Windows PowerShell:
$Env:RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

[unstable features]: https://internals.rust-lang.org/t/feature-request-unstable-opt-in-non-transitive-crate-features/16193#why-not-a-crate-feature-2
[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

# Supported platforms

Tokio currently guarantees support for the following platforms:

 * Linux
 * Windows
 * Android (API level 21)
 * macOS
 * iOS
 * FreeBSD

Tokio will continue to support these platforms in the future. However,
future releases may change requirements such as the minimum required libc
version on Linux, the API level on Android, or the supported FreeBSD
release.

Beyond the above platforms, Tokio is intended to work on all platforms
supported by the mio crate. You can find a longer list [in mio’s
documentation][mio-supported]. However, these additional platforms may
become unsupported in the future.

Note that Wine is considered to be a different platform from Windows. See
mio’s documentation for more information on Wine support.

[mio-supported]: https://crates.io/crates/mio#platforms

## `WASM` support

Tokio has some limited support for the `WASM` platform. Without the
`tokio_unstable` flag, the following features are supported:

 * `sync`
 * `macros`
 * `io-util`
 * `rt`
 * `time`

Enabling any other feature (including `full`) will cause a compilation
failure.

The `time` module will only work on `WASM` platforms that have support for
timers (e.g. wasm32-wasi). The timing functions will panic if used on a `WASM`
platform that does not support timers.

Note also that if the runtime becomes indefinitely idle, it will panic
immediately instead of blocking forever. On platforms that don’t support
time, this means that the runtime can never be idle in any way.

## Unstable `WASM` support

Tokio also has unstable support for some additional `WASM` features. This
requires the use of the `tokio_unstable` flag.

Using this flag enables the use of `tokio::net` on the wasm32-wasi target.
However, not all methods are available on the networking types as `WASI`
currently does not support the creation of new sockets from within `WASM`.
Because of this, sockets must currently be created via the `FromRawFd`
trait.