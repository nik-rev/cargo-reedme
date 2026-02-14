This is a library for creating temporary files and directories that are automatically deleted
when no longer referenced (i.e., on drop).

- Use [`tempfile()`] when you need a real [`std::fs::File`](https://doc.rust-lang.org/stable/std/fs/struct.File.html) but don’t need to refer to it
  by-path.
- Use [`NamedTempFile::new()`](https://docs.rs/tempfile/3.25.0/tempfile/file/struct.NamedTempFile.html#method.new) when you need a _named_ temporary file that can be refered to its
  path.
- Use [`tempdir()`] when you need a temporary directory that will be recursively deleted on drop.
- Use [`spooled_tempfile()`](https://docs.rs/tempfile/3.25.0/tempfile/spooled/fn.spooled_tempfile.html) when you need an in-memory buffer that will ultimately be backed by
  a temporary file if it gets too large.

# Design

This crate provides several approaches to creating temporary files and directories.
[`tempfile()`] relies on the OS to remove the temporary file once the last handle is closed.
[`TempDir`] and [`NamedTempFile`] both rely on Rust destructors for cleanup.

## Resource Leaking

`tempfile` will (almost) never fail to cleanup temporary resources. However `TempDir` and
`NamedTempFile` will fail if their destructors don’t run. This is because `tempfile` relies on
the OS to cleanup the underlying file, while `TempDir` and `NamedTempFile` rely on rust
destructors to do so. Destructors may fail to run if the process exits through an unhandled
signal interrupt (like `SIGINT`), or if the instance is declared statically (like with
[`lazy_static`]), among other possible reasons.

## Unexpected File Deletion

Most operating systems periodically clean up temporary files that haven’t been accessed recently
(often on the order of multiple days). This issue does not affect unnamed temporary files but
can invalidate the paths associated with named temporary files on Unix-like systems because the
temporary file can be unlinked from the filesystem while still open and in-use. See the
[temporary file cleaner](#temporary-file-cleaners) section for more security implications.

## Security

This section discusses security issues relevant to Unix-like operating systems that use shared
temporary directories by default. Importantly, it’s not relevant for Windows or macOS as both
operating systems use private per-user temporary directories by default.

Applications can mitigate the issues described below by using [`env::override_temp_dir`](https://docs.rs/tempfile/3.25.0/tempfile/env/fn.override_temp_dir.html) to
change the default temporary directory but should do so if and only if default the temporary
directory ([`env::temp_dir`](https://docs.rs/tempfile/3.25.0/tempfile/env/fn.temp_dir.html)) is unsuitable (is world readable, world writable, managed by a
temporary file cleaner, etc.).

### Temporary File Cleaners

In the presence of pathological temporary file cleaner, relying on file paths is unsafe because
a temporary file cleaner could delete the temporary file which an attacker could then replace.

This isn’t an issue for [`tempfile`](https://docs.rs/tempfile/3.25.0/tempfile/file/fn.tempfile.html) as it doesn’t rely on file paths. However, [`NamedTempFile`]
and temporary directories _do_ rely on file paths for _some_ operations. See the security
documentation on the [`NamedTempFile`] and the [`TempDir`] types for more information.

Mitigation:

- This is rarely an issue for short-lived files as temporary file cleaners usually only remove
  temporary files that haven’t been modified or accessed within many (10-30) days.
- Very long lived temporary files should be placed in directories not managed by temporary file
  cleaners.

### Access Permissions

Temporary _files_ created with this library are private by default on all operating systems.
However, temporary _directories_ are created with the default permissions and will therefore be
world-readable by default unless the user has changed their umask and/or default temporary
directory.

### Denial of Service

If the file-name randomness ([`Builder::rand_bytes`](https://docs.rs/tempfile/3.25.0/tempfile/struct.Builder.html#method.rand_bytes)) is too small and/or this crate is built
without the `getrandom` feature, it may be possible for an attacker to predict the random file
names chosen by this library, preventing temporary file creation by creating temporary files
with these predicted file names. By default, this library mitigates this denial of service
attack by:

1. Defaulting to 6 random characters per temporary file forcing an attacker to create billions
   of files before random collisions are expected (at which point you probably have larger
   problems).
2. Re-seeding the random filename generator from system randomness after 3 failed attempts to
   create temporary a file (when the `getrandom` feature is enabled as it is by default on all
   major platforms).

## Early drop pitfall

Because `TempDir` and `NamedTempFile` rely on their destructors for cleanup, this can lead
to an unexpected early removal of the directory/file, usually when working with APIs which are
generic over `AsRef<Path>`. Consider the following example:

```rust
use tempfile::tempdir;
use std::process::Command;

// Create a directory inside of `env::temp_dir()`.
let temp_dir = tempdir()?;

// Spawn the `touch` command inside the temporary directory and collect the exit status
// Note that `temp_dir` is **not** moved into `current_dir`, but passed as a reference
let exit_status = Command::new("touch").arg("tmp").current_dir(&temp_dir).status()?;
assert!(exit_status.success());

```

This works because a reference to `temp_dir` is passed to `current_dir`, resulting in the
destructor of `temp_dir` being run after the `Command` has finished execution. Moving the
`TempDir` into the `current_dir` call would result in the `TempDir` being converted into
an internal representation, with the original value being dropped and the directory thus
being deleted, before the command can be executed.

The `touch` command would fail with an `No such file or directory` error.

## Examples

Create a temporary file and write some data into it:

```rust
use tempfile::tempfile;
use std::io::Write;

// Create a file inside of `env::temp_dir()`.
let mut file = tempfile()?;

writeln!(file, "Brian was here. Briefly.")?;
```

Create a named temporary file and open an independent file handle:

```rust
use tempfile::NamedTempFile;
use std::io::{Write, Read};

let text = "Brian was here. Briefly.";

// Create a file inside of `env::temp_dir()`.
let mut file1 = NamedTempFile::new()?;

// Re-open it.
let mut file2 = file1.reopen()?;

// Write some test data to the first handle.
file1.write_all(text.as_bytes())?;

// Read the test data using the second handle.
let mut buf = String::new();
file2.read_to_string(&mut buf)?;
assert_eq!(buf, text);
```

Create a temporary directory and add a file to it:

```rust
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

// Create a directory inside of `env::temp_dir()`.
let dir = tempdir()?;

let file_path = dir.path().join("my-temporary-note.txt");
let mut file = File::create(file_path)?;
writeln!(file, "Brian was here. Briefly.")?;

// By closing the `TempDir` explicitly, we can check that it has
// been deleted successfully. If we don't close it explicitly,
// the directory will still be deleted when `dir` goes out
// of scope, but we won't know whether deleting the directory
// succeeded.
drop(file);
dir.close()?;
```

[`tempfile()`]: fn.tempfile.html
[`tempdir()`]: fn.tempdir.html
[`TempDir`]: struct.TempDir.html
[`NamedTempFile`]: struct.NamedTempFile.html
[`lazy_static`]: https://github.com/rust-lang-nursery/lazy-static.rs/issues/62