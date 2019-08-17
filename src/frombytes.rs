use crate::*;
use core::mem;
use frunk::hlist::HNil;

/// A valid instance of `T` is also a valid instance of `Self`
///
/// ```rust
/// #[derive(Generic)]
/// #[repr(C)]
/// struct Struct1 {
///   a: u32
/// }
/// 
/// unsafe impl Repr<C> for Struct1 {}
/// 
/// #[derive(Generic)]
/// #[repr(C)]
/// struct Struct2 {
///   a: Struct1,
/// }
/// 
/// unsafe impl Repr<C> for Struct2 {}
///
/// fn from_bytes<T, U>()
/// where T: FromBytes<U>{}
///
/// from_bytes::<
///     Struct1,
///     Struct2>();
/// ```
pub trait FromBytes<T>
{
  /// Construct an instance of `Self` from the bytes of `T`.
  #[inline(always)]
  unsafe fn from_bytes(from: T) -> Self
  where Self: Sized
  {
    let to = mem::transmute_copy(&from);
    mem::forget(from);
    to
  }
}

impl<T, U> FromBytes<T> for U
where
  T: TypeLayout,
  U: TypeLayout,
  <U as TypeLayout>::Repr: FromLayout<<T as TypeLayout>::Repr>,
{}

// U: FromBytes<T> indicates that the bytes of any valid T
// correspond to the bytes of a valid instance of U.
pub trait FromLayout<T>
{}

/// Base case.
impl FromLayout<HNil> for HNil
{}

// U: FromBytes<T> indicates that the bytes of any valid T
// correspond to the bytes of a valid instance of U.

///  `Init -> *`
impl<
      TR,
  U1, UR,
>
FromLayout<
  Hlist![Init, ...TR]
>
for
  Hlist![U1, ...UR]
where
  U1: FromSlot<Init>,
  UR: FromLayout<TR>,
{}

// An initialized byte may only be constructed from another initialized byte.
assert_impl_all!(
  Hlist![Init],
  FromLayout<Hlist![Init]>);

// An initialized byte may not be constructed from an uninitialized byte.
assert_not_impl_any!(
  Hlist![Init],
  FromLayout<Hlist![Uninit]>);

/// `Uninit -> *`
impl<
      TR,
  U1, UR,
>
FromLayout<
  Hlist![Uninit, ...TR]
>
for
  Hlist![U1, ...UR]
where
  U1: FromSlot<Uninit>,
  UR: FromLayout<TR>,
{}

// An uninitialized byte may be constructed from an initialized or uninitialized byte.
assert_impl_all!(
  Hlist![Uninit],
  FromLayout<Hlist![Init]>,
  FromLayout<Hlist![Uninit]>);

/// `*const T -> *const U`
impl<
  T1, TR,
  U1, UR>
FromLayout<
  Hlist![*const T1, ...TR]
>
for
  Hlist![*const U1, ...UR]
where
  U1: FromBytes<T1>,
  UR: FromLayout<TR>,
{}

assert_impl_all!(
  Hlist![*const u64],
  FromLayout<Hlist![*const i64]>);

assert_not_impl_any!(
  Hlist![*const u64],
  FromLayout<Hlist![*const i16]>);

/// `*mut T -> *const U`
impl<
  T1, TR,
  U1, UR>
FromLayout<
  Hlist![*mut T1, ...TR]
>
for
  Hlist![*const U1, ...UR]
where
  U1: FromBytes<T1>,
  UR: FromLayout<TR>,
{}

// A const pointer may be created from a mut pointer.
assert_impl_all!(
  Hlist![*const u64],
  FromLayout<Hlist![*mut u64]>);

// A mut pointer may NOT be created from a const pointer.
assert_not_impl_any!(
  Hlist![*mut u64],
  FromLayout<Hlist![*const u64]>);

/// `&'t mut T -> *const U`
impl<'t,
  T1, TR,
  U1, UR>
