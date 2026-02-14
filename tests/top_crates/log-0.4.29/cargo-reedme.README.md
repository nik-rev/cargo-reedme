A lightweight logging facade.

The `log` crate provides a single logging API that abstracts over the
actual logging implementation. Libraries can use the logging API provided
by this crate, and the consumer of those libraries can choose the logging
implementation that is most suitable for its use case.

If no logging implementation is selected, the facade falls back to a “noop”
implementation that ignores all log messages. The overhead in this case
is very small - just an integer load, comparison and jump.

A log request consists of a _target_, a _level_, and a _body_. A target is a
string which defaults to the module path of the location of the log request,
though that default may be overridden. Logger implementations typically use
the target to filter requests based on some user configuration.

# Usage

The basic use of the log crate is through the five logging macros: [`error!`],
[`warn!`], [`info!`], [`debug!`] and [`trace!`]
where `error!` represents the highest-priority log messages
and `trace!` the lowest. The log messages are filtered by configuring
the log level to exclude messages with a lower priority.
Each of these macros accept format strings similarly to [`println!`].


[`error!`]: ./macro.error.html
[`warn!`]: ./macro.warn.html
[`info!`]: ./macro.info.html
[`debug!`]: ./macro.debug.html
[`trace!`]: ./macro.trace.html
[`println!`]: https://doc.rust-lang.org/stable/std/macro.println.html

Avoid writing expressions with side-effects in log statements. They may not be evaluated.

## In libraries

Libraries should link only to the `log` crate, and use the provided
macros to log whatever information will be useful to downstream consumers.

### Examples

```rust
use log::{info, warn};

pub fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", "Commencing yak shaving for {yak:?}");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {razor}");
                yak.shave(razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {err}, retrying");
            }
        }
    }
}
```

## In executables

Executables should choose a logging implementation and initialize it early in the
runtime of the program. Logging implementations will typically include a
function to do this. Any log messages generated before
the implementation is initialized will be ignored.

The executable itself may use the `log` crate to log as well.

### Warning

The logging system may only be initialized once.

## Structured logging

If you enable the `kv` feature you can associate structured values
with your log records. If we take the example from before, we can include
some additional context besides what’s in the formatted message:

```rust
use log::{info, warn};

pub fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", yak:serde; "Commencing yak shaving");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!(razor; "Razor located");
                yak.shave(razor);
                break;
            }
            Err(e) => {
                warn!(e:err; "Unable to locate a razor, retrying");
            }
        }
    }
}
```

See the [`kv`] module documentation for more details.

# Available logging implementations

In order to produce log output executables have to use
a logger implementation compatible with the facade.
There are many available implementations to choose from,
here are some of the most popular ones:

* Simple minimal loggers:
    * [env_logger]
    * [colog]
    * [simple_logger]
    * [simplelog]
    * [pretty_env_logger]
    * [stderrlog]
    * [flexi_logger]
    * [call_logger]
    * [std-logger]
    * [structured-logger]
    * [clang_log]
    * [ftail]
* Complex configurable frameworks:
    * [log4rs]
    * [logforth]
    * [fern]
    * [spdlog-rs]
* Adaptors for other facilities:
    * [syslog]
    * [slog-stdlog]
    * [systemd-journal-logger]
    * [android_log]
    * [win_dbg_logger]
    * [db_logger]
    * [log-to-defmt]
    * [logcontrol-log]
* For WebAssembly binaries:
    * [console_log]
* For dynamic libraries:
    * You may need to construct an FFI-safe wrapper over `log` to initialize in your libraries
* Utilities:
    * [log_err]
    * [log-reload]
    * [alterable_logger]

# Implementing a Logger

Loggers implement the [`Log`] trait. Here’s a very basic example that simply
logs all messages at the [`Error`][level_link], [`Warn`][level_link] or
[`Info`][level_link] levels to stdout:

```rust
use log::{Record, Level, Metadata};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

```

Loggers are installed by calling the [`set_logger`] function. The maximum
log level also needs to be adjusted via the [`set_max_level`] function. The
logging facade uses this as an optimization to improve performance of log
messages at levels that are disabled. It’s important to set it, as it
defaults to [`Off`][filter_link], so no log messages will ever be captured!
In the case of our example logger, we’ll want to set the maximum log level
to [`Info`][filter_link], since we ignore any [`Debug`][level_link] or
[`Trace`][level_link] level log messages. A logging implementation should
provide a function that wraps a call to [`set_logger`] and
[`set_max_level`], handling initialization of the logger:

