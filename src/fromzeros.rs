use crate::*;
use core::mem;
use frunk::hlist::{HCons, HNil};
use frunk_core::generic::Generic;

/// ## Working Example
/// ```rust
/// use typelayout::{ReprC, Generic, Layout, FromZeros};
///
/// #[derive(Generic, Default, Debug, PartialEq)]
/// #[repr(C)]
/// pub struct Tree {
///   height: u8,
///   age: u8,
/// }
///
/// unsafe impl ReprC for Tree {}
///
/// assert_eq!(<Tree as Default>::default(), <Tree as FromZeros>::zeroed());
/// ```
///
/// ## Failing Example
/// This marker trait is not implemented if a type has padding.
/// ```compile_fail
/// use typelayout::{ReprC, Generic, Layout, FromZeros};
///
/// #[derive(Generic, Default, Debug, PartialEq)]
/// #[repr(C)]
/// pub struct Tree {
///   height: u8,
///   age: u16, // padding will be inserted between `height` and `age`
/// }
///
/// unsafe impl ReprC for Tree {}
///
/// assert_eq!(<Tree as Default>::default(), <Tree as FromZeros>::zeroed());
/// ```
pub unsafe trait FromZeros {
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

unsafe impl<T: Generic + ReprC> FromZeros for T
where
  T: NoPadding,
  <Self as Generic>::Repr: FromZeros,
{}

/// TODO: Create wrapper types around HNil and HCons, so this isn't public.
#[doc(hidden)]
unsafe impl FromZeros for HNil {}

/// TODO: Create wrapper types around HNil and HCons, so this isn't public.
#[doc(hidden)]
unsafe impl<H, Tail> FromZeros for HCons<H, Tail>
where
    H: FromZeros,
    Tail: FromZeros,
{}
