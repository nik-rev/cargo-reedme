A pure-Rust library to manage extended attributes.

It provides support for manipulating extended attributes
(`xattrs`) on modern Unix filesystems. See the `attr(5)`
manpage for more details.

An extension trait [`FileExt`](https://docs.rs/xattr/1.6.1/xattr/trait.FileExt.html) is provided to directly work with
standard `File` objects and file descriptors.

If the path argument is a symlink, the get/set/list/remove functions
operate on the symlink itself. To operate on the symlink target, use
the _deref variant of these functions.

```rust
let mut xattrs = xattr::list("/").unwrap().peekable();

if xattrs.peek().is_none() {
    println!("no xattr set on root");
    return;
}

println!("Extended attributes:");
for attr in xattrs {
    println!(" - {:?}", attr);
}
```