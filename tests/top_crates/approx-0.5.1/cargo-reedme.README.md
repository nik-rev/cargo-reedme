A crate that provides facilities for testing the approximate equality of floating-point
based types, using either relative difference, or units in the last place (ULPs)
comparisons.

You can also use the `*_{eq, ne}!` and `assert_*_{eq, ne}!` macros to test for equality using a
more positional style:

```rust
#[macro_use]
extern crate approx;

use std::f64;

abs_diff_eq!(1.0, 1.0);
abs_diff_eq!(1.0, 1.0, epsilon = f64::EPSILON);

relative_eq!(1.0, 1.0);
relative_eq!(1.0, 1.0, epsilon = f64::EPSILON);
relative_eq!(1.0, 1.0, max_relative = 1.0);
relative_eq!(1.0, 1.0, epsilon = f64::EPSILON, max_relative = 1.0);
relative_eq!(1.0, 1.0, max_relative = 1.0, epsilon = f64::EPSILON);

ulps_eq!(1.0, 1.0);
ulps_eq!(1.0, 1.0, epsilon = f64::EPSILON);
ulps_eq!(1.0, 1.0, max_ulps = 4);
ulps_eq!(1.0, 1.0, epsilon = f64::EPSILON, max_ulps = 4);
ulps_eq!(1.0, 1.0, max_ulps = 4, epsilon = f64::EPSILON);
```

# Implementing approximate equality for custom types

The `*Eq` traits allow approximate equalities to be implemented on types, based on the
fundamental floating point implementations.

For example, we might want to be able to do approximate assertions on a complex number type:

```rust
#[macro_use]
extern crate approx;

#[derive(Debug, PartialEq)]
struct Complex<T> {
    x: T,
    i: T,
}

let x = Complex { x: 1.2, i: 2.3 };

assert_relative_eq!(x, x);
assert_ulps_eq!(x, x, max_ulps = 4);
```

To do this we can implement [`AbsDiffEq`](https://docs.rs/approx/0.5.1/approx/abs_diff_eq/trait.AbsDiffEq.html), [`RelativeEq`](https://docs.rs/approx/0.5.1/approx/relative_eq/trait.RelativeEq.html) and [`UlpsEq`](https://docs.rs/approx/0.5.1/approx/ulps_eq/trait.UlpsEq.html) generically in terms
of a type parameter that also implements `AbsDiffEq`, `RelativeEq` and `UlpsEq` respectively.
This means that we can make comparisons for either `Complex<f32>` or `Complex<f64>`:

```rust
impl<T: AbsDiffEq> AbsDiffEq for Complex<T> where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        T::abs_diff_eq(&self.x, &other.x, epsilon) &&
        T::abs_diff_eq(&self.i, &other.i, epsilon)
    }
}

impl<T: RelativeEq> RelativeEq for Complex<T> where
    T::Epsilon: Copy,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        T::relative_eq(&self.i, &other.i, epsilon, max_relative)
    }
}

impl<T: UlpsEq> UlpsEq for Complex<T> where
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
        T::ulps_eq(&self.i, &other.i, epsilon, max_ulps)
    }
}
```

# References

Floating point is hard! Thanks goes to these links for helping to make things a _little_
easier to understand:

- [Comparing Floating Point Numbers, 2012 Edition](
  https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
- [The Floating Point Guide - Comparison](http://floating-point-gui.de/errors/comparison/)
- [What Every Computer Scientist Should Know About Floating-Point Arithmetic](
  https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)