Dead simple endianness conversions.
The following operations are implemented on
`u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `f32`, `f64`:


### Read Numbers
```rust
use lebe::prelude::*;
let mut reader: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

let number : u64 = reader.read_from_little_endian()?;
let number = u64::read_from_big_endian(&mut reader)?;
```

### Read Slices
```rust
use std::io::Read;
use lebe::prelude::*;
let mut reader: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

let mut numbers: &mut [u64] = &mut [0, 0];
reader.read_from_little_endian_into(numbers)?;
```

### Write Numbers
```rust
use std::io::Read;
use lebe::prelude::*;
let mut writer: Vec<u8> = Vec::new();

let number: u64 = 1237691;
writer.write_as_big_endian(&number)?;
```

### Write Slices
```rust
use std::io::Write;
use lebe::prelude::*;
let mut writer: Vec<u8> = Vec::new();

let numbers: &[u64] = &[1_u64, 234545_u64];
writer.write_as_little_endian(numbers)?;
```
