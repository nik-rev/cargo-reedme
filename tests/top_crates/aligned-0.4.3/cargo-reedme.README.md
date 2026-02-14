A newtype with alignment of at least `A` bytes

# Examples

```rust
use std::mem;

use aligned::{Aligned, A2, A4, A16};

// Array aligned to a 2 byte boundary
static X: Aligned<A2, [u8; 3]> = Aligned([0; 3]);

// Array aligned to a 4 byte boundary
static Y: Aligned<A4, [u8; 3]> = Aligned([0; 3]);

// Unaligned array
static Z: [u8; 3] = [0; 3];

// You can allocate the aligned arrays on the stack too
let w: Aligned<A16, _> = Aligned([0u8; 3]);

assert_eq!(mem::align_of_val(&X), 2);
assert_eq!(mem::align_of_val(&Y), 4);
assert_eq!(mem::align_of_val(&Z), 1);
assert_eq!(mem::align_of_val(&w), 16);
```