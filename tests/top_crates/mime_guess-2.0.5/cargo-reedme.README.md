Guessing of MIME types by file extension.

Uses a static list of file-extension : MIME type mappings.

```rust
// the file doesn't have to exist, it just looks at the path
let guess = mime_guess::from_path("some_file.gif");
assert_eq!(guess.first(), Some(mime::IMAGE_GIF));

```

#### Note: MIME Types Returned Are Not Stable/Guaranteed
The media types returned for a given extension are not considered to be part of the crate’s
stable API and are often updated in patch <br /> (`x.y.[z + 1]`) releases to be as correct as
possible.

Additionally, only the extensions of paths/filenames are inspected in order to guess the MIME
type. The file that may or may not reside at that path may or may not be a valid file of the
returned MIME type.  Be wary of unsafe or un-validated assumptions about file structure or
length.