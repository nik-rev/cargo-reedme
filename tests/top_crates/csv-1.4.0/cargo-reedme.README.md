The `csv` crate provides a fast and flexible CSV reader and writer, with
support for Serde.

The [tutorial](tutorial/index.html) is a good place to start if you’re new to
Rust.

The [cookbook](cookbook/index.html) will give you a variety of complete Rust
programs that do CSV reading and writing.

# Brief overview

**If you’re new to Rust**, you might find the
[tutorial](tutorial/index.html)
to be a good place to start.

The primary types in this crate are
[`Reader`](struct.Reader.html)
and
[`Writer`](struct.Writer.html),
for reading and writing CSV data respectively.
Correspondingly, to support CSV data with custom field or record delimiters
(among many other things), you should use either a
[`ReaderBuilder`](struct.ReaderBuilder.html)
or a
[`WriterBuilder`](struct.WriterBuilder.html),
depending on whether you’re reading or writing CSV data.

Unless you’re using Serde, the standard CSV record types are
[`StringRecord`](struct.StringRecord.html)
and
[`ByteRecord`](struct.ByteRecord.html).
`StringRecord` should be used when you know your data to be valid UTF-8.
For data that may be invalid UTF-8, `ByteRecord` is suitable.

Finally, the set of errors is described by the
[`Error`](struct.Error.html)
type.

The rest of the types in this crate mostly correspond to more detailed errors,
position information, configuration knobs or iterator types.

# Setup

Run `cargo add csv` to add the latest version of the `csv` crate to your
Cargo.toml.

If you want to use Serde’s custom derive functionality on your custom structs,
then run `cargo add serde --features derive` to add the `serde` crate with its
`derive` feature enabled to your `Cargo.toml`.

# Example

This example shows how to read CSV data from stdin and print each record to
stdout.

There are more examples in the [cookbook](cookbook/index.html).

```rust
use std::{error::Error, io, process};

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
```

The above example can be run like so:

```rust
$ git clone git://github.com/BurntSushi/rust-csv
$ cd rust-csv
$ cargo run --example cookbook-read-basic < examples/data/smallpop.csv
```

# Example with Serde

This example shows how to read CSV data from stdin into your own custom struct.
By default, the member names of the struct are matched with the values in the
header record of your CSV data.

```rust
use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
```

The above example can be run like so:

```rust
$ git clone git://github.com/BurntSushi/rust-csv
$ cd rust-csv
$ cargo run --example cookbook-read-serde < examples/data/smallpop.csv
```