`icu_provider` is one of the `ICU4X` components.

Unicode’s experience with ICU4X’s parent projects, ICU4C and ICU4J, led the team to realize
that data management is the most critical aspect of deploying internationalization, and that it requires
a high level of customization for the needs of the platform it is embedded in. As a result
ICU4X comes with a selection of providers that should allow for ICU4X to naturally fit into
different business and technological needs of customers.

`icu_provider` defines traits and structs for transmitting data through the ICU4X locale
data pipeline. The primary trait is [`DataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DataProvider.html). It is parameterized by a
[`DataMarker`](https://docs.rs/icu_provider/2.1.1/icu_provider/marker_full/trait.DataMarker.html), which is the type-system-level data identifier. [`DataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DataProvider.html) has a single method,
[`DataProvider::load`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DataProvider.html#tymethod.load), which transforms a [`DataRequest`](https://docs.rs/icu_provider/2.1.1/icu_provider/request/struct.DataRequest.html) into a [`DataResponse`](https://docs.rs/icu_provider/2.1.1/icu_provider/response/struct.DataResponse.html).

- [`DataRequest`](https://docs.rs/icu_provider/2.1.1/icu_provider/request/struct.DataRequest.html) contains selectors to choose a specific variant of the marker, such as a locale.
- [`DataResponse`](https://docs.rs/icu_provider/2.1.1/icu_provider/response/struct.DataResponse.html) contains the data if the request was successful.

The most common types required for this crate are included via the prelude:

```rust
use icu_provider::prelude::*;
```

## Dynamic Data Providers

If the type system cannot be leveraged to load data (such as when dynamically loading from I/O),
there’s another form of the [`DataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DataProvider.html): [`DynamicDataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DynamicDataProvider.html). While [`DataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DataProvider.html) is parametrized
on the type-system level by a [`DataMarker`](https://docs.rs/icu_provider/2.1.1/icu_provider/marker_full/trait.DataMarker.html) (which are distinct types implementing this trait),
[`DynamicDataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DynamicDataProvider.html)s are parametrized at runtime by a [`DataMarkerInfo`](https://docs.rs/icu_provider/2.1.1/icu_provider/marker_full/struct.DataMarkerInfo.html) struct, which essentially is the runtime
representation of the [`DataMarker`](https://docs.rs/icu_provider/2.1.1/icu_provider/marker_full/trait.DataMarker.html) type.

The [`DynamicDataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DynamicDataProvider.html) is still type-level parametrized by the type that it loads, and there are two
implementations that should be called out

- [`DynamicDataProvider<BufferMarker>`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DynamicDataProvider.html), a.k.a. [`BufferProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/buf/trait.BufferProvider.html) returns data as `[u8]` buffers.

### BufferProvider

These providers are able to return unstructured data typically represented as
[`serde`](https://docs.rs/serde/1.0.228/serde/)-serialized buffers. Users can call [`as_deserializing()`] to get an object
implementing [`DataProvider`](https://docs.rs/icu_provider/2.1.1/icu_provider/data_provider/trait.DataProvider.html) by invoking Serde Deserialize.

Examples of BufferProviders:

- [`FsDataProvider`] reads individual buffers from the filesystem.
- [`BlobDataProvider`] reads buffers from a large in-memory blob.

## Provider Adapters

ICU4X offers several built-in modules to combine providers in interesting ways.
These can be found in the [`icu_provider_adapters`] crate.

## Testing Provider

This crate also contains a concrete provider for demonstration purposes:

- [`HelloWorldProvider`] returns “hello world” strings in several languages.

## Types and Lifetimes

Types compatible with [`Yokeable`] can be passed through the data provider, so long as they are
associated with a marker type implementing [`DynamicDataMarker`](https://docs.rs/icu_provider/2.1.1/icu_provider/marker_full/trait.DynamicDataMarker.html).

Data structs should generally have one lifetime argument: `'data`. This lifetime allows data
structs to borrow zero-copy data.

[`FixedProvider`]: https://docs.rs/icu_provider_adapters/latest/fixed/any_payload/struct.FixedProvider.html
[`HelloWorldProvider`]: hello_world::HelloWorldProvider
[`Yokeable`]: yoke::Yokeable
[`impl_dynahttps://docs.rs/icu_provider/2.1.1/icu_provider/hello_world/struct.HelloWorldProvider.htmlimpl_dynamic_dahttps://docs.rs/yoke/latest/yoke/yokeable/trait.Yokeable.htmlicu_provider_adapters`]: https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/index.html
[`SourceDataProvider`]: https://docs.rs/icu_provider_source/latest/icu_provider_source/struct.SourceDataProvider.html
[`as_deserializing()`]: https://docs.rs/icu_provider/2.1.1/icu_provider/buf/serde/trait.AsDeserializingBufferProvider.html#tymethod.as_deserializing
[`FsDataProvider`]: https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html
[`BlobDataProvider`]: https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html