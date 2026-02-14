Generic `Atomic<T>` wrapper type

Atomic types provide primitive shared-memory communication between
threads, and are the building blocks of other concurrent types.

This library defines a generic atomic wrapper type `Atomic<T>` for all
`T: NoUninit` types.
Atomic types present operations that, when used correctly, synchronize
updates between threads.

The `NoUninit` bound is from the [bytemuck] crate, and indicates that a
type has no internal padding bytes. You will need to derive or implement
this trait for all types used with `Atomic<T>`.

Each method takes an `Ordering` which represents the strength of
the memory barrier for that operation. These orderings are the
same as [LLVM atomic orderings][1].

[1]: http://llvm.org/docs/LangRef.html#memory-model-for-concurrent-operations

Atomic variables are safe to share between threads (they implement `Sync`)
but they do not themselves provide the mechanism for sharing. The most
common way to share an atomic variable is to put it into an `Arc` (an
atomically-reference-counted shared pointer).

Most atomic types may be stored in static variables, initialized using
the `const fn` constructors. Atomic statics are often used for lazy global
initialization.

[bytemuck]: https://docs.rs/bytemuck