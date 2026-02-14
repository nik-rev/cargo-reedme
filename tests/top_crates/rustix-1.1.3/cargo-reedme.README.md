`rustix` provides efficient memory-safe and [I/O-safe] wrappers to
POSIX-like, Unix-like, Linux, and Winsock syscall-like APIs, with
configurable backends.

With rustix, you can write code like this:

```rust
let (nread, _received) = rustix::net::recv(&sock, buf, RecvFlags::PEEK)?;
```

instead of like this:

```rust
let nread = unsafe {
    #[cfg(any(unix, target_os = "wasi"))]
    let raw = sock.as_raw_fd();
    #[cfg(windows)]
    let raw = sock.as_raw_socket();
    match libc::recv(
        raw as _,
        buf.as_mut_ptr().cast(),
        buf.len().try_into().unwrap_or(i32::MAX as _),
        MSG_PEEK,
    ) {
        -1 => return Err(std::io::Error::last_os_error()),
        nread => nread as usize,
    }
};
```

rustix’s APIs perform the following tasks:
 - Error values are translated to [`Result`](https://doc.rust-lang.org/stable/core/result/enum.Result.html)s.
 - Buffers are passed as Rust slices.
 - Out-parameters are presented as return values.
 - Path arguments use [`Arg`], so they accept any string type.
 - File descriptors are passed and returned via [`AsFd`] and [`OwnedFd`]
   instead of bare integers, ensuring I/O safety.
 - Constants use `enum`s and [`bitflags`] types, and enable [support for
   externally defined flags].
 - Multiplexed functions (eg. `fcntl`, `ioctl`, etc.) are de-multiplexed.
 - Variadic functions (eg. `openat`, etc.) are presented as non-variadic.
 - Functions that return strings automatically allocate sufficient memory
   and retry the syscall as needed to determine the needed length.
 - Functions and types which need `l` prefixes or `64` suffixes to enable
   large-file support (LFS) are used automatically. File sizes and offsets
   are always presented as `u64` and `i64`.
 - Behaviors that depend on the sizes of C types like `long` are hidden.
 - In some places, more human-friendly and less historical-accident names
   are used (and documentation aliases are used so that the original names
   can still be searched for).
 - Provide y2038 compatibility, on platforms which support this.
 - Correct selected platform bugs, such as behavioral differences when
   running under seccomp.
 - Use `timespec` for timestamps and timeouts instead of `timeval` and
   `c_int` milliseconds.

Things they don’t do include:
 - Detecting whether functions are supported at runtime, except in specific
   cases where new interfaces need to be detected to support y2038 and LFS.
 - Hiding significant differences between platforms.
 - Restricting ambient authorities.
 - Imposing sandboxing features such as filesystem path or network address
   sandboxing.

See [`cap-std`], [`system-interface`], and [`io-streams`] for libraries
which do hide significant differences between platforms, and [`cap-std`]
which does perform sandboxing and restricts ambient authorities.

[`cap-std`]: https://crates.io/crates/cap-std
[`system-interface`]: https://crates.io/crates/system-interface
[`io-streams`]: https://crates.io/crates/io-streams
[`bitflags`]: bitflags
[`AsFd`]: crate::fd::AsFd
[`OwnedFd`]: crate::fd::OwnedFd
[Ihttps://doc.rust-lang.org/stable/std/os/fd/owned/trait.AsFd.html://github.com/hthttps://docs.rs/bitflags/latest/bitflags/c.rust-lang.org/stable/std/os/fd/owned/struct.OwnedFd.htmlb/master/text/3128-io-safety.md
[`Arg`]: https://docs.rs/rustix/1.1.3/rustix/path/arg/trait.Arg.html
[support for externally defined flags]: bitflags#externally-defined-flags