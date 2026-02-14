This crate provides helper types for matching against enum variants, and
extracting bindings to each of the fields in the deriving Struct or Enum in
a generic way.

If you are writing a `#[derive]` which needs to perform some operation on
every field, then you have come to the right place!

# Example: `WalkFields`
### Trait Implementation
```rust
pub trait WalkFields: std::any::Any {
    fn walk_fields(&self, walk: &mut FnMut(&WalkFields));
}
impl WalkFields for i32 {
    fn walk_fields(&self, _walk: &mut FnMut(&WalkFields)) {}
}
```

### Custom Derive
```rust
fn walkfields_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| quote!{
        walk(#bi)
    });

    s.gen_impl(quote! {
        extern crate synstructure_test_traits;

        gen impl synstructure_test_traits::WalkFields for @Self {
            fn walk_fields(&self, walk: &mut FnMut(&synstructure_test_traits::WalkFields)) {
                match *self { #body }
            }
        }
    })
}
synstructure::decl_derive!([WalkFields] => walkfields_derive);

/*
 * Test Case
 */
fn main() {
    synstructure::test_derive! {
        walkfields_derive {
            enum A<T> {
                B(i32, T),
                C(i32),
            }
        }
        expands to {
            const _: () = {
                extern crate synstructure_test_traits;
                impl<T> synstructure_test_traits::WalkFields for A<T>
                    where T: synstructure_test_traits::WalkFields
                {
                    fn walk_fields(&self, walk: &mut FnMut(&synstructure_test_traits::WalkFields)) {
                        match *self {
                            A::B(ref __binding_0, ref __binding_1,) => {
                                { walk(__binding_0) }
                                { walk(__binding_1) }
                            }
                            A::C(ref __binding_0,) => {
                                { walk(__binding_0) }
                            }
                        }
                    }
                }
            };
        }
    }
}
```

# Example: `Interest`
### Trait Implementation
```rust
pub trait Interest {
    fn interesting(&self) -> bool;
}
impl Interest for i32 {
    fn interesting(&self) -> bool { *self > 0 }
}
```

### Custom Derive
```rust
fn interest_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.fold(false, |acc, bi| quote!{
        #acc || synstructure_test_traits::Interest::interesting(#bi)
    });

    s.gen_impl(quote! {
        extern crate synstructure_test_traits;
        gen impl synstructure_test_traits::Interest for @Self {
            fn interesting(&self) -> bool {
                match *self {
                    #body
                }
            }
        }
    })
}
synstructure::decl_derive!([Interest] => interest_derive);

/*
 * Test Case
 */
fn main() {
    synstructure::test_derive!{
        interest_derive {
            enum A<T> {
                B(i32, T),
                C(i32),
            }
        }
        expands to {
            const _: () = {
                extern crate synstructure_test_traits;
                impl<T> synstructure_test_traits::Interest for A<T>
                    where T: synstructure_test_traits::Interest
                {
                    fn interesting(&self) -> bool {
                        match *self {
                            A::B(ref __binding_0, ref __binding_1,) => {
                                false ||
                                    synstructure_test_traits::Interest::interesting(__binding_0) ||
                                    synstructure_test_traits::Interest::interesting(__binding_1)
                            }
                            A::C(ref __binding_0,) => {
                                false ||
                                    synstructure_test_traits::Interest::interesting(__binding_0)
                            }
                        }
                    }
                }
            };
        }
    }
}
```

For more example usage, consider investigating the `abomonation_derive` crate,
which makes use of this crate, and is fairly simple.