FromLayout<
  Hlist![&'t T1, ...TR]
>
for
  Hlist![*const U1, ...UR]
where
  U1: FromBytes<T1>,
  UR: FromLayout<TR>,
{}

// A smart pointer may be converted to a const pointer.
assert_impl_all!(
  Hlist![*const u64],
  FromLayout<Hlist![&'static u64]>);

// A const pointer may NOT be converter to a smart pointer.
assert_not_impl_any!(
  Hlist![&'static u64],
  FromLayout<Hlist![*const u64]>);

/// `&'t T -> &'u U`
impl<'t, 'u,
  T1, TR,
  U1, UR>
FromLayout<
  Hlist![&'t T1, ...TR]
>
for
  Hlist![&'u U1, ...UR]
where
  't: 'u,
  U1: FromBytes<T1>,
  UR: FromLayout<TR>,
{}

// Pointers are convertible if their underlying types are convertible.
assert_impl_all!(
  Hlist![&'static u64],
  FromLayout<Hlist![&'static i64]>);

// Pointers are not convertible if their underlying types aren't convertible.
assert_not_impl_any!(
  Hlist![&'static u64],
  FromLayout<Hlist![&'static u16]>);

/// `&mut 't T -> &'u U`
impl<'t, 'u,
  T1, TR,
  U1, UR>
FromLayout<
  Hlist![&'t mut T1, ...TR]
>
for
  Hlist![&'u U1, ...UR]
where
  't: 'u,
  U1: FromBytes<T1>,
  UR: FromLayout<TR>,
{}

// Pointers are convertible if their underlying types are convertible.
assert_impl_all!(
  Hlist![&'static u64],
  FromLayout<Hlist![&'static mut i64]>);

// Pointers are not convertible if their underlying types aren't convertible.
assert_not_impl_any!(
  Hlist![&'static mut u64],
  FromLayout<Hlist![&'static u16]>);

/// `&mut 't T -> *const U`
impl<'t,
  T1, TR,
  U1, UR>
FromLayout<
  Hlist![&'t mut T1, ...TR]
>
for
  Hlist![*const U1, ...UR]
where
  U1: FromBytes<T1>,
  UR: FromLayout<TR>,
{}

// If the underlying types are convertible, a const pointer may be created from:
assert_impl_all!(
  Hlist![*const u64],
  FromLayout<Hlist![*const i64]>,
  FromLayout<Hlist![*mut i64]>,
  FromLayout<Hlist![&'static i64]>,
  FromLayout<Hlist![&'static mut i64]>);

// A mut smart pointer may not be created from a const pointer.
assert_not_impl_any!(
  Hlist![&'static mut u64],
  FromLayout<Hlist![*const u64]>);

macro_rules! decompose_ptr{
  ($ty: ty) => {
    #[cfg(target_pointer_width = "32")]
    impl<'t,
      T, TR,
      U2, U3, U4, UR>
    FromLayout<
      Hlist![$ty, ...TR]
    >
    for
      Hlist![Init, U2, U3, U4, ...UR]
    where
      U2: FromSlot<Init>,
      U3: FromSlot<Init>,
      U4: FromSlot<Init>,
      UR: FromLayout<TR>,
    {}

    #[cfg(target_pointer_width = "32")]
    impl<'t,
      T, TR,
      U2, U3, U4, UR>
    FromLayout<
      Hlist![$ty, ...TR]
    >
    for
      Hlist![Uninit, U2, U3, U4, ...UR]
    where
      U2: FromSlot<Init>,
      U3: FromSlot<Init>,
      U4: FromSlot<Init>,
      UR: FromLayout<TR>,
    {}

    #[cfg(target_pointer_width = "64")]
    impl<'t,
      T, TR,
      U2, U3, U4, U5, U6, U7, U8, UR>
    FromLayout<
      Hlist![$ty, ...TR]
    >
    for
      Hlist![Init, U2, U3, U4, U5, U6, U7, U8, ...UR]
    where
      U2: FromSlot<Init>,
      U3: FromSlot<Init>,
      U4: FromSlot<Init>,
      U5: FromSlot<Init>,
      U6: FromSlot<Init>,
      U7: FromSlot<Init>,
      U8: FromSlot<Init>,
      UR: FromLayout<TR>,
    {}

    #[cfg(target_pointer_width = "64")]
    impl<'t,
      T, TR,
      U2, U3, U4, U5, U6, U7, U8, UR>
    FromLayout<
      Hlist![$ty, ...TR]
    >
    for
      Hlist![Uninit, U2, U3, U4, U5, U6, U7, U8, ...UR]
    where
      U2: FromSlot<Init>,
      U3: FromSlot<Init>,
      U4: FromSlot<Init>,
      U5: FromSlot<Init>,
      U6: FromSlot<Init>,
      U7: FromSlot<Init>,
      U8: FromSlot<Init>,
      UR: FromLayout<TR>,
    {}
  };
}

decompose_ptr!(*const T);
decompose_ptr!(*mut T);
decompose_ptr!(&'t T);
decompose_ptr!(&'t mut T);

#[cfg(target_pointer_width = "32")]
type InitializedBytes<R=HNil> = Hlist![Init, Init, Init, Init, ...R];
#[cfg(target_pointer_width = "64")]
type InitializedBytes<R=HNil> = Hlist![Init, Init, Init, Init, Init, Init, Init, Init, ...R];

#[cfg(target_pointer_width = "32")]
type UninitializedBytes<R=HNil> = Hlist![Uninit, Uninit, Uninit, Uninit, ...R];
#[cfg(target_pointer_width = "64")]
type UninitializedBytes<R=HNil> = Hlist![Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, Uninit, ...R];

// pointers may be decomposed into initialized bytes
assert_impl_all!(
  InitializedBytes,
  FromLayout<Hlist![*const u64]>,
  FromLayout<Hlist![*mut u64]>,
  FromLayout<Hlist![&'static u64]>,
  FromLayout<Hlist![&'static mut u64]>);

// pointers may be decomposed into uninitialized bytes
assert_impl_all!(
  UninitializedBytes,
  FromLayout<Hlist![*const u64]>,
  FromLayout<Hlist![*mut u64]>,
  FromLayout<Hlist![&'static u64]>,
  FromLayout<Hlist![&'static mut u64]>);

/// `[Init; target_pointer_width] -> *const U`
impl<
  TR,
  U1, UR>
FromLayout<
  InitializedBytes<TR>
>
for
  Hlist![*const U1, ...UR]
where
  UR: FromLayout<TR>,
{}

/// `[Init; target_pointer_width] -> *mut U`
impl<
  TR,
  U1, UR>
FromLayout<
  InitializedBytes<TR>
>
for
  Hlist![*mut U1, ...UR]
where
  UR: FromLayout<TR>,
{}

// const ptr may be created from initialized bytes
#[cfg(target_pointer_width = "64")]
assert_impl_all!(
  Hlist![*const u64],
  FromLayout<InitializedBytes>);

// const ptr may NOT be created from uninitialized bytes
#[cfg(target_pointer_width = "64")]
assert_not_impl_any!(
  Hlist![*const u64],
  FromLayout<UninitializedBytes>);

/// A valid instance of `T` is also a valid instance of `Self` 
pub trait FromSlot<T>
{}

/// An initialized byte is a valid instance of an initialized byte.
impl FromSlot<Init>   for Init    {}

/// An uninitialized byte is a valid instance of an uninitialized byte.
impl FromSlot<Uninit> for Uninit  {}

/// An initialized byte is a valid instance of an uninitialized byte.
impl FromSlot<Init>   for Uninit  {}

/// `&'t T` is a valid instance of `&'u U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U>  FromSlot<&'t T>       for   &'u U
where U: FromBytes<T>
{}

/// `&'t mut T` is a valid instance of `&'u U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U>  FromSlot<&'t mut T>   for   &'u U
where U: FromBytes<T>
{}

/// `&'t mut T` is a valid instance of `&'u mut U`, if the underlying types are
/// covertible and `'t` outlives `'u`.
impl<'t: 'u, 'u, T, U>  FromSlot<&'t mut T>   for   &'u mut U
where U: FromBytes<T>
{}

/// `&'t T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<'t, T, U>          FromSlot<&'t T>       for   *const U
where U: FromBytes<T>
{}

/// `&'t mut T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<'t, T, U>          FromSlot<&'t mut T>   for   *const U
where U: FromBytes<T>
{}

/// `&'t mut T` is a valid instance of `*mut U`, if the underlying types are
/// covertible.
impl<'t, T, U>          FromSlot<&'t mut T>   for   *mut U
where U: FromBytes<T>
{}

/// `*const T` is a valid instance of `*const U`, if the underlying types are
/// covertible.
impl<T, U>              FromSlot<*const T>   for   *const U
where U: FromBytes<T>
{}
