use core::ops::*;
use core::mem::*;
use core::num::*;
use typenum::*;
use frunk::hlist::*;
use crate::*;

/// The layout of a type.
pub trait TypeLayout {
  /// The `repr` marker of the type.
  type Marker: ReprMarker;

  /// The alignment of the type, in bytes.
  type Align: Unsigned;

  /// A generic representation of the type's memory layout.
  type Repr: Representation;
}

/// If a type is `Repr<C>`, use the `LayoutAlgorithm<C>` computer to calculate
/// its characteristics.
impl<T: Repr<C>> TypeLayout for T
{
  type Marker = C;
  type Align = <T as LayoutAlgorithm<C>>::Align;
  type Repr = <T as LayoutAlgorithm<C>>::Repr;
}

/// An `HList` is a `Representation` if it consistes solely of `Slot` items.
pub trait Representation: HList {
  /// The size of the layout, in bytes.
  type Size : Unsigned;
}

/// ZSTs have a representation of `HNil`.
impl Representation for HNil {
  type Size = U0;
}

/// The `Size` of an `HCons` is the sum of the size of its slots.
impl<H, T> Representation for HCons<H, T>
where
  H: Slot,
  T: Representation,

  <T as Representation>::Size: Add<<H as Slot>::Size>,
  Sum<<T as Representation>::Size, <H as Slot>::Size>: Unsigned,
{
  type Size = Sum<<T as Representation>::Size, <H as Slot>::Size>;
}


/// An item in a `Representation`.
pub trait Slot {
  type Size : Unsigned;
}

/// A slot that consumes one byte.
pub trait Byte: Slot {}

/// A byte of initialized memory.
pub type Init = u8;
impl Byte for Init {}

/// A slot may be byte of initialized memory.
impl Slot for Init {
  type Size = U1;
}

/// A non-zero byte of memory.
pub type NonZero = core::num::NonZeroU8;
impl Byte for NonZero {}

impl Slot for NonZero {
  type Size = U1;
}

/// A byte of possibly uninitialized memory.
pub type Uninit = MaybeUninit<u8>;
impl Byte for Uninit {}

/// A slot may be a byte of possibly uninitialized memory.
impl Slot for Uninit {
  type Size = U1;
}

/// A `Slot` may be a pointer.
impl<'t, T> Slot for &'t T {
  type Size = <*const T as Slot>::Size;
}

/// A `Slot` may be a pointer.
impl<'t, T> Slot for &'t mut T {
  type Size = <*const T as Slot>::Size;
}

/// A `Slot` may be a pointer.
impl<T> Slot for *const T {
  #[cfg(target_pointer_width = "32")]
  type Size = U4;

  #[cfg(target_pointer_width = "64")]
  type Size = U8;
}

/// A `Slot` may be a pointer.
impl<T> Slot for *mut T {
  type Size = <*const T as Slot>::Size;
}


macro_rules! primitive_layout {
  ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
    $(
      unsafe impl<R> Repr<R> for $ty
      where R: ReprMarker,
            Self: LayoutAlgorithm<R>
      {}

      impl CLayout for $ty {
        type Align = $align;
        type Repr = <u8 as Repeat<$size>>::Output;
      }
    )*
  };
}

primitive_layout! {
  u8   { size: U1,   align: U1  };
  u16  { size: U2,   align: U2  };
  u32  { size: U4,   align: U4  };
  u64  { size: U8,   align: U8  };
  u128 { size: U16,  align: U16 };
  i8   { size: U1,   align: U1  };
  i16  { size: U2,   align: U2  };
  i32  { size: U4,   align: U4  };
  i64  { size: U8,   align: U8  };
  i128 { size: U16,  align: U16 };
}

macro_rules! nonzero_layout {
  ($($ty: ty { size: $size: ty, align: $align: ty };)*) => {
    $(
      unsafe impl<R> Repr<R> for $ty
      where R: ReprMarker,
            Self: LayoutAlgorithm<R>
      {}

      impl CLayout for $ty {
        type Align = $align;
        type Repr = <NonZero as Repeat<$size>>::Output;
      }
    )*
  };
}

nonzero_layout! {
  NonZeroU8    { size: U1,   align: U1  };
  NonZeroU16   { size: U2,   align: U2  };
  NonZeroU32   { size: U4,   align: U4  };
  NonZeroU64   { size: U8,   align: U8  };
  NonZeroU128  { size: U16,  align: U16 };
  NonZeroI8    { size: U1,   align: U1  };
  NonZeroI16   { size: U2,   align: U2  };
  NonZeroI32   { size: U4,   align: U4  };
  NonZeroI64   { size: U8,   align: U8  };
  NonZeroI128  { size: U16,  align: U16 };
}

