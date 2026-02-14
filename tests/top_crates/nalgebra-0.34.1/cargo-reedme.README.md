# nalgebra

**nalgebra** is a linear algebra library written for Rust targeting:

* General-purpose linear algebra (still lacks a lot of features…)
* Real-time computer graphics.
* Real-time computer physics.

## Using **nalgebra**
You will need the last stable build of the [rust compiler](https://www.rust-lang.org)
and the official package manager: [cargo](https://github.com/rust-lang/cargo).

Simply add the following to your `Cargo.toml` file:

```rust
[dependencies]
// TODO: replace the * by the latest version.
nalgebra = "*"
```


Most useful functionalities of **nalgebra** are grouped in the root module `nalgebra::`.

However, the recommended way to use **nalgebra** is to import types and traits
explicitly, and call free-functions using the `na::` prefix:

```rust
#[macro_use]
extern crate approx; // For the macro assert_relative_eq!
extern crate nalgebra as na;
use na::{Vector3, Rotation3};

fn main() {
    let axis  = Vector3::x_axis();
    let angle = 1.57;
    let b     = Rotation3::from_axis_angle(&axis, angle);

    assert_relative_eq!(b.axis().unwrap(), axis);
    assert_relative_eq!(b.angle(), angle);
}
```


## Features
**nalgebra** is meant to be a general-purpose, low-dimensional, linear algebra library, with
an optimized set of tools for computer graphics and physics. Those features include:

* A single parametrizable type [`Matrix`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/matrix/struct.Matrix.html) for vectors, (square or rectangular) matrices, and
  slices with dimensions known either at compile-time (using type-level integers) or at runtime.
* Matrices and vectors with compile-time sizes are statically allocated while dynamic ones are
  allocated on the heap.
* Convenient aliases for low-dimensional matrices and vectors: [`Vector1`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/alias/type.Vector1.html) to
  [`Vector6`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/alias/type.Vector6.html) and [`Matrix1x1`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/alias/type.Matrix1.html) to [`Matrix6x6`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/alias/type.Matrix6.html), including rectangular
  matrices like [`Matrix2x5`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/alias/type.Matrix2x5.html).
* Points sizes known at compile time, and convenience aliases: [`Point1`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/point_alias/type.Point1.html) to
  [`Point6`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/point_alias/type.Point6.html).
* Translation (seen as a transformation that composes by multiplication):
  [`Translation2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/translation_alias/type.Translation2.html), [`Translation3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/translation_alias/type.Translation3.html).
* Rotation matrices: [`Rotation2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/rotation_alias/type.Rotation2.html), [`Rotation3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/rotation_alias/type.Rotation3.html).
* Quaternions: [`Quaternion`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/quaternion/struct.Quaternion.html), [`UnitQuaternion`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/quaternion/type.UnitQuaternion.html) (for 3D rotation).
* Unit complex numbers can be used for 2D rotation: [`UnitComplex`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/unit_complex/type.UnitComplex.html).
* Algebraic entities with a norm equal to one: [`Unit<T>`](https://docs.rs/nalgebra/0.34.1/nalgebra/base/unit/struct.Unit.html), e.g., `Unit<Vector3<f32>>`.
* Isometries (translation ⨯ rotation): [`Isometry2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/isometry_alias/type.Isometry2.html), [`Isometry3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/isometry_alias/type.Isometry3.html)
* Similarity transformations (translation ⨯ rotation ⨯ uniform scale):
  [`Similarity2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/similarity_alias/type.Similarity2.html), [`Similarity3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/similarity_alias/type.Similarity3.html).
* Affine transformations stored as a homogeneous matrix:
  [`Affine2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/transform_alias/type.Affine2.html), [`Affine3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/transform_alias/type.Affine3.html).
* Projective (i.e. invertible) transformations stored as a homogeneous matrix:
  [`Projective2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/transform_alias/type.Projective2.html), [`Projective3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/transform_alias/type.Projective3.html).
* General transformations that does not have to be invertible, stored as a homogeneous matrix:
  [`Transform2`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/transform_alias/type.Transform2.html), [`Transform3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/transform_alias/type.Transform3.html).
* 3D projections for computer graphics: [`Perspective3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/perspective/struct.Perspective3.html),
  [`Orthographic3`](https://docs.rs/nalgebra/0.34.1/nalgebra/geometry/orthographic/struct.Orthographic3.html).
* Matrix factorizations: [`Cholesky`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/cholesky/struct.Cholesky.html), [`QR`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/qr/struct.QR.html), [`LU`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/lu/struct.LU.html), [`FullPivLU`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/full_piv_lu/struct.FullPivLU.html),
  [`SVD`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/svd/struct.SVD.html), [`Schur`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/schur/struct.Schur.html), [`Hessenberg`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/hessenberg/struct.Hessenberg.html), [`SymmetricEigen`](https://docs.rs/nalgebra/0.34.1/nalgebra/linalg/symmetric_eigen/struct.SymmetricEigen.html).
* Insertion and removal of rows of columns of a matrix.