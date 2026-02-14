encoding_rs is a Gecko-oriented Free Software / Open Source implementation
of the [Encoding Standard](https://encoding.spec.whatwg.org/) in Rust.
Gecko-oriented means that converting to and from UTF-16 is supported in
addition to converting to and from UTF-8, that the performance and
streamability goals are browser-oriented, and that FFI-friendliness is a
goal.

Additionally, the `mem` module provides functions that are useful for
applications that need to be able to deal with legacy in-memory
representations of Unicode.

For expectation setting, please be sure to read the sections
[_UTF-16LE, UTF-16BE and Unicode Encoding Schemes_](#utf-16le-utf-16be-and-unicode-encoding-schemes),
[_ISO-8859-1_](#iso-8859-1) and [_Web / Browser Focus_](#web--browser-focus) below.

There is a [long-form write-up](https://hsivonen.fi/encoding_rs/) about the
design and internals of the crate.

# Availability

The code is available under the
[Apache license, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
or the [MIT license](https://opensource.org/licenses/MIT), at your option.
See the
[`COPYRIGHT`](https://github.com/hsivonen/encoding_rs/blob/master/COPYRIGHT)
file for details.
The [repository is on GitHub](https://github.com/hsivonen/encoding_rs). The
[crate is available on crates.io](https://crates.io/crates/encoding_rs).

# Integration with `std::io`

This crate doesn’t implement traits from `std::io`. However, for the case of
wrapping a `std::io::Read` in a decoder that implements `std::io::Read` and
presents the data from the wrapped `std::io::Read` as UTF-8 is addressed by
the [`encoding_rs_io`](https://docs.rs/encoding_rs_io/) crate.

# Examples

Example programs:

* [Rust](https://github.com/hsivonen/recode_rs)
* [C](https://github.com/hsivonen/recode_c)
* [C++](https://github.com/hsivonen/recode_cpp)

Decode using the non-streaming API:

```rust
#[cfg(feature = "alloc")] {
use encoding_rs::*;

let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";
let bytes = b"\x83n\x83\x8D\x81[\x81E\x83\x8F\x81[\x83\x8B\x83h";

let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(bytes);
assert_eq!(&cow[..], expectation);
assert_eq!(encoding_used, SHIFT_JIS);
assert!(!had_errors);
}
```

Decode using the streaming API with minimal `unsafe`:

```rust
use encoding_rs::*;

let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";

// Use an array of byte slices to demonstrate content arriving piece by
// piece from the network.
let bytes: [&'static [u8]; 4] = [b"\x83",
                                 b"n\x83\x8D\x81",
                                 b"[\x81E\x83\x8F\x81[\x83",
                                 b"\x8B\x83h"];

// Very short output buffer to demonstrate the output buffer getting full.
// Normally, you'd use something like `[0u8; 2048]`.
let mut buffer_bytes = [0u8; 8];
let mut buffer: &mut str = std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap();

// How many bytes in the buffer currently hold significant data.
let mut bytes_in_buffer = 0usize;

// Collect the output to a string for demonstration purposes.
let mut output = String::new();

// The `Decoder`
let mut decoder = SHIFT_JIS.new_decoder();

// Track whether we see errors.
let mut total_had_errors = false;

// Decode using a fixed-size intermediate buffer (for demonstrating the
// use of a fixed-size buffer; normally when the output of an incremental
// decode goes to a `String` one would use `Decoder.decode_to_string()` to
// avoid the intermediate buffer).
for input in &bytes[..] {
    // The number of bytes already read from current `input` in total.
    let mut total_read_from_current_input = 0usize;

    loop {
        let (result, read, written, had_errors) =
            decoder.decode_to_str(&input[total_read_from_current_input..],
                                  &mut buffer[bytes_in_buffer..],
                                  false);
        total_read_from_current_input += read;
        bytes_in_buffer += written;
        total_had_errors |= had_errors;
        match result {
            CoderResult::InputEmpty => {
                // We have consumed the current input buffer. Break out of
                // the inner loop to get the next input buffer from the
                // outer loop.
                break;
            },
            CoderResult::OutputFull => {
                // Write the current buffer out and consider the buffer
                // empty.
                output.push_str(&buffer[..bytes_in_buffer]);
                bytes_in_buffer = 0usize;
                continue;
            }
        }
    }
}

// Process EOF
loop {
    let (result, _, written, had_errors) =
        decoder.decode_to_str(b"",
                              &mut buffer[bytes_in_buffer..],
                              true);
    bytes_in_buffer += written;
    total_had_errors |= had_errors;
    // Write the current buffer out and consider the buffer empty.
    // Need to do this here for both `match` arms, because we exit the
    // loop on `CoderResult::InputEmpty`.
    output.push_str(&buffer[..bytes_in_buffer]);
    bytes_in_buffer = 0usize;
    match result {
        CoderResult::InputEmpty => {
            // Done!
            break;
        },
        CoderResult::OutputFull => {
            continue;
        }
    }
}

assert_eq!(&output[..], expectation);
assert!(!total_had_errors);
```

## UTF-16LE, UTF-16BE and Unicode Encoding Schemes

The Encoding Standard doesn’t specify encoders for UTF-16LE and UTF-16BE,
__so this crate does not provide encoders for those encodings__!
Along with the replacement encoding, their _output encoding_ (i.e. the
encoding used for form submission and error handling in the query string
of URLs) is UTF-8, so you get an UTF-8 encoder if you request an encoder
for them.

Additionally, the Encoding Standard factors BOM handling into wrapper
algorithms so that BOM handling isn’t part of the definition of the
encodings themselves. The Unicode _encoding schemes_ in the Unicode
Standard define BOM handling or lack thereof as part of the encoding
scheme.

When used with the `_without_bom_handling` entry points, the UTF-16LE
and UTF-16BE _encodings_ match the same-named _encoding schemes_ from
the Unicode Standard.

When used with the `_with_bom_removal` entry points, the UTF-8
_encoding_ matches the UTF-8 _encoding scheme_ from the Unicode
Standard.

This crate does not provide a mode that matches the UTF-16 _encoding
scheme_ from the Unicode Stardard. The UTF-16BE encoding used with
the entry points without `_bom_` qualifiers is the closest match,
but in that case, the UTF-8 BOM triggers UTF-8 decoding, which is
not part of the behavior of the UTF-16 _encoding scheme_ per the
Unicode Standard.

The UTF-32 family of Unicode encoding schemes is not supported
by this crate. The Encoding Standard doesn’t define any UTF-32
family encodings, since they aren’t necessary for consuming Web
content.

While gb18030 is capable of representing U+FEFF, the Encoding
Standard does not treat the gb18030 byte representation of U+FEFF
as a BOM, so neither does this crate.

## ISO-8859-1

ISO-8859-1 does not exist as a distinct encoding from windows-1252 in
the Encoding Standard. Therefore, an encoding that maps the unsigned
byte value to the same Unicode scalar value is not available via
`Encoding` in this crate.

However, the functions whose name starts with `convert` and contains
`latin1` in the `mem` module support such conversions, which are known as
[_isomorphic decode_](https://infra.spec.whatwg.org/#isomorphic-decode)
and [_isomorphic encode_](https://infra.spec.whatwg.org/#isomorphic-encode)
in the [Infra Standard](https://infra.spec.whatwg.org/).

## Web / Browser Focus

Both in terms of scope and performance, the focus is on the Web. For scope,
this means that encoding_rs implements the Encoding Standard fully and
doesn’t implement encodings that are not specified in the Encoding
Standard. For performance, this means that decoding performance is
important as well as performance for encoding into UTF-8 or encoding the
Basic Latin range (ASCII) into legacy encodings. Non-Basic Latin needs to
be encoded into legacy encodings in only two places in the Web platform: in
the query part of URLs, in which case it’s a matter of relatively rare
error handling, and in form submission, in which case the user action and
networking tend to hide the performance of the encoder.

Deemphasizing performance of encoding non-Basic Latin text into legacy
encodings enables smaller code size thanks to the encoder side using the
decode-optimized data tables without having encode-optimized data tables at
all. Even in decoders, smaller lookup table size is preferred over avoiding
multiplication operations.

Additionally, performance is a non-goal for the ASCII-incompatible
ISO-2022-JP encoding, which are rarely used on the Web. Instead of
performance, the decoder for ISO-2022-JP optimizes for ease/clarity
of implementation.

Despite the browser focus, the hope is that non-browser applications
that wish to consume Web content or submit Web forms in a Web-compatible
way will find encoding_rs useful. While encoding_rs does not try to match
Windows behavior, many of the encodings are close enough to legacy
encodings implemented by Windows that applications that need to consume
data in legacy Windows encodins may find encoding_rs useful. The
[codepage](https://crates.io/crates/codepage) crate maps from Windows
code page identifiers onto encoding_rs `Encoding`s and vice versa.

For decoding email, UTF-7 support is needed (unfortunately) in additition
to the encodings defined in the Encoding Standard. The
[charset](https://crates.io/crates/charset) wraps encoding_rs and adds
UTF-7 decoding for email purposes.

For single-byte DOS encodings beyond the ones supported by the Encoding
Standard, there is the [`oem_cp`](https://crates.io/crates/oem_cp) crate.

# Preparing Text for the Encoders

Normalizing text into Unicode Normalization Form C prior to encoding text
into a legacy encoding minimizes unmappable characters. Text can be
normalized to Unicode Normalization Form C using the
[`icu_normalizer`](https://crates.io/crates/icu_normalizer) crate, which
is part of [ICU4X](https://icu4x.unicode.org/).

The exception is windows-1258, which after normalizing to Unicode
Normalization Form C requires tone marks to be decomposed in order to
minimize unmappable characters. Vietnamese tone marks can be decomposed
using the [`detone`](https://crates.io/crates/detone) crate.

# Streaming & Non-Streaming; Rust & C/C++

The API in Rust has two modes of operation: streaming and non-streaming.
The streaming API is the foundation of the implementation and should be
used when processing data that arrives piecemeal from an i/o stream. The
streaming API has an FFI wrapper (as a [separate crate][1]) that exposes it
to C callers. The non-streaming part of the API is for Rust callers only and
is smart about borrowing instead of copying when possible. When
streamability is not needed, the non-streaming API should be preferrer in
order to avoid copying data when a borrow suffices.

There is no analogous C API exposed via FFI, mainly because C doesn’t have
standard types for growable byte buffers and Unicode strings that know
their length.

The C API (header file generated at `target/include/encoding_rs.h` when
building encoding_rs) can, in turn, be wrapped for use from C++. Such a
C++ wrapper can re-create the non-streaming API in C++ for C++ callers.
The C binding comes with a [C++17 wrapper][2] that uses standard library +
[GSL][3] types and that recreates the non-streaming API in C++ on top of
the streaming API. A C++ wrapper with XPCOM/MFBT types is available as
[`mozilla::Encoding`][4].

The `Encoding` type is common to both the streaming and non-streaming
modes. In the streaming mode, decoding operations are performed with a
`Decoder` and encoding operations with an `Encoder` object obtained via
`Encoding`. In the non-streaming mode, decoding and encoding operations are
performed using methods on `Encoding` objects themselves, so the `Decoder`
and `Encoder` objects are not used at all.

[1]: https://github.com/hsivonen/encoding_c
[2]: https://github.com/hsivonen/encoding_c/blob/master/include/encoding_rs_cpp.h
[3]: https://github.com/Microsoft/GSL/
[4]: https://searchfox.org/mozilla-central/source/intl/Encoding.h

# Memory management

The non-streaming mode never performs heap allocations (even the methods
that write into a `Vec<u8>` or a `String` by taking them as arguments do
not reallocate the backing buffer of the `Vec<u8>` or the `String`). That
is, the non-streaming mode uses caller-allocated buffers exclusively.

The methods of the streaming mode that return a `Vec<u8>` or a `String`
perform heap allocations but only to allocate the backing buffer of the
`Vec<u8>` or the `String`.

`Encoding` is always statically allocated. `Decoder` and `Encoder` need no
`Drop` cleanup.

# Buffer reading and writing behavior

Based on experience gained with the `java.nio.charset` encoding converter
API and with the Gecko uconv encoding converter API, the buffer reading
and writing behaviors of encoding_rs are asymmetric: input buffers are
fully drained but output buffers are not always fully filled.

When reading from an input buffer, encoding_rs always consumes all input
up to the next error or to the end of the buffer. In particular, when
decoding, even if the input buffer ends in the middle of a byte sequence
for a character, the decoder consumes all input. This has the benefit that
the caller of the API can always fill the next buffer from the start from
whatever source the bytes come from and never has to first copy the last
bytes of the previous buffer to the start of the next buffer. However, when
encoding, the UTF-8 input buffers have to end at a character boundary, which
is a requirement for the Rust `str` type anyway, and UTF-16 input buffer
boundaries falling in the middle of a surrogate pair result in both
suggorates being treated individually as unpaired surrogates.

Additionally, decoders guarantee that they can be fed even one byte at a
time and encoders guarantee that they can be fed even one code point at a
time. This has the benefit of not placing restrictions on the size of
chunks the content arrives e.g. from network.

When writing into an output buffer, encoding_rs makes sure that the code
unit sequence for a character is never split across output buffer
boundaries. This may result in wasted space at the end of an output buffer,
but the advantages are that the output side of both decoders and encoders
is greatly simplified compared to designs that attempt to fill output
buffers exactly even when that entails splitting a code unit sequence and
when encoding_rs methods return to the caller, the output produces thus
far is always valid taken as whole. (In the case of encoding to ISO-2022-JP,
the output needs to be considered as a whole, because the latest output
buffer taken alone might not be valid taken alone if the transition away
from the ASCII state occurred in an earlier output buffer. However, since
the ISO-2022-JP decoder doesn’t treat streams that don’t end in the ASCII
state as being in error despite the encoder generating a transition to the
ASCII state at the end, the claim about the partial output taken as a whole
being valid is true even for ISO-2022-JP.)

# Error Reporting

Based on experience gained with the `java.nio.charset` encoding converter
API and with the Gecko uconv encoding converter API, the error reporting
behaviors of encoding_rs are asymmetric: decoder errors include offsets
that leave it up to the caller to extract the erroneous bytes from the
input stream if the caller wishes to do so but encoder errors provide the
code point associated with the error without requiring the caller to
extract it from the input on its own.

On the encoder side, an error is always triggered by the most recently
pushed Unicode scalar, which makes it simple to pass the `char` to the
caller. Also, it’s very typical for the caller to wish to do something with
this data: generate a numeric escape for the character. Additionally, the
ISO-2022-JP encoder reports U+FFFD instead of the actual input character in
certain cases, so requiring the caller to extract the character from the
input buffer would require the caller to handle ISO-2022-JP details.
Furthermore, requiring the caller to extract the character from the input
buffer would require the caller to implement UTF-8 or UTF-16 math, which is
the job of an encoding conversion library.

On the decoder side, errors are triggered in more complex ways. For
example, when decoding the sequence ESC, ‘$’, _buffer boundary_, ‘A’ as
ISO-2022-JP, the ESC byte is in error, but this is discovered only after
the buffer boundary when processing ‘A’. Thus, the bytes in error might not
be the ones most recently pushed to the decoder and the error might not even
be in the current buffer.

Some encoding conversion APIs address the problem by not acknowledging
trailing bytes of an input buffer as consumed if it’s still possible for
future bytes to cause the trailing bytes to be in error. This way, error
reporting can always refer to the most recently pushed buffer. This has the
problem that the caller of the API has to copy the unconsumed trailing
bytes to the start of the next buffer before being able to fill the rest
of the next buffer. This is annoying, error-prone and inefficient.

A possible solution would be making the decoder remember recently consumed
bytes in order to be able to include a copy of the erroneous bytes when
reporting an error. This has two problem: First, callers a rarely
interested in the erroneous bytes, so attempts to identify them are most
often just overhead anyway. Second, the rare applications that are
interested typically care about the location of the error in the input
stream.

To keep the API convenient for common uses and the overhead low while making
it possible to develop applications, such as HTML validators, that care
about which bytes were in error, encoding_rs reports the length of the
erroneous sequence and the number of bytes consumed after the erroneous
sequence. As long as the caller doesn’t discard the 6 most recent bytes,
this makes it possible for callers that care about the erroneous bytes to
locate them.

# No Convenience API for Custom Replacements

The Web Platform and, therefore, the Encoding Standard supports only one
error recovery mode for decoders and only one error recovery mode for
encoders. The supported error recovery mode for decoders is emitting the
REPLACEMENT CHARACTER on error. The supported error recovery mode for
encoders is emitting an HTML decimal numeric character reference for
unmappable characters.

Since encoding_rs is Web-focused, these are the only error recovery modes
for which convenient support is provided. Moreover, on the decoder side,
there aren’t really good alternatives for emitting the REPLACEMENT CHARACTER
on error (other than treating errors as fatal). In particular, simply
ignoring errors is a
[security problem](http://www.unicode.org/reports/tr36/#Substituting_for_Ill_Formed_Subsequences),
so it would be a bad idea for encoding_rs to provide a mode that encouraged
callers to ignore errors.

On the encoder side, there are plausible alternatives for HTML decimal
numeric character references. For example, when outputting CSS, CSS-style
escapes would seem to make sense. However, instead of facilitating the
output of CSS, JS, etc. in non-UTF-8 encodings, encoding_rs takes the design
position that you shouldn’t generate output in encodings other than UTF-8,
except where backward compatibility with interacting with the legacy Web
requires it. The legacy Web requires it only when parsing the query strings
of URLs and when submitting forms, and those two both use HTML decimal
numeric character references.

While encoding_rs doesn’t make encoder replacements other than HTML decimal
numeric character references easy, it does make them _possible_.
`encode_from_utf8()`, which emits HTML decimal numeric character references
for unmappable characters, is implemented on top of
`encode_from_utf8_without_replacement()`. Applications that really, really
want other replacement schemes for unmappable characters can likewise
implement them on top of `encode_from_utf8_without_replacement()`.

# No Extensibility by Design

The set of encodings supported by encoding_rs is not extensible by design.
That is, `Encoding`, `Decoder` and `Encoder` are intentionally `struct`s
rather than `trait`s. encoding_rs takes the design position that all future
text interchange should be done using UTF-8, which can represent all of
Unicode. (It is, in fact, the only encoding supported by the Encoding
Standard and encoding_rs that can represent all of Unicode and that has
encoder support. UTF-16LE and UTF-16BE don’t have encoder support, and
gb18030 cannot encode U+E5E5.) The other encodings are supported merely for
legacy compatibility and not due to non-UTF-8 encodings having benefits
other than being able to consume legacy content.

Considering that UTF-8 can represent all of Unicode and is already supported
by all Web browsers, introducing a new encoding wouldn’t add to the
expressiveness but would add to compatibility problems. In that sense,
adding new encodings to the Web Platform doesn’t make sense, and, in fact,
post-UTF-8 attempts at encodings, such as BOCU-1, have been rejected from
the Web Platform. On the other hand, the set of legacy encodings that must
be supported for a Web browser to be able to be successful is not going to
expand. Empirically, the set of encodings specified in the Encoding Standard
is already sufficient and the set of legacy encodings won’t grow
retroactively.

Since extensibility doesn’t make sense considering the Web focus of
encoding_rs and adding encodings to Web clients would be actively harmful,
it makes sense to make the set of encodings that encoding_rs supports
non-extensible and to take the (admittedly small) benefits arising from
that, such as the size of `Decoder` and `Encoder` objects being known ahead
 of time, which enables stack allocation thereof.

This does have downsides for applications that might want to put encoding_rs
to non-Web uses if those non-Web uses involve legacy encodings that aren’t
needed for Web uses. The needs of such applications should not complicate
encoding_rs itself, though. It is up to those applications to provide a
framework that delegates the operations with encodings that encoding_rs
supports to encoding_rs and operations with other encodings to something
else (as opposed to encoding_rs itself providing an extensibility
framework).

# Panics

Methods in encoding_rs can panic if the API is used against the requirements
stated in the documentation, if a state that’s supposed to be impossible
is reached due to an internal bug or on integer overflow. When used
according to documentation with buffer sizes that stay below integer
overflow, in the absence of internal bugs, encoding_rs does not panic.

Panics arising from API misuse aren’t documented beyond this on individual
methods.

# At-Risk Parts of the API

The foreseeable source of partially backward-incompatible API change is the
way the instances of `Encoding` are made available.

If Rust changes to allow the entries of `[&'static Encoding; N]` to be
initialized with `static`s of type `&'static Encoding`, the non-reference
`FOO_INIT` public `Encoding` instances will be removed from the public API.

If Rust changes to make the referent of `pub const FOO: &'static Encoding`
unique when the constant is used in different crates, the reference-typed
`static`s for the encoding instances will be changed from `static` to
`const` and the non-reference-typed `_INIT` instances will be removed.

# Mapping Spec Concepts onto the API

<table>
<thead>
<tr><th>Spec Concept</th><th>Streaming</th><th>Non-Streaming</th></tr>
</thead>
<tbody>
<tr><td><a href="https://encoding.spec.whatwg.org/#encoding">encoding</a></td><td><code>&amp;'static Encoding</code></td><td><code>&amp;'static Encoding</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8">UTF-8 encoding</a></td><td><code>UTF_8</code></td><td><code>UTF_8</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#concept-encoding-get">get an encoding</a></td><td><code>Encoding::for_label(<var>label</var>)</code></td><td><code>Encoding::for_label(<var>label</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#name">name</a></td><td><code><var>encoding</var>.name()</code></td><td><code><var>encoding</var>.name()</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#get-an-output-encoding">get an output encoding</a></td><td><code><var>encoding</var>.output_encoding()</code></td><td><code><var>encoding</var>.output_encoding()</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#decode">decode</a></td><td><code>let d = <var>encoding</var>.new_decoder();<br>let res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code><var>encoding</var>.decode(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-decode">UTF-8 decode</a></td><td><code>let d = UTF_8.new_decoder_with_bom_removal();<br>let res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code>UTF_8.decode_with_bom_removal(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-decode-without-bom">UTF-8 decode without BOM</a></td><td><code>let d = UTF_8.new_decoder_without_bom_handling();<br>let res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = d.decode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code>UTF_8.decode_without_bom_handling(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-decode-without-bom-or-fail">UTF-8 decode without BOM or fail</a></td><td><code>let d = UTF_8.new_decoder_without_bom_handling();<br>let res = d.decode_to_<var>*</var>_without_replacement(<var>src</var>, <var>dst</var>, false);<br>// &hellip; (fail if malformed)</br>let last_res = d.decode_to_<var>*</var>_without_replacement(<var>src</var>, <var>dst</var>, true);<br>// (fail if malformed)</code></td><td><code>UTF_8.decode_without_bom_handling_and_without_replacement(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#encode">encode</a></td><td><code>let e = <var>encoding</var>.new_encoder();<br>let res = e.encode_to_<var>*</var>(<var>src</var>, <var>dst</var>, false);<br>// &hellip;</br>let last_res = e.encode_to_<var>*</var>(<var>src</var>, <var>dst</var>, true);</code></td><td><code><var>encoding</var>.encode(<var>src</var>)</code></td></tr>
<tr><td><a href="https://encoding.spec.whatwg.org/#utf-8-encode">UTF-8 encode</a></td><td>Use the UTF-8 nature of Rust strings directly:<br><code><var>write</var>(<var>src</var>.as_bytes());<br>// refill src<br><var>write</var>(<var>src</var>.as_bytes());<br>// refill src<br><var>write</var>(<var>src</var>.as_bytes());<br>// &hellip;</code></td><td>Use the UTF-8 nature of Rust strings directly:<br><code><var>src</var>.as_bytes()</code></td></tr>
</tbody>
</table>

# Compatibility with the rust-encoding API

The crate
[encoding_rs_compat](https://github.com/hsivonen/encoding_rs_compat/)
is a drop-in replacement for rust-encoding 0.2.32 that implements (most of)
the API of rust-encoding 0.2.32 on top of encoding_rs.

# Mapping rust-encoding concepts to encoding_rs concepts

The following table provides a mapping from rust-encoding constructs to
encoding_rs ones.

<table>
<thead>
<tr><th>rust-encoding</th><th>encoding_rs</th></tr>
</thead>
<tbody>
<tr><td><code>encoding::EncodingRef</code></td><td><code>&amp;'static encoding_rs::Encoding</code></td></tr>
<tr><td><code>encoding::all::<var>WINDOWS_31J</var></code> (not based on the WHATWG name for some encodings)</td><td><code>encoding_rs::<var>SHIFT_JIS</var></code> (always the WHATWG name uppercased and hyphens replaced with underscores)</td></tr>
<tr><td><code>encoding::all::ERROR</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::all::ASCII</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::all::ISO_8859_1</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::all::HZ</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::label::encoding_from_whatwg_label(<var>string</var>)</code></td><td><code>encoding_rs::Encoding::for_label(<var>string</var>)</code></td></tr>
<tr><td><code><var>enc</var>.whatwg_name()</code> (always lower case)</td><td><code><var>enc</var>.name()</code> (potentially mixed case)</td></tr>
<tr><td><code><var>enc</var>.name()</code></td><td>Not available because not in the Encoding Standard</td></tr>
<tr><td><code>encoding::decode(<var>bytes</var>, encoding::DecoderTrap::Replace, <var>enc</var>)</code></td><td><code><var>enc</var>.decode(<var>bytes</var>)</code></td></tr>
<tr><td><code><var>enc</var>.decode(<var>bytes</var>, encoding::DecoderTrap::Replace)</code></td><td><code><var>enc</var>.decode_without_bom_handling(<var>bytes</var>)</code></td></tr>
<tr><td><code><var>enc</var>.encode(<var>string</var>, encoding::EncoderTrap::NcrEscape)</code></td><td><code><var>enc</var>.encode(<var>string</var>)</code></td></tr>
<tr><td><code><var>enc</var>.raw_decoder()</code></td><td><code><var>enc</var>.new_decoder_without_bom_handling()</code></td></tr>
<tr><td><code><var>enc</var>.raw_encoder()</code></td><td><code><var>enc</var>.new_encoder()</code></td></tr>
<tr><td><code>encoding::RawDecoder</code></td><td><code>encoding_rs::Decoder</code></td></tr>
<tr><td><code>encoding::RawEncoder</code></td><td><code>encoding_rs::Encoder</code></td></tr>
<tr><td><code><var>raw_decoder</var>.raw_feed(<var>src</var>, <var>dst_string</var>)</code></td><td><code><var>dst_string</var>.reserve(<var>decoder</var>.max_utf8_buffer_length_without_replacement(<var>src</var>.len()));<br><var>decoder</var>.decode_to_string_without_replacement(<var>src</var>, <var>dst_string</var>, false)</code></td></tr>
<tr><td><code><var>raw_encoder</var>.raw_feed(<var>src</var>, <var>dst_vec</var>)</code></td><td><code><var>dst_vec</var>.reserve(<var>encoder</var>.max_buffer_length_from_utf8_without_replacement(<var>src</var>.len()));<br><var>encoder</var>.encode_from_utf8_to_vec_without_replacement(<var>src</var>, <var>dst_vec</var>, false)</code></td></tr>
<tr><td><code><var>raw_decoder</var>.raw_finish(<var>dst</var>)</code></td><td><code><var>dst_string</var>.reserve(<var>decoder</var>.max_utf8_buffer_length_without_replacement(0));<br><var>decoder</var>.decode_to_string_without_replacement(b"", <var>dst</var>, true)</code></td></tr>
<tr><td><code><var>raw_encoder</var>.raw_finish(<var>dst</var>)</code></td><td><code><var>dst_vec</var>.reserve(<var>encoder</var>.max_buffer_length_from_utf8_without_replacement(0));<br><var>encoder</var>.encode_from_utf8_to_vec_without_replacement("", <var>dst</var>, true)</code></td></tr>
<tr><td><code>encoding::DecoderTrap::Strict</code></td><td><code>decode*</code> methods that have <code>_without_replacement</code> in their name (and treating the `Malformed` result as fatal).</td></tr>
<tr><td><code>encoding::DecoderTrap::Replace</code></td><td><code>decode*</code> methods that <i>do not</i> have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::DecoderTrap::Ignore</code></td><td>It is a bad idea to ignore errors due to security issues, but this could be implemented using <code>decode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::DecoderTrap::Call(DecoderTrapFunc)</code></td><td>Can be implemented using <code>decode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::Strict</code></td><td><code>encode*</code> methods that have <code>_without_replacement</code> in their name (and treating the `Unmappable` result as fatal).</td></tr>
<tr><td><code>encoding::EncoderTrap::Replace</code></td><td>Can be implemented using <code>encode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::Ignore</code></td><td>It is a bad idea to ignore errors due to security issues, but this could be implemented using <code>encode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::NcrEscape</code></td><td><code>encode*</code> methods that <i>do not</i> have <code>_without_replacement</code> in their name.</td></tr>
<tr><td><code>encoding::EncoderTrap::Call(EncoderTrapFunc)</code></td><td>Can be implemented using <code>encode*</code> methods that have <code>_without_replacement</code> in their name.</td></tr>
</tbody>
</table>

# Relationship with Windows Code Pages

Despite the Web and browser focus, the encodings defined by the Encoding
Standard and implemented by this crate may be useful for decoding legacy
data that uses Windows code pages. The following table names the single-byte
encodings
that have a closely related Windows code page, the number of the closest
code page, a column indicating whether Windows maps unassigned code points
to the Unicode Private Use Area instead of U+FFFD and a remark number
indicating remarks in the list after the table.

<table>
<thead>
<tr><th>Encoding</th><th>Code Page</th><th>PUA</th><th>Remarks</th></tr>
</thead>
<tbody>
<tr><td>Shift_JIS</td><td>932</td><td></td><td></td></tr>
<tr><td>GBK</td><td>936</td><td></td><td></td></tr>
<tr><td>EUC-KR</td><td>949</td><td></td><td></td></tr>
<tr><td>Big5</td><td>950</td><td></td><td></td></tr>
<tr><td>IBM866</td><td>866</td><td></td><td></td></tr>
<tr><td>windows-874</td><td>874</td><td>&bullet;</td><td></td></tr>
<tr><td>UTF-16LE</td><td>1200</td><td></td><td></td></tr>
<tr><td>UTF-16BE</td><td>1201</td><td></td><td></td></tr>
<tr><td>windows-1250</td><td>1250</td><td></td><td></td></tr>
<tr><td>windows-1251</td><td>1251</td><td></td><td></td></tr>
<tr><td>windows-1252</td><td>1252</td><td></td><td></td></tr>
<tr><td>windows-1253</td><td>1253</td><td>&bullet;</td><td></td></tr>
<tr><td>windows-1254</td><td>1254</td><td></td><td></td></tr>
<tr><td>windows-1255</td><td>1255</td><td>&bullet;</td><td></td></tr>
<tr><td>windows-1256</td><td>1256</td><td></td><td></td></tr>
<tr><td>windows-1257</td><td>1257</td><td>&bullet;</td><td></td></tr>
<tr><td>windows-1258</td><td>1258</td><td></td><td></td></tr>
<tr><td>macintosh</td><td>10000</td><td></td><td>1</td></tr>
<tr><td>x-mac-cyrillic</td><td>10017</td><td></td><td>2</td></tr>
<tr><td>KOI8-R</td><td>20866</td><td></td><td></td></tr>
<tr><td>EUC-JP</td><td>20932</td><td></td><td></td></tr>
<tr><td>KOI8-U</td><td>21866</td><td></td><td></td></tr>
<tr><td>ISO-8859-2</td><td>28592</td><td></td><td></td></tr>
<tr><td>ISO-8859-3</td><td>28593</td><td></td><td></td></tr>
<tr><td>ISO-8859-4</td><td>28594</td><td></td><td></td></tr>
<tr><td>ISO-8859-5</td><td>28595</td><td></td><td></td></tr>
<tr><td>ISO-8859-6</td><td>28596</td><td>&bullet;</td><td></td></tr>
<tr><td>ISO-8859-7</td><td>28597</td><td>&bullet;</td><td>3</td></tr>
<tr><td>ISO-8859-8</td><td>28598</td><td>&bullet;</td><td>4</td></tr>
<tr><td>ISO-8859-13</td><td>28603</td><td>&bullet;</td><td></td></tr>
<tr><td>ISO-8859-15</td><td>28605</td><td></td><td></td></tr>
<tr><td>ISO-8859-8-I</td><td>38598</td><td></td><td>5</td></tr>
<tr><td>ISO-2022-JP</td><td>50220</td><td></td><td></td></tr>
<tr><td>gb18030</td><td>54936</td><td></td><td></td></tr>
<tr><td>UTF-8</td><td>65001</td><td></td><td></td></tr>
</tbody>
</table>

1. Windows decodes 0xBD to U+2126 OHM SIGN instead of U+03A9 GREEK CAPITAL LETTER OMEGA.
2. Windows decodes 0xFF to U+00A4 CURRENCY SIGN instead of U+20AC EURO SIGN.
3. Windows decodes the currency signs at 0xA4 and 0xA5 as well as 0xAA,
   which should be U+037A GREEK YPOGEGRAMMENI, to PUA code points. Windows
   decodes 0xA1 to U+02BD MODIFIER LETTER REVERSED COMMA instead of U+2018
   LEFT SINGLE QUOTATION MARK and 0xA2 to U+02BC MODIFIER LETTER APOSTROPHE
   instead of U+2019 RIGHT SINGLE QUOTATION MARK.
4. Windows decodes 0xAF to OVERLINE instead of MACRON and 0xFE and 0xFD to PUA instead
   of LRM and RLM.
5. Remarks from the previous item apply.

The differences between this crate and Windows in the case of multibyte encodings
are not yet fully documented here. The lack of remarks above should not be taken
as indication of lack of differences.

# Notable Differences from IANA Naming

In some cases, the Encoding Standard specifies the popular unextended encoding
name where in IANA terms one of the other labels would be more precise considering
the extensions that the Encoding Standard has unified into the encoding.

<table>
<thead>
<tr><th>Encoding</th><th>IANA</th></tr>
</thead>
<tbody>
<tr><td>Big5</td><td>Big5-HKSCS</td></tr>
<tr><td>EUC-KR</td><td>windows-949</td></tr>
<tr><td>Shift_JIS</td><td>windows-31j</td></tr>
<tr><td>x-mac-cyrillic</td><td>x-mac-ukrainian</td></tr>
</tbody>
</table>

In other cases where the Encoding Standard unifies unextended and extended
variants of an encoding, the encoding gets the name of the extended
variant.

<table>
<thead>
<tr><th>IANA</th><th>Unified into Encoding</th></tr>
</thead>
<tbody>
<tr><td>ISO-8859-1</td><td>windows-1252</td></tr>
<tr><td>ISO-8859-9</td><td>windows-1254</td></tr>
<tr><td>TIS-620</td><td>windows-874</td></tr>
</tbody>
</table>

See the section [_UTF-16LE, UTF-16BE and Unicode Encoding Schemes_](#utf-16le-utf-16be-and-unicode-encoding-schemes)
for discussion about the UTF-16 family.