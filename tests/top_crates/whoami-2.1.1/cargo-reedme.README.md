Rust library for getting information about the current user and environment.

## Getting Started

Using the whoami crate is super easy!  All of the public items are simple
functions with no parameters that return [`String`]s or [`OsString`]s (with
the exception of [`desktop_env()`](https://docs.rs/whoami/2.1.1/whoami/api/fn.desktop_env.html), [`platform()`](https://docs.rs/whoami/2.1.1/whoami/api/fn.platform.html), and [`cpu_arch()`](https://docs.rs/whoami/2.1.1/whoami/api/fn.cpu_arch.html),
which return enums, and [`lang_prefs()`](https://docs.rs/whoami/2.1.1/whoami/api/fn.lang_prefs.html) that returns
[`LanguagePreferences`](https://docs.rs/whoami/2.1.1/whoami/langs/struct.LanguagePreferences.html)).  The following example shows how to use all of the
functions (except those that return [`OsString`]):

```rust
println!(
    "User's Language        whoami::lang_prefs():          {}",
    whoami::lang_prefs().unwrap_or_default(),
);
println!(
    "User's Name            whoami::realname():            {}",
    whoami::realname().unwrap_or_else(|_| "<unknown>".to_string()),
);
println!(
    "User's Username        whoami::username():            {}",
    whoami::username().unwrap_or_else(|_| "<unknown>".to_string()),
);
println!(
    "User's Username        whoami::account():             {}",
    whoami::account().unwrap_or_else(|_| "<unknown>".to_string()),
);
println!(
    "Device's Pretty Name   whoami::devicename():          {}",
    whoami::devicename().unwrap_or_else(|_| "<unknown>".to_string()),
);
println!(
    "Device's Hostname      whoami::hostname():            {}",
    whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string()),
);
println!(
    "Device's Platform      whoami::platform():            {}",
    whoami::platform(),
);
println!(
    "Device's OS Distro     whoami::distro():              {}",
    whoami::distro().unwrap_or_else(|_| "<unknown>".to_string()),
);
println!(
    "Device's Desktop Env.  whoami::desktop_env():         {}",
    whoami::desktop_env()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "<unknown>".to_string()),
);
println!(
    "Device's CPU Arch      whoami::cpu_arch():            {}",
    whoami::cpu_arch(),
);
```

[`OsString`]: std::ffi::OsString
[`String`]: https://doc.ruhttps://doc.rust-lang.org/stable/std/ffi/os_str/sthttps://doc.rust-lang.org/stable/std/ffi/os_str/struct.OsString.html/alloc/string/struct.String.html