use crate::*;
use typenum::*;
use core::mem;
use frunk::hlist::HNil;

/// `U: AlignedTo<T>` indicates that `U`’s alignment requirement is at least as
/// strict as `T`'s, and so any memory address which satisfies the alignment of
/// `U` also satisfies the alignment of `T`.
///
/// ```rust
/// use typelayout::*;
/// use static_assertions::*;
/// use core::mem::align_of;
///
/// #[derive(Generic)]
/// #[repr(C)]
/// struct Struct1 {
///   a: u8,
///   b: u8,
///   c: u8,
///   d: u8,
/// }
/// 
/// unsafe impl Repr<C> for Struct1 {}
/// 
/// #[derive(Generic)]
/// #[repr(C)]
/// struct Struct2 {
///   a: u32,
/// }
/// 
/// unsafe impl Repr<C> for Struct2 {}
///
/// fn aligned_to<T, U>()
/// where U: AlignedTo<T>{}
///
/// assert_eq!(align_of::<Struct1>(), 1);
/// assert_eq!(align_of::<Struct2>(), 4);
///
/// assert_impl_all!(
///   Struct2,
///   AlignedTo<Struct1>);
///
/// assert_not_impl_any!(
///   Struct1,
///   AlignedTo<Struct2>);
/// ```
pub trait AlignedTo<T>
{}

impl<T, U> AlignedTo<T> for U
where
  T: TypeLayout,
  U: TypeLayout,
  <U as TypeLayout>::Align: PartialDiv<<T as TypeLayout>::Align>
{}