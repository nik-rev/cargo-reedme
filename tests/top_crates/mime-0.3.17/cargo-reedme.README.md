# Mime

Mime is now Media Type, technically, but `Mime` is more immediately
understandable, so the main type here is `Mime`.

## What is Mime?

Example mime string: `text/plain`

```rust
let plain_text: mime::Mime = "text/plain".parse().unwrap();
assert_eq!(plain_text, mime::TEXT_PLAIN);
```

## Inspecting Mimes

```rust
let mime = mime::TEXT_PLAIN;
match (mime.type_(), mime.subtype()) {
    (mime::TEXT, mime::PLAIN) => println!("plain text!"),
    (mime::TEXT, _) => println!("structured text"),
    _ => println!("not text"),
}
```