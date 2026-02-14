This crate provides a push based XML parser library that
adheres to XML5 specification. In other words this library
trades well-formedness for error recovery.

The idea behind this, was to minimize number of errors from
tools that generate XML (e.g. `&#83` won’t just return `&#83`
as text, but will parse it into `S` ).
You can check out full specification [here](https://ygg01.github.io/xml5_draft/).

What this library provides is a solid XML parser that can:

  * Parse somewhat erroneous XML input
  * Provide support for [Numeric character references](https://en.wikipedia.org/wiki/Numeric_character_reference).
  * Provide partial [XML namespace](http://www.w3.org/TR/xml-names11/) support.
  * Provide full set of SVG/MathML entities

What isn’t in scope for this library:

  * Document Type Definition parsing - this is pretty hard to do right and nowadays, its used
