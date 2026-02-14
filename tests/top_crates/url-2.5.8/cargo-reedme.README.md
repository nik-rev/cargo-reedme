
rust-url is an implementation of the [URL Standard](http://url.spec.whatwg.org/)
for the [Rust](http://rust-lang.org/) programming language.


# URL parsing and data structures

First, URL parsing may fail for various reasons and therefore returns a `Result`.

```rust
use url::{Url, ParseError};

assert!(Url::parse("http://[:::1]") == Err(ParseError::InvalidIpv6Address))
```

Let’s parse a valid URL and look at its components.

```rust
use url::{Url, Host, Position};
let issue_list_url = Url::parse(
    "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
)?;


assert!(issue_list_url.scheme() == "https");
assert!(issue_list_url.username() == "");
assert!(issue_list_url.password() == None);
assert!(issue_list_url.host_str() == Some("github.com"));
assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
assert!(issue_list_url.port() == None);
assert!(issue_list_url.path() == "/rust-lang/rust/issues");
assert!(issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
        Some(vec!["rust-lang", "rust", "issues"]));
assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
assert!(&issue_list_url[Position::BeforePath..] == "/rust-lang/rust/issues?labels=E-easy&state=open");
assert!(issue_list_url.fragment() == None);
assert!(!issue_list_url.cannot_be_a_base());
```

Some URLs are said to be *cannot-be-a-base*:
they don’t have a username, password, host, or port,
and their “path” is an arbitrary string rather than slash-separated segments:

```rust
use url::Url;

let data_url = Url::parse("data:text/plain,Hello?World#")?;

assert!(data_url.cannot_be_a_base());
assert!(data_url.scheme() == "data");
assert!(data_url.path() == "text/plain,Hello");
assert!(data_url.path_segments().is_none());
assert!(data_url.query() == Some("World"));
assert!(data_url.fragment() == Some(""));
```

## Default Features

Versions `<= 2.5.2` of the crate have no default features. Versions `> 2.5.2` have the default feature ‘std’.
If you are upgrading across this boundary and you have specified `default-features = false`, then
you will need to add the ‘std’ feature or the ‘alloc’ feature to your dependency.
The ‘std’ feature has the same behavior as the previous versions. The ‘alloc’ feature
provides no_std support.

## Serde

Enable the `serde` feature to include `Deserialize` and `Serialize` implementations for `url::Url`.

# Base URL

Many contexts allow URL *references* that can be relative to a *base URL*:

```html
<link rel="stylesheet" href="../main.css">
```

Since parsed URLs are absolute, giving a base is required for parsing relative URLs:

```rust
use url::{Url, ParseError};

assert!(Url::parse("../main.css") == Err(ParseError::RelativeUrlWithoutBase))
```

Use the `join` method on an `Url` to use it as a base URL:

```rust
use url::Url;

let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html")?;
let css_url = this_document.join("../main.css")?;
assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css");
```

# Feature: `serde`

If you enable the `serde` feature, [`Url`](struct.Url.html) will implement
[`serde::Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html) and
[`serde::Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html).
See [serde documentation](https://serde.rs) for more information.

```toml
url = { version = "2", features = ["serde"] }
```

# Feature: `debugger_visualizer`

If you enable the `debugger_visualizer` feature, the `url` crate will include
a [natvis file](https://docs.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects)
for [Visual Studio](https://www.visualstudio.com/) that allows you to view
[`Url`](struct.Url.html) objects in the debugger.

This feature requires Rust 1.71 or later.

```toml
url = { version = "2", features = ["debugger_visualizer"] }
```