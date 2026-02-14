> **Command Line Argument Parser for Rust**

Quick Links:
- Derive [_derive::_tutorial](https://docs.rs/clap/4.5.58/clap/_derive/_tutorial/) and [_derive](https://docs.rs/clap/4.5.58/clap/_derive/)
- Builder [_tutorial](https://docs.rs/clap/4.5.58/clap/_tutorial/) and [Command](https://docs.rs/clap_builder/latest/clap_builder/builder/command/struct.Command.html)
- [_cookbook](https://docs.rs/clap/4.5.58/clap/_cookbook/)
- [_concepts](https://docs.rs/clap/4.5.58/clap/_concepts/)
- [_faq](https://docs.rs/clap/4.5.58/clap/_faq/)
- [Discussions](https://github.com/clap-rs/clap/discussions)
- [CHANGELOG](https://github.com/clap-rs/clap/blob/v4.5.58/CHANGELOG.md) (includes major version migration
  guides)

## Aspirations

- Out of the box, users get a polished CLI experience
  - Including common argument behavior, help generation, suggested fixes for users, colored output, [shell completions](https://github.com/clap-rs/clap/tree/master/clap_complete), etc
- Flexible enough to port your existing CLI interface
  - However, we won’t necessarily streamline support for each use case
- Reasonable parse performance
- Resilient maintainership, including
  - Willing to break compatibility rather than batching up breaking changes in large releases
  - Leverage feature flags to keep to one active branch
  - Being under [WG-CLI](https://github.com/rust-cli/team/) to increase the bus factor
- We follow semver and will wait about 6-9 months between major breaking changes
- We will support the last two minor Rust releases (MSRV, currently 1.74)

While these aspirations can be at odds with fast build times and low binary
size, we will still strive to keep these reasonable for the flexibility you
get.  Check out the
[argparse-benchmarks](https://github.com/rust-cli/argparse-benchmarks-rs) for
CLI parsers optimized for other use cases.

## Example

Run
```console
$ cargo add clap --features derive
```
*(See also [_features](https://docs.rs/clap/4.5.58/clap/_features/))*

Then define your CLI in `main.rs`:
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

And try it out:
```console
$ demo --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: demo[EXE] [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version

$ demo --name Me
Hello Me!

```
*(version number and `.exe` extension on windows replaced by placeholders)*

See also the derive [_derive::_tutorial](https://docs.rs/clap/4.5.58/clap/_derive/_tutorial/) and [_derive](https://docs.rs/clap/4.5.58/clap/_derive/)

### Related Projects

Augment clap:
- [wild](https://crates.io/crates/wild) for supporting wildcards (`*`) on Windows like you do Linux
- [argfile](https://crates.io/crates/argfile) for loading additional arguments from a file (aka response files)
- [shadow-rs](https://crates.io/crates/shadow-rs) for generating `Command::long_version`
- [clap_mangen](https://crates.io/crates/clap_mangen) for generating man page source (roff)
- [clap_complete](https://crates.io/crates/clap_complete) for shell completion support
- [clap-i18n-richformatter](https://crates.io/crates/clap-i18n-richformatter) for i18n support with `clap::error::RichFormatter`

CLI Helpers
- [clio](https://crates.io/crates/clio) for reading/writing to files specified as arguments
- [clap-verbosity-flag](https://crates.io/crates/clap-verbosity-flag)
- [clap-cargo](https://crates.io/crates/clap-cargo)
- [colorchoice-clap](https://crates.io/crates/colorchoice-clap)

Testing
- [`trycmd`](https://crates.io/crates/trycmd):  Bulk snapshot testing
- [`snapbox`](https://crates.io/crates/snapbox):  Specialized snapshot testing
- [`assert_cmd`](https://crates.io/crates/assert_cmd) and [`assert_fs`](https://crates.io/crates/assert_fs): Customized testing

Documentation:
- [Command-line Apps for Rust](https://rust-cli.github.io/book/index.html) book
