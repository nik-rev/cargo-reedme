A [serde]-compatible [TOML]-parsing library

TOML itself is a simple, ergonomic, and readable configuration format:

```toml
[package]
name = "toml"

[dependencies]
serde = "1.0"
```

The TOML format tends to be relatively common throughout the Rust community
for configuration, notably being used by [Cargo], Rust’s package manager.

## TOML values

A TOML document is represented with the [`Table`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/table/type.Table.html) type which maps `String` to the [`Value`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/value/enum.Value.html) enum:

 ```rust
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(Datetime),
    Array(Array),
    Table(Table),
}
```

## Parsing TOML

The easiest way to parse a TOML document is via the [`Table`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/table/type.Table.html) type:

 ```rust
use toml::Table;

let value = "foo = 'bar'".parse::<Table>().unwrap();

assert_eq!(value["foo"].as_str(), Some("bar"));
```

The [`Table`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/table/type.Table.html) type implements a number of convenience methods and
traits; the example above uses [`FromStr`](https://doc.rust-lang.org/stable/core/str/traits/trait.FromStr.html) to parse a [`str`](https://doc.rust-lang.org/stable/std/primitive.str.html) into a
[`Table`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/table/type.Table.html).

## Deserialization and Serialization

This crate supports [`serde`] 1.0 with a number of
implementations of the `Deserialize`, `Serialize`, `Deserializer`, and
`Serializer` traits. Namely, you’ll find:

* `Deserialize for Table`
* `Serialize for Table`
* `Deserialize for Value`
* `Serialize for Value`
* `Deserialize for Datetime`
* `Serialize for Datetime`
* `Deserializer for de::Deserializer`
* `Serializer for ser::Serializer`
* `Deserializer for Table`
* `Deserializer for Value`

This means that you can use Serde to deserialize/serialize the
[`Table`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/table/type.Table.html) type as well as [`Value`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/value/enum.Value.html) and [`Datetime`](https://docs.rs/toml_datetime/latest/toml_datetime/datetime/struct.Datetime.html) type in this crate. You can also
use the [`Deserializer`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/de/deserializer/struct.Deserializer.html), [`Serializer`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/ser/document/struct.Serializer.html), or [`Table`](https://docs.rs/toml/1.0.1+spec-1.1.0/toml/table/type.Table.html) type itself to act as
a deserializer/serializer for arbitrary types.

An example of deserializing with TOML is:

 ```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config: Config = toml::from_str(r#"
    ip = '127.0.0.1'

    [keys]
    github = 'xxxxxxxxxxxxxxxxx'
    travis = 'yyyyyyyyyyyyyyyyy'
"#).unwrap();

assert_eq!(config.ip, "127.0.0.1");
assert_eq!(config.port, None);
assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
```

You can serialize types in a similar fashion:

 ```rust
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

let config = Config {
    ip: "127.0.0.1".to_string(),
    port: None,
    keys: Keys {
        github: "xxxxxxxxxxxxxxxxx".to_string(),
        travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
    },
};

let toml = toml::to_string(&config).unwrap();
```

[TOML]: https://github.com/toml-lang/toml
[Cargo]: https://crates.io/
[`serde`]: https://serde.rs/
[serde]: https://serde.rs/