```rust
use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

Implementations that adjust their configurations at runtime should take care
to adjust the maximum log level as well.

# Use with `std`

`set_logger` requires you to provide a `&'static Log`, which can be hard to
obtain if your logger depends on some runtime configuration. The
`set_boxed_logger` function is available with the `std` Cargo feature. It is
identical to `set_logger` except that it takes a `Box<Log>` rather than a
`&'static Log`:

```rust
pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

# Compile time filters

Log levels can be statically disabled at compile time by enabling one of these Cargo features:

* `max_level_off`
* `max_level_error`
* `max_level_warn`
* `max_level_info`
* `max_level_debug`
* `max_level_trace`

Log invocations at disabled levels will be skipped and will not even be present in the
resulting binary. These features control the value of the `STATIC_MAX_LEVEL` constant. The
logging macros check this value before logging a message. By default, no levels are disabled.

It is possible to override this level for release builds only with the following features:

* `release_max_level_off`
* `release_max_level_error`
* `release_max_level_warn`
* `release_max_level_info`
* `release_max_level_debug`
* `release_max_level_trace`

Libraries should avoid using the max level features because they’re global and can’t be changed
once they’re set.

For example, a crate can disable trace level logs in debug builds and trace, debug, and info
level logs in release builds with the following configuration:

```toml
[dependencies]
log = { version = "0.4", features = ["max_level_debug", "release_max_level_warn"] }
```
# Crate Feature Flags

The following crate feature flags are available in addition to the filters. They are
configured in your `Cargo.toml`.

* `std` allows use of `std` crate instead of the default `core`. Enables using `std::error` and
  `set_boxed_logger` functionality.
* `serde` enables support for serialization and deserialization of `Level` and `LevelFilter`.

```toml
[dependencies]
log = { version = "0.4", features = ["std", "serde"] }
```

# Version compatibility

The 0.3 and 0.4 versions of the `log` crate are almost entirely compatible. Log messages
made using `log` 0.3 will forward transparently to a logger implementation using `log` 0.4. Log
messages made using `log` 0.4 will forward to a logger implementation using `log` 0.3, but the
module path and file name information associated with the message will unfortunately be lost.

[`Log`]: trait.Log.html
[level_link]: enum.Level.html
[filter_link]: enum.LevelFilter.html
[`set_logger`]: fn.set_logger.html
[`set_max_level`]: fn.set_max_level.html
[`try_set_logger_raw`]: fn.try_set_logger_raw.html
[`shutdown_logger_raw`]: fn.shutdown_logger_raw.html
[env_logger]: https://docs.rs/env_logger/*/env_logger/
[colog]: https://docs.rs/colog/*/colog/
[simple_logger]: https://github.com/borntyping/rust-simple_logger
[simplelog]: https://github.com/drakulix/simplelog.rs
[pretty_env_logger]: https://docs.rs/pretty_env_logger/*/pretty_env_logger/
[stderrlog]: https://docs.rs/stderrlog/*/stderrlog/
[flexi_logger]: https://docs.rs/flexi_logger/*/flexi_logger/
[call_logger]: https://docs.rs/call_logger/*/call_logger/
[std-logger]: https://docs.rs/std-logger/*/std_logger/
[syslog]: https://docs.rs/syslog/*/syslog/
[slog-stdlog]: https://docs.rs/slog-stdlog/*/slog_stdlog/
[log4rs]: https://docs.rs/log4rs/*/log4rs/
[logforth]: https://docs.rs/logforth/*/logforth/
[fern]: https://docs.rs/fern/*/fern/
[spdlog-rs]: https://docs.rs/spdlog-rs/*/spdlog/
[systemd-journal-logger]: https://docs.rs/systemd-journal-logger/*/systemd_journal_logger/
[android_log]: https://docs.rs/android_log/*/android_log/
[win_dbg_logger]: https://docs.rs/win_dbg_logger/*/win_dbg_logger/
[db_logger]: https://docs.rs/db_logger/*/db_logger/
[log-to-defmt]: https://docs.rs/log-to-defmt/*/log_to_defmt/
[console_log]: https://docs.rs/console_log/*/console_log/
[structured-logger]: https://docs.rs/structured-logger/latest/structured_logger/
[logcontrol-log]: https://docs.rs/logcontrol-log/*/logcontrol_log/
[log_err]: https://docs.rs/log_err/*/log_err/
[log-reload]: https://docs.rs/log-reload/*/log_reload/
[alterable_logger]: https://docs.rs/alterable_logger/*/alterable_logger
[clang_log]: https://docs.rs/clang_log/latest/clang_log
[ftail]: https://docs.rs/ftail/latest/ftail