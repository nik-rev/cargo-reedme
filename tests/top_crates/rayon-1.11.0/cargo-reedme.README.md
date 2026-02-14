Rayon is a data-parallelism library that makes it easy to convert sequential
computations into parallel.

It is lightweight and convenient for introducing parallelism into existing
code. It guarantees data-race free executions and takes advantage of
parallelism when sensible, based on work-load at runtime.

# How to use Rayon

There are two ways to use Rayon:

- **High-level parallel constructs** are the simplest way to use Rayon and also
  typically the most efficient.
  - [Parallel iterators] make it easy to convert a sequential iterator to
    execute in parallel.
    - The [`ParallelIterator`] trait defines general methods for all parallel iterators.
    - The [`IndexedParallelIterator`] trait adds methods for iterators that support random
      access.
  - The [`par_sort`] method sorts `&mut [T]` slices (or vectors) in parallel.
  - [`par_extend`] can be used to efficiently grow collections with items produced
    by a parallel iterator.
- **Custom tasks** let you divide your work into parallel tasks yourself.
  - [`join`](https://docs.rs/rayon_core/latest/rayon_core/join/fn.join.html) is used to subdivide a task into two pieces.
  - [`scope`](https://docs.rs/rayon_core/latest/rayon_core/scope/fn.scope.html) creates a scope within which you can create any number of parallel tasks.
  - [`ThreadPoolBuilder`](https://docs.rs/rayon_core/latest/rayon_core/struct.ThreadPoolBuilder.html) can be used to create your own thread pools or customize
    the global one.

[Parallel iterators]: iter
[`par_sort`]: slice::ParallelSliceMut::par_sort
[`par_extend`]: iter::ParallelExtend::par_extend
[`Paralhttps://docs.rs/rayon/1.11.0/rayon/slice/trait.ParallelSliceMut.html#tymethod.par_sortyon/1.11.0/rayon/https://docs.rs/rayon/1.11.0/rayon/iter/trait.ParallelExtend.html#tymethod.par_extend
[`IndexedParallelIterator`]: https://docs.rs/rayon/1.11.0/rayon/iter/trait.IndexedParallelIterator.html

# Basic usage and the Rayon prelude

First, you will need to add `rayon` to your `Cargo.toml`.

Next, to use parallel iterators or the other high-level methods,
you need to import several traits. Those traits are bundled into
the module [`rayon::prelude`]. It is recommended that you import
all of these traits at once by adding `use rayon::prelude::*` at
the top of each module that uses Rayon methods.

These traits give you access to the `par_iter` method which provides
parallel implementations of many iterative functions such as [`map`],
[`for_each`], [`filter`], [`fold`], and [more].

[`rayon::prelude`]: https://docs.rs/rayon/1.11.0/rayon/prelude/
[`map`]: https://docs.rs/rayon/1.11.0/rayon/iter/trait.ParallelIterator.html#tymethod.map
[`for_each`]: https://docs.rs/rayon/1.11.0/rayon/iter/trait.ParallelIterator.html#tymethod.for_each
[`filter`]: https://docs.rs/rayon/1.11.0/rayon/iter/trait.ParallelIterator.html#tymethod.filter
[`fold`]: https://docs.rs/rayon/1.11.0/rayon/iter/trait.ParallelIterator.html#tymethod.fold
[more]: https://docs.rs/rayon/1.11.0/rayon/iter/trait.ParallelIterator.html

# Crate Layout

Rayon extends many of the types found in the standard library with
parallel iterator implementations. The modules in the `rayon`
crate mirror [`std`](https://doc.rust-lang.org/stable/std/) itself: so, e.g., the `option` module in
Rayon contains parallel iterators for the `Option` type, which is
found in [the `option` module of `std`]. Similarly, the
`collections` module in Rayon offers parallel iterator types for
[the `collections` from `std`]. You will rarely need to access
these submodules unless you need to name iterator types
explicitly.

[the `option` module of `std`]: std::option
[the `collections` from `std`]: std::collections

# Targets without threading

Rayon has limited support for targets without `std` threading implementations.
See the [`rayon_core`](https://docs.rs/rayon_core/latest/rayon_core/) documentation for more information about its global fallback.

# Other questions?

See [the Rayon FAQ][faq].

[faq]: https://github.com/rayon-rs/rayon/blob/main/FAQ.md