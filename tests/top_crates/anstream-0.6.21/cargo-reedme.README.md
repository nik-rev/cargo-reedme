**Auto-adapting [`stdout`](https://docs.rs/anstream/0.6.21/anstream/fn.stdout.html) / [`stderr`](https://docs.rs/anstream/0.6.21/anstream/fn.stderr.html) streams**

*A portmanteau of “ansi stream”*

[`AutoStream`](https://docs.rs/anstream/0.6.21/anstream/auto/struct.AutoStream.html) always accepts [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
[AutoStream](https://docs.rs/anstream/0.6.21/anstream/auto/struct.AutoStream.html).

Benefits
- Allows the caller to not be concerned with the terminal’s capabilities
- Semver safe way of passing styled text between crates as ANSI escape codes offer more
  compatibility than most crate APIs.

Available styling crates:
- [anstyle](https://docs.rs/anstyle) for minimal runtime styling, designed to go in public APIs
- [owo-colors](https://docs.rs/owo-colors) for feature-rich runtime styling
- [color-print](https://docs.rs/color-print) for feature-rich compile-time styling

# Example

```rust
use anstream::println;
use owo_colors::OwoColorize as _;

// Foreground colors
println!("My number is {:#x}!", 10.green());
// Background colors
println!("My number is not {}!", 4.on_red());
```

And this will correctly handle piping to a file, etc