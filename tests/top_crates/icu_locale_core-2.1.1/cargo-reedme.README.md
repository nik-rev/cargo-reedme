Parsing, manipulating, and serializing Unicode Language and Locale Identifiers.

This module is published as its own crate ([`icu_locale_core`](https://docs.rs/icu_locale_core/latest/icu_locale_core/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

The module provides algorithms for parsing a string into a well-formed language or locale identifier
as defined by [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]. Additionally
the module provides [`preferences`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/preferences/) interface for operations on locale preferences and conversions
from and to locale unicode extensions.

[`Locale`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/locale/struct.Locale.html) is the most common structure to use for storing information about a language,
script, region, variants and extensions. In almost all cases, this struct should be used as the
base unit for all locale management operations.

[`LanguageIdentifier`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/langid/struct.LanguageIdentifier.html) is a strict subset of [`Locale`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/locale/struct.Locale.html) which can be useful in a narrow range of
cases where [`Unicode Extensions`] are not relevant.

If in doubt, use [`Locale`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/locale/struct.Locale.html).

# Examples

```rust
use icu::locale::Locale;
use icu::locale::{
    locale,
    subtags::{language, region},
};

let mut loc: Locale = locale!("en-US");

assert_eq!(loc.id.language, language!("en"));
assert_eq!(loc.id.script, None);
assert_eq!(loc.id.region, Some(region!("US")));
assert_eq!(loc.id.variants.len(), 0);

loc.id.region = Some(region!("GB"));

assert_eq!(loc, locale!("en-GB"));
```

For more details, see [`Locale`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/locale/struct.Locale.html) and [`LanguageIdentifier`](https://docs.rs/icu_locale_core/2.1.1/icu_locale_core/langid/struct.LanguageIdentifier.html).

[`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
[`ICU4X`]: ../icu/index.html
[`Unicode Extensions`]: extensions