unsafe impl<'t, T, R> Repr<R> for &'t T
where R: ReprMarker, Self: LayoutAlgorithm<R> {}
unsafe impl<'t, T, R> Repr<R> for &'t mut T
where R: ReprMarker, Self: LayoutAlgorithm<R> {}
unsafe impl<R, T> Repr<R> for *const T
where R: ReprMarker, Self: LayoutAlgorithm<R> {}
unsafe impl<R, T> Repr<R> for *mut T
where R: ReprMarker, Self: LayoutAlgorithm<R> {}

unsafe impl<R: ReprMarker> Repr<R> for usize
where Self: LayoutAlgorithm<R> {}
unsafe impl<R: ReprMarker> Repr<R> for isize
where Self: LayoutAlgorithm<R> {}

unsafe impl<R: ReprMarker> Repr<R> for NonZeroUsize
where Self: LayoutAlgorithm<R>
{}
unsafe impl<R: ReprMarker> Repr<R> for NonZeroIsize
where Self: LayoutAlgorithm<R>
{}

/// Compute the layout characteristics of a given type, for a given algorithm.
pub trait LayoutAlgorithm<R> {
  type Align: Unsigned;
  /// The layout of this struct.
  type Repr: Representation;
}

impl<T: Generic> LayoutAlgorithm<C> for T
where
  <T as Generic>::Repr: CLayout,
{
  type Align = <<Self as Generic>::Repr as CLayout>::Align;
  type Repr = <<Self as Generic>::Repr as CLayout>::Repr;
}

pub trait CLayout {
  type Align : Unsigned;
  type Repr: Representation;
}

impl CLayout for usize {
  #[cfg(target_pointer_width = "32")]
  type Align = U4;

  #[cfg(target_pointer_width = "64")]
  type Align = U8;

  #[cfg(target_pointer_width = "32")]
  type Repr = <u8 as Repeat<U4>>::Output;

  #[cfg(target_pointer_width = "64")]
  type Repr = <u8 as Repeat<U8>>::Output;
}

impl CLayout for isize
{
  type Align = <usize as CLayout>::Align;
  type Repr = <usize as CLayout>::Repr;
}

impl CLayout for NonZeroUsize {
  #[cfg(target_pointer_width = "32")]
  type Align = U4;

  #[cfg(target_pointer_width = "64")]
  type Align = U8;

  #[cfg(target_pointer_width = "32")]
  type Repr = <NonZero as Repeat<U4>>::Output;

  #[cfg(target_pointer_width = "64")]
  type Repr = <NonZero as Repeat<U8>>::Output;
}

impl CLayout for NonZeroIsize
{
  type Align = <NonZeroUsize as CLayout>::Align;
  type Repr = <NonZeroUsize as CLayout>::Repr;
}

