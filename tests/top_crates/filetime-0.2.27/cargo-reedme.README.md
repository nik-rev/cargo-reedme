Timestamps for files in Rust

This library provides platform-agnostic inspection of the various timestamps
present in the standard `fs::Metadata` structure.

# Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
filetime = "0.2"
```

# Usage

```rust
use std::fs;
use filetime::FileTime;

let metadata = fs::metadata("foo.txt").unwrap();

let mtime = FileTime::from_last_modification_time(&metadata);
println!("{}", mtime);

let atime = FileTime::from_last_access_time(&metadata);
assert!(mtime < atime);

// Inspect values that can be interpreted across platforms
println!("{}", mtime.unix_seconds());
println!("{}", mtime.nanoseconds());

// Print the platform-specific value of seconds
println!("{}", mtime.seconds());
```