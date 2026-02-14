A simple utility for getting the size of a terminal.

Works on Linux, macOS, Windows, and illumos.

This crate requires a minimum Rust version of 1.63.0 (2022-08-11).

# Example

```rust
use terminal_size::{Width, Height, terminal_size};

let size = terminal_size();
if let Some((Width(w), Height(h))) = size {
    println!("Your terminal is {} cols wide and {} lines tall", w, h);
} else {
    println!("Unable to get terminal size");
}
```
