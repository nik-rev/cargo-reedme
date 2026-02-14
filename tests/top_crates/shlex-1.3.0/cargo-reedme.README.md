Parse strings like, and escape strings for, POSIX shells.

Same idea as (but implementation not directly based on) the Python shlex module.

Disabling the `std` feature (which is enabled by default) will allow the crate to work in
`no_std` environments, where the `alloc` crate, and a global allocator, are available.

## <span style="color:red">Warning</span>

The [`try_quote`](https://docs.rs/shlex/1.3.0/shlex/fn.try_quote.html)/[`try_join`](https://docs.rs/shlex/1.3.0/shlex/fn.try_join.html) family of APIs does not quote control characters (because they
cannot be quoted portably).

This is fully safe in noninteractive contexts, like shell scripts and `sh -c` arguments (or
even scripts `source`d from interactive shells).

But if you are quoting for human consumption, you should keep in mind that ugly inputs produce
ugly outputs (which may not be copy-pastable).

And if by chance you are piping the output of [`try_quote`](https://docs.rs/shlex/1.3.0/shlex/fn.try_quote.html)/[`try_join`](https://docs.rs/shlex/1.3.0/shlex/fn.try_join.html) directly to the stdin
of an interactive shell, you should stop, because control characters can lead to arbitrary
command injection.

For more information, and for information about more minor issues, please see [quoting_warning](https://docs.rs/shlex/1.3.0/shlex/quoting_warning/).

## Compatibility

This crate’s quoting functionality tries to be compatible with **any POSIX-compatible shell**;
it’s tested against `bash`, `zsh`, `dash`, Busybox `ash`, and `mksh`, plus `fish` (which is not
POSIX-compatible but close enough).

It also aims to be compatible with Python `shlex` and C `wordexp`.