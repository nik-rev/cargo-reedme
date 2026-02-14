> winnow, making parsing a breeze

`winnow` is a parser combinator library

Quick links:
- [crate::combinator](https://docs.rs/winnow/0.7.14/winnow/combinator/)
- [Tutorial][_tutorial::chapter_0]
- [Special Topics][_topic]
- [Discussions](https://github.com/winnow-rs/winnow/discussions)
- [CHANGELOG](https://github.com/winnow-rs/winnow/blob/v0.7.14/CHANGELOG.md) (includes major version migration
  guides)

## Aspirations

`winnow` aims to be your “do everything” parser, much like people treat regular expressions.

In roughly priority order:
1. Support writing parser declaratively while not getting in the way of imperative-style
   parsing when needed, working as an open-ended toolbox rather than a close-ended framework.
2. Flexible enough to be used for any application, including parsing strings, binary data,
   or separate [lexing and parsing phases][_topic::lexing]
3. Zero-cost abstractions, making it easy to write high performance parsers
4. Easy to use, making it trivial for one-off uses

In addition:
- Resilient maintainership, including
  - Willing to break compatibility rather than batching up breaking changes in large releases
  - Leverage feature flags to keep one active branch
- We will support the last 6 months of rust releases (MSRV, currently 1.64.0)

See also [Special Topic: Why winnow?][crate::_topic::why]

## Example

Run
```console
$ cargo add winnow
```

Then use it to parse:
```rust
use winnow::combinator::seq;
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::Result;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl std::str::FromStr for Color {
    // The error must be owned
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex_color.parse(s).map_err(|e| e.to_string())
    }
}

pub(crate) fn hex_color(input: &mut &str) -> Result<Color> {
    seq!(Color {
        _: '#',
        red: hex_primary,
        green: hex_primary,
        blue: hex_primary
    })
    .parse_next(input)
}

fn hex_primary(input: &mut &str) -> Result<u8> {
    take_while(2, |c: char| c.is_ascii_hexdigit())
        .try_map(|input| u8::from_str_radix(input, 16))
        .parse_next(input)
}
```

See also the [Tutorial][_tutorial::chapter_0] and [Special Topics][_topic]