impl<'t, T> CLayout for &'t T
where
{
  type Align = <usize as CLayout>::Align;
  type Repr = Hlist!(&'t T);
}

impl<'t, T> CLayout for &'t mut T
where
{
  type Align = <usize as CLayout>::Align;
  type Repr = Hlist!(&'t mut T);
}

impl<T> CLayout for *const T
where
{
  type Align = <usize as CLayout>::Align;
  type Repr = Hlist!(*const T);
}

impl<T> CLayout for *mut T
where
{
  type Align = <usize as CLayout>::Align;
  type Repr = Hlist!(*const T);
}


/// A computer for the alignment of structs.
pub trait CStructAlign {
  type Align: Unsigned;
}

/// The alignment of `HNil` is 0.
impl CStructAlign for HNil {
  type Align = U0;
}

/// The alignment of `HCons<H, T>` is the maximum of the alignments of `H` and `T`.
impl<H, T> CStructAlign for HCons<H, T>
where
  H: TypeLayout,
  T: CStructAlign,
  <H as TypeLayout>::Align: Max<<T as CStructAlign>::Align>,
  Maximum<<H as TypeLayout>::Align, <T as CStructAlign>::Align>: Unsigned,
{
  type Align = Maximum<<H as TypeLayout>::Align, <T as CStructAlign>::Align>;
}

impl<H, T> CLayout for HCons<H, T>
where
  Self: CStructAlign + CStructLayout,
{
  /// Delegate the alignment computation to `CStructAlign`
  type Align = <Self as CStructAlign>::Align;

  /// Delegate the repr computation to `CStructLayout`
  type Repr = <Self as CStructLayout>::Repr;
}

/// Apply the `repr(C)` layout algorithm to find the representation of a struct.
pub trait CStructLayout<Alignment=<Self as CStructAlign>::Align, Offset=U0> {
    /// The representation of this struct.
    type Repr: Representation;
}

/// After the last field, insert trailing padding.
impl<Alignment, Offset> CStructLayout<Alignment, Offset> for HNil
where
    (Alignment, Offset): Padding,
    Uninit: Repeat<<(Alignment, Offset) as Padding>::Bytes>,

    <Uninit as Repeat<<(Alignment, Offset) as Padding>::Bytes>>::Output: Representation,
{
    type Repr = <Uninit as Repeat<<(Alignment, Offset) as Padding>::Bytes>>::Output;
}

impl<H, T, Alignment, Offset> CStructLayout<Alignment, Offset> for HCons<H, T>
where
    H: TypeLayout,
    (<H as TypeLayout>::Align, Offset): Padding,

    <(<H as TypeLayout>::Align, Offset) as Padding>::Repr:
        Add<<H as TypeLayout>::Repr>,

    Sum<
        <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
        <H as TypeLayout>::Repr,
    >: Representation,

    T: CStructLayout<
        Alignment,
        <Sum<
            <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
            <H as TypeLayout>::Repr,
        > as Representation>::Size,
    >,

    Sum<<(<H as TypeLayout>::Align, Offset) as Padding>::Repr, <H as TypeLayout>::Repr>: Add<
        <T as CStructLayout<
            Alignment,
            <Sum<
                <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
                <H as TypeLayout>::Repr,
            > as Representation>::Size,
        >>::Repr,
    >,

    Sum<
        // padding + `H` field repr
        Sum<
          // padding bytes
          <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
          // `H` repr bytes
          <H as TypeLayout>::Repr,
        >,
        // repr bytes of the rest of this structure
        <T as CStructLayout<
            Alignment,
            // the offset increases by (padding + `H` field repr) bytes.
            <Sum<
                <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
                <H as TypeLayout>::Repr,
            > as Representation>::Size,
        >>::Repr>:
    Representation,
{
    /// `[Uninit; offset % field.alignment] + [H repr] + [T repr]`
    type Repr =
      // repr of padded `H` + repr of the rest of this structure
      Sum<
        // padding + `H` field repr
        Sum<
          // padding bytes
          <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
          // `H` repr bytes
          <H as TypeLayout>::Repr,
        >,
        // repr bytes of the rest of this structure
        <T as CStructLayout<
            Alignment,
            // the offset increases by (padding + `H` field repr) bytes.
            <Sum<
                <(<H as TypeLayout>::Align, Offset) as Padding>::Repr,
                <H as TypeLayout>::Repr,
            > as Representation>::Size,
        >>::Repr,
    >;
}

/// A padding computer for the C layout.
pub trait Padding
{
  /// the number of trailing padding bytes
  type Bytes: Unsigned;

  /// the hlist representation of trailing padding bytes
  type Repr: Representation;
}

impl<Alignment, Offset> Padding for (Alignment, Offset)
where
  Offset: Rem<Alignment>,
  Mod<Offset, Alignment>: Unsigned,
  Uninit: Repeat<Mod<Offset, Alignment>>,
{
  type Bytes =
      Mod<
        Offset,
        Alignment>;

  type Repr =
    <Uninit as Repeat<Self::Bytes>>::Output;
}

/// Produce `N` repetitions of `Self`
pub trait Repeat<N>
where
  N: Unsigned,
{
  type Output: Representation;
}

impl<T> Repeat<UTerm> for T
where T: Slot,
{
  type Output = HNil;
}

impl<T, Ul: Unsigned, Bl: Bit> Repeat<UInt<Ul, Bl>> for T
where
  UInt<Ul, Bl>: Sub<B1>,
  Sub1<UInt<Ul, Bl>>: Unsigned,
  HCons<T, <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output>: Representation,
  T: Repeat<<UInt<Ul, Bl> as Sub<B1>>::Output>
{
  type Output =
    HCons<T,
      <T as Repeat<Sub1<UInt<Ul, Bl>>>>::Output>;
}