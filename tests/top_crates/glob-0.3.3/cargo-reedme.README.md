Support for matching file paths against Unix shell style patterns.

The `glob` and `glob_with` functions allow querying the filesystem for all
files that match a particular pattern (similar to the libc `glob` function).
The methods on the `Pattern` type provide functionality for checking if
individual paths match a particular pattern (similar to the libc `fnmatch`
function).

For consistency across platforms, and for Windows support, this module
is implemented entirely in Rust rather than deferring to the libc
`glob`/`fnmatch` functions.

# Examples

To print all jpg files in `/media/` and all of its subdirectories.

```rust
use glob::glob;

for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
```

To print all files containing the letter “a”, case insensitive, in a `local`
directory relative to the current working directory. This ignores errors
instead of printing them.

```rust
use glob::glob_with;
use glob::MatchOptions;

let options = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};
for entry in glob_with("local/*a*", options).unwrap() {
    if let Ok(path) = entry {
        println!("{:?}", path.display())
    }
}
```