This crate provides [Yoke](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html), which allows one to “yoke” (attach) a zero-copy deserialized
object (say, a [`Cow<'a, str>`](alloc::borrow::Cow)) to the source it was deserialized from, (say, an [`Rc<[u8]>`](alloc::rc::Rc)),
known in this crate as a “cart”, producing a type that looks like `Yoke<Cow<'static, str>, Rc<[u8]>>`
and can be moved around with impunity.

Succinctly, this allows one to “erase” static lifetimes and turn them into dynamic ones, similarly
to how `dyn` allows one to “erase” static types and turn them into dynamic ones.

Most of the time the yokeable `Y` type will be some kind of zero-copy deserializable
abstraction, potentially with an owned variant (like [`Cow`](alloc::borrow::Cow),
[`ZeroVec`](https://docs.rs/zerovec), or an aggregate containing such types), and the cart `C` will be some smart pointer like
  [`Box<T>`](alloc::boxed::Box), [`Rc<T>`](alloc::rc::Rc), or [`Arc<T>`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html), potentially wrapped in an [`Option<T>`](https://doc.rust-lang.org/stable/core/option/enum.Option.html).

The key behind this crate is [`Yoke::get()`](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html#method.get), where calling [Yoke::get](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html#method.get) on a type like
`Yoke<Cow<'static, str>, _>` will get you a short-lived `&'a Cow<'a, str>`, restricted to the
lifetime of the borrow used during [`.get()`](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html#method.get). This is entirely safe since the `Cow` borrows from
the cart type `C`, which cannot be interfered with as long as the `Yoke` is borrowed by [`.get()`](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html#method.get).
[`.get()`](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html#method.get) protects access by essentially reifying the erased lifetime to a safe local one
when necessary.

See the documentation of [`Yoke`](https://docs.rs/yoke/0.8.1/yoke/yoke/struct.Yoke.html) for more details.