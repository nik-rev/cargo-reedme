Cross-platform interface to the `errno` variable.

# Examples
```rust
use errno::{Errno, errno, set_errno};

// Get the current value of errno
let e = errno();

// Set the current value of errno
set_errno(e);

// Extract the error code as an i32
let code = e.0;

// Display a human-friendly error message
println!("Error {}: {}", code, e);
```