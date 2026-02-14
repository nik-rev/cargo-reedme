Convert to and from different string cases.

# Basic Usage

The most common use of this crate is to just convert a string into a
particular case, like snake, camel, or kebab.  You can use the [`ccase`](https://docs.rs/convert_case/0.10.0/convert_case/macro.ccase.html)
macro to convert most string types into the new case.
```rust
use convert_case::ccase;

let s = "myVarName";
assert_eq!(ccase!(snake, s),  "my_var_name");
assert_eq!(ccase!(kebab, s),  "my-var-name");
assert_eq!(ccase!(pascal, s), "MyVarName");
assert_eq!(ccase!(title, s),  "My Var Name");
```

For more explicit conversion, import the [`Casing`](https://docs.rs/convert_case/0.10.0/convert_case/trait.Casing.html) trait which adds methods
to string types that perform the conversion based on a variant of the [`Case`](https://docs.rs/convert_case/0.10.0/convert_case/case/enum.Case.html) enum.
```rust
use convert_case::{Case, Casing};

let s = "myVarName";
assert_eq!(s.to_case(Case::Snake),  "my_var_name");
assert_eq!(s.to_case(Case::Kebab),  "my-var-name");
assert_eq!(s.to_case(Case::Pascal), "MyVarName");
assert_eq!(s.to_case(Case::Title),  "My Var Name");
```

For a full list of cases, see [`Case`](https://docs.rs/convert_case/0.10.0/convert_case/case/enum.Case.html).

# Splitting Conditions

Case conversion starts by splitting a single identifier into a list of words.  The
condition for when to split and how to perform the split is defined by a [`Boundary`](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html).

By default, [`ccase`](https://docs.rs/convert_case/0.10.0/convert_case/macro.ccase.html) and [`Casing::to_case`](https://docs.rs/convert_case/0.10.0/convert_case/trait.Casing.html#tymethod.to_case) will split identifiers at all locations
based on a list of [default boundaries](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html#method.defaults).

```rust
use convert_case::ccase;

assert_eq!(ccase!(pascal, "hyphens-and_underscores"), "HyphensAndUnderscores");
assert_eq!(ccase!(pascal, "lowerUpper space"), "LowerUpperSpace");
assert_eq!(ccase!(snake, "HTTPRequest"), "http_request");
assert_eq!(ccase!(snake, "vector4d"), "vector_4_d")
```

Associated with each case is a [list of boundaries](https://docs.rs/convert_case/0.10.0/convert_case/case/enum.Case.html#method.boundaries) that can be
used to split identifiers instead of the defaults.  We can use the following notation
with the [`ccase`](https://docs.rs/convert_case/0.10.0/convert_case/macro.ccase.html) macro.
```rust
use convert_case::ccase;

assert_eq!(
    ccase!(title, "1999-25-01_family_photo.png"),
    "1999 25 01 Family Photo.png",
);
assert_eq!(
    ccase!(snake -> title, "1999-25-01_family_photo.png"),
    "1999-25-01 Family Photo.png",
);
```
Or we can use the [`from_case`](https://docs.rs/convert_case/0.10.0/convert_case/trait.Casing.html#tymethod.from_case) method on `Casing` before calling
`to_case`.
```rust
use convert_case::{Case, Casing};

assert_eq!(
    "John McCarthy".to_case(Case::Snake),
    "john_mc_carthy",
);
assert_eq!(
    "John McCarthy".from_case(Case::Title).to_case(Case::Snake),
    "john_mccarthy",
);
```
You can remove boundaries from the list of defaults with [`Casing::remove_boundaries`](https://docs.rs/convert_case/0.10.0/convert_case/trait.Casing.html#tymethod.remove_boundaries).  See
the list of constants on [`Boundary`](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html) for splitting conditions.
```rust
use convert_case::{Boundary, Case, Casing};

assert_eq!(
    "Vector4D".remove_boundaries(&[Boundary::DigitUpper]).to_case(Case::Snake),
    "vector_4d",
);
```

# Other Behavior

### Acronyms
Part of the default list of boundaries is [`acronym`](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html#variant.Acronym) which
will detect two capital letters followed by a lowercase letter.  But there is no memory
that the word itself was parsed considered an acronym.
```rust
assert_eq!(ccase!(snake, "HTTPRequest"), "http_request");
assert_eq!(ccase!(pascal, "HTTPRequest"), "HttpRequest");
```

### Digits
The default list of boundaries includes splitting before and after digits.
```rust
assert_eq!(ccase!(title, "word2vec"), "Word 2 Vec");
```

### Unicode
Conversion works on _graphemes_ as defined by the
[`unicode_segmentation`](unicode_segmentation::UnicodeSegmentation::graphemes) library.
This means that transforming letters to lowercase or uppercase works on all unicode
characters, which also means that the number of characters isn’t necessarily the
same after conversion.
```rust
assert_eq!(ccase!(kebab, "GranatÄpfel"), "granat-äpfel");
assert_eq!(ccase!(title, "ПЕРСПЕКТИВА24"), "Перспектива 24");
assert_eq!(ccase!(lower, "ὈΔΥΣΣΕΎΣ"), "ὀδυσσεύς");
```

### Symbols
All symbols that are not part of the default boundary conditions are ignored.  This
is any symbol that isn’t an underscore, hyphen, or space.
```rust
assert_eq!(ccase!(snake, "dots.arent.default"), "dots.arent.default");
assert_eq!(ccase!(pascal, "path/to/file_name"), "Path/to/fileName");
assert_eq!(ccase!(pascal, "list\nof\nwords"),   "List\nof\nwords");
```

### Delimiters
Leading, trailing, and duplicate delimiters create empty words.
This propogates and the converted string will share the behavior.  **This can cause
unintuitive behavior for patterns that transform words based on index.**
```rust
assert_eq!(ccase!(constant, "_leading_score"), "_LEADING_SCORE");
assert_eq!(ccase!(ada, "trailing-dash-"), "Trailing_Dash_");
assert_eq!(ccase!(train, "duplicate----hyphens"), "Duplicate----Hyphens");

// not what you might expect!
assert_eq!(ccase!(camel, "_empty__first_word"), "EmptyFirstWord");
```

# Customizing Behavior

Case conversion takes place in three steps:
1. Splitting the identifier into a list of words
2. Mutating the letter case of graphemes within each word
3. Joining the words back into an identifier using a delimiter

Those are defined by boundaries, patterns, and delimiters respectively.  Graphically:

```md
Identifier        Identifier'
    |                 ^
    | boundaries      | delimiter
    V                 |
  Words ----------> Words'
          pattern
```

## Patterns

How to change the case of letters across a list of words is called a _pattern_.
A pattern is a function that when passed a `&[&str]`, produces a
`Vec<String>`.  The [`Pattern`](https://docs.rs/convert_case/0.10.0/convert_case/pattern/enum.Pattern.html) enum encapsulates the common transformations
used across all cases.  Although custom functions can be supplied with the
[`Custom`](https://docs.rs/convert_case/0.10.0/convert_case/pattern/enum.Pattern.html#variant.Custom) variant.

## Boundaries

The condition for splitting at part of an identifier, where to perform
the split, and if any characters are removed are defined by [boundaries](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html).
By default, identifiers are split based on [`Boundary::defaults`](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html#method.defaults).  This list
contains word boundaries that you would likely see after creating a multi-word
identifier of typical cases.

Custom boundary conditions can also be created.  Commonly, you might split based on some
character or list of characters.  The [`delim_boundary`](https://docs.rs/convert_case/0.10.0/convert_case/macro.delim_boundary.html) macro builds
a boundary that splits on the presence of a string, and then removes the string
while producing the list of words.

You can also use [`Boundary::Custom`](https://docs.rs/convert_case/0.10.0/convert_case/boundary/enum.Boundary.html#variant.Custom) to explicitly define boundary
conditions.  If you actually need to create a
boundary condition from scratch, you should file an issue to let the author know
how you used it.  I’m not certain what other boundary condition would be helpful.

## Cases

A case is defined by a list of boundaries, a pattern, and a _delimiter_: the string to
intersperse between words before concatenation. [`Case::Custom`](https://docs.rs/convert_case/0.10.0/convert_case/case/enum.Case.html#variant.Custom) is a struct enum variant with
exactly those three fields.  You could create your own case like so.
```rust
use convert_case::{Case, Casing, delim_boundary, Pattern};

let dot_case = Case::Custom {
    boundaries: &[delim_boundary!(".")],
    pattern: Pattern::Lowercase,
    delim: ".",
};

assert_eq!("AnimalFactoryFactory".to_case(dot_case), "animal.factory.factory");

assert_eq!(
    "pd.options.mode.copy_on_write"
        .from_case(dot_case)
        .to_case(Case::Title),
    "Pd Options Mode Copy_on_write",
)
```

## Converter

Case conversion with `convert_case` allows using attributes from two cases.  From
the first case is how you split the identifier (the _from_ case), and
from the second is how to mutate and join the words (the _to_ case.)  The
[`Converter`](https://docs.rs/convert_case/0.10.0/convert_case/converter/struct.Converter.html) is used to define the _conversion_ process, not a case directly.

It has the same fields as case, but is exposed via a builder interface
and can be used to apply a conversion on a string directly, without
specifying all the parameters at the time of conversion.

In the below example, we build a converter that maps the double colon
delimited module path in rust into a series of file directories.

```rust
use convert_case::{Case, Converter, delim_boundary};

let modules_into_path = Converter::new()
    .set_boundaries(&[delim_boundary!("::")])
    .set_delim("/");

assert_eq!(
    modules_into_path.convert("std::os::unix"),
    "std/os/unix",
);
```

# Associated Projects

## Rust library `convert_case_extras`

Some extra utilties for convert_case that don’t need to be in the main library.
You can read more here: [`convert_case_extras`](https://docs.rs/convert_case_extras).

## stringcase.org

While developing `convert_case`, the author became fascinated in the naming conventions
used for cases as well as different implementations for conversion.  On [stringcase.org](https://stringcase.org)
is documentation of the history of naming conventions, a catalogue of case conversion tools,
and a more rigorous definition of what it means to “convert the case of an identifier.”

## Command Line Utility `ccase`

`convert_case` was originally developed for the purposes of a command line utility
for converting the case of strings and filenames.  You can check out
[`ccase` on Github](https://github.com/rutrum/ccase).