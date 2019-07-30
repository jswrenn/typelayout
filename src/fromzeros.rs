use crate::*;
use core::mem;
use frunk::hlist::{HCons, HNil};
use frunk_core::generic::Generic;

/// ## Working Example
/// This marker trait is implemented for structs iff it is `ReprC`, has no
/// padding bytes, and all fields are also `FromZeros`:
/// ```rust
/// use typelayout::{ReprC, Generic, Layout, FromZeros};
///
/// #[derive(Generic, Default, Debug, PartialEq)]
/// #[repr(C)]
/// pub struct Struct {
///   first: u8,
///   second: u8,
/// }
///
/// unsafe impl ReprC for Struct {}
///
/// assert_eq!(<Struct as Default>::default(), <Struct as FromZeros>::zeroed());
/// ```
///
/// ## Failing Example
/// This marker trait is not implemented if a type has padding:
/// ```compile_fail
/// use typelayout::{ReprC, Generic, Layout, FromZeros};
///
/// #[derive(Generic, Default, Debug, PartialEq)]
/// #[repr(C)]
/// pub struct Struct {
///   first: u8,
///   second: u16, // padding will be inserted between `first` and `second`
/// }
///
/// unsafe impl ReprC for Struct {}
///
/// // `Struct` does not implement `FromZeros`, because it has a padding byte!
/// assert_eq!(<Struct as Default>::default(), <Struct as FromZeros>::zeroed());
/// ```
pub unsafe trait FromZeros {
  /// Initialize an instance of `Self` from zeroed bytes.
  #[inline(always)]
  fn zeroed() -> Self
  where Self: Sized,
  {
    unsafe { mem::zeroed() }
  }
}

/// A `*const T` from zeros is a null pointer.
unsafe impl<T> FromZeros for *const T {}

/// A `*mut T` from zeros is a null pointer.
unsafe impl<T> FromZeros for *mut T {}

unsafe impl FromZeros for i8    {}
unsafe impl FromZeros for i16   {}
unsafe impl FromZeros for i32   {}
unsafe impl FromZeros for i64   {}
unsafe impl FromZeros for i128  {}
unsafe impl FromZeros for isize {}

unsafe impl FromZeros for u8    {}
unsafe impl FromZeros for u16   {}
unsafe impl FromZeros for u32   {}
unsafe impl FromZeros for u64   {}
unsafe impl FromZeros for u128  {}
unsafe impl FromZeros for usize {}

unsafe impl FromZeros for f32   {}
unsafe impl FromZeros for f64   {}

/// A pub-in-priv wrapper type so we don't expose `FromZeros` implementations
/// for arbitrary `HNil` and `HCons` instances.
pub struct Struct<F>(F);

unsafe impl<T: Generic + ReprC> FromZeros for T
where
  T: NoPadding,
  Struct<<Self as Generic>::Repr>: FromZeros,
{}

unsafe impl FromZeros for Struct<HNil> {}

unsafe impl<H, Tail> FromZeros for Struct<HCons<H, Tail>>
where
    H: FromZeros,
    Struct<Tail>: FromZeros,
{}
