A crate used for calculating offsets of struct members and their spans.

This functionality currently can not be used in compile time code such as `const` or `const fn` definitions.

## Examples
```rust
use memoffset::{offset_of, span_of};

#[repr(C, packed)]
struct HelpMeIAmTrappedInAStructFactory {
    help_me_before_they_: [u8; 15],
    a: u32
}

assert_eq!(offset_of!(HelpMeIAmTrappedInAStructFactory, a), 15);
assert_eq!(span_of!(HelpMeIAmTrappedInAStructFactory, a), 15..19);
assert_eq!(span_of!(HelpMeIAmTrappedInAStructFactory, help_me_before_they_ .. a), 0..15);
```

This functionality can be useful, for example, for checksum calculations:

```rust
#[repr(C, packed)]
struct Message {
    header: MessageHeader,
    fragment_index: u32,
    fragment_count: u32,
    payload: [u8; 1024],
    checksum: u16
}

let checksum_range = &raw[span_of!(Message, header..checksum)];
let checksum = crc16(checksum_range);
```