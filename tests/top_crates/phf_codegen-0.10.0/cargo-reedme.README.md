A set of builders to generate Rust source for PHF data structures at
compile time.

The provided builders are intended to be used in a Cargo build script to
generate a Rust source file that will be included in a library at build
time.

# Examples

build.rs:

```rust
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
         "static KEYWORDS: phf::Map<&'static str, Keyword> = \n{};\n",
         phf_codegen::Map::new()
             .entry("loop", "Keyword::Loop")
             .entry("continue", "Keyword::Continue")
             .entry("break", "Keyword::Break")
             .entry("fn", "Keyword::Fn")
             .entry("extern", "Keyword::Extern")
             .build()
    ).unwrap();
}
```

lib.rs:

```rust
#[derive(Clone)]
enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}
```

##### Byte-String Keys
Byte strings by default produce references to fixed-size arrays; the compiler needs a hint
to coerce them to slices:

build.rs:

```rust
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
         "static KEYWORDS: phf::Map<&'static [u8], Keyword> = \n{};\n",
         phf_codegen::Map::<&[u8]>::new()
             .entry(b"loop", "Keyword::Loop")
             .entry(b"continue", "Keyword::Continue")
             .entry(b"break", "Keyword::Break")
             .entry(b"fn", "Keyword::Fn")
             .entry(b"extern", "Keyword::Extern")
             .build()
    ).unwrap();
}
```

lib.rs:

```rust
#[derive(Clone)]
enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn parse_keyword(keyword: &[u8]) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}
```

# Note

The compiler’s stack will overflow when processing extremely long method
chains (500+ calls). When generating large PHF data structures, consider
looping over the entries or making each call a separate statement:

```rust
let entries = [("hello", "1"), ("world", "2")];

let mut builder = phf_codegen::Map::new();
for &(key, value) in &entries {
    builder.entry(key, value);
}
// ...
```

```rust
let mut builder = phf_codegen::Map::new();
builder.entry("hello", "1");
builder.entry("world", "2");
// ...
```