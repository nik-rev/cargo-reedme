The `ndarray` crate provides an *n*-dimensional container for general elements
and for numerics.

In *n*-dimensional we include, for example, 1-dimensional rows or columns,
2-dimensional matrices, and higher dimensional arrays. If the array has *n*
dimensions, then an element in the array is accessed by using that many indices.
Each dimension is also called an *axis*.

To get started, functionality is provided in the following core types:
- **[`ArrayBase`](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayBase.html)**:
  The *n*-dimensional array type itself.<br>
  It is used to implement both the owned arrays and the views; see its docs
  for an overview of all array features.<br>
- The main specific array type is **[`Array`](https://docs.rs/ndarray/0.17.2/ndarray/type.Array.html)**, which owns
  its elements.
- A reference type, **[`ArrayRef`](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayRef.html)**, that contains most of the functionality
  for reading and writing to arrays.
- A reference type, **[`LayoutRef`](https://docs.rs/ndarray/0.17.2/ndarray/struct.LayoutRef.html)**, that contains most of the functionality
  for reading and writing to array layouts: their shape and strides.

## Highlights

- Generic *n*-dimensional array
- [Slicing](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayBase.html), also with arbitrary step size, and negative
  indices to mean elements from the end of the axis.
- Views and subviews of arrays; iterators that yield subviews.
- Higher order operations and arithmetic are performant
- Array views can be used to slice and mutate any `[T]` data using
  `ArrayView::from` and `ArrayViewMut::from`.
- [`Zip`](https://docs.rs/ndarray/0.17.2/ndarray/zip/struct.Zip.html) for lock step function application across two or more arrays or other
  item producers ([`NdProducer`](https://docs.rs/ndarray/0.17.2/ndarray/zip/ndproducer/trait.NdProducer.html) trait).

## Crate Status

- Still iterating on and evolving the crate
  + The crate is continuously developing, and breaking changes are expected
    during evolution from version to version. We adopt the newest stable
    rust features if we need them.
  + Note that functions/methods/traits/etc. hidden from the docs are not
    considered part of the public API, so changes to them are not
    considered breaking changes.
- Performance:
  + Prefer higher order methods and arithmetic operations on arrays first,
    then iteration, and as a last priority using indexed algorithms.
  + The higher order functions like [`.map()`](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayRef.html#method.map),
    [`.map_inplace()`](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayRef.html#method.map_inplace), [`.zip_mut_with()`](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayRef.html#method.zip_mut_with),
    [`Zip`](https://docs.rs/ndarray/0.17.2/ndarray/zip/struct.Zip.html) and [`azip!()`](https://docs.rs/ndarray/0.17.2/ndarray/macro.azip.html) are the most efficient ways
    to perform single traversal and lock step traversal respectively.
  + Performance of an operation depends on the memory layout of the array
    or array view. Especially if it’s a binary operation, which
    needs matching memory layout to be efficient (with some exceptions).
  + Efficient floating point matrix multiplication even for very large
    matrices; can optionally use BLAS to improve it further.

- **MSRV: Requires Rust 1.64 or later**

## Crate Feature Flags

The following crate feature flags are available. They are configured in your
`Cargo.toml`. See [`doc::crate_feature_flags`] for more information.

- `std`: Rust standard library-using functionality (enabled by default)
- `serde`: serialization support for serde 1.x
- `rayon`: Parallel iterators, parallelized methods, the [`parallel`](https://docs.rs/ndarray/0.17.2/ndarray/parallel/) module and [`par_azip!`](https://docs.rs/ndarray/0.17.2/ndarray/macro.par_azip.html).
- `approx` Implementations of traits from the [`approx`](https://docs.rs/approx/latest/approx/) crate.
- `blas`: transparent BLAS support for matrix multiplication, needs configuration.
- `matrixmultiply-threading`: Use threading from `matrixmultiply`.

## Documentation

* The docs for [`ArrayBase`](https://docs.rs/ndarray/0.17.2/ndarray/struct.ArrayBase.html) provide an overview of
  the *n*-dimensional array type. Other good pages to look at are the
  documentation for the [`s![]`](https://docs.rs/ndarray/0.17.2/ndarray/macro.s.html) and
  [`azip!()`](https://docs.rs/ndarray/0.17.2/ndarray/macro.azip.html) macros.

* If you have experience with NumPy, you may also be interested in
  [`ndarray_for_numpy_users`](doc::ndarray_for_numpy_users).

## The ndarray ecosystem

`ndarray` provides a lot of functionality, but it’s not a one-stop solution.

`ndarray` includes matrix multiplication and other binary/unary operations out of the box.
More advanced linear algebra routines (e.g. SVD decomposition or eigenvalue computation)
can be found in [`ndarray-linalg`](https://crates.io/crates/ndarray-linalg).

The same holds for statistics: `ndarray` provides some basic functionalities (e.g. `mean`)
but more advanced routines can be found in [`ndarray-stats`](https://crates.io/crates/ndarray-stats).

If you are looking to generate random arrays instead, check out [`ndarray-rand`](https://crates.io/crates/ndarray-rand).

For conversion between `ndarray`, [`nalgebra`](https://crates.io/crates/nalgebra) and
[`image`](https://crates.io/crates/image) check out [`nshare`](https://crates.io/crates/nshare).