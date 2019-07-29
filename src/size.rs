use std::ops::*;
use typenum::*;
use frunk::hlist::*;
use crate::*;

/// Types for which the size is known at type-checking time.
pub trait Size<R> {
  /// The size of a type, in bytes.
  type Value: Unsigned;
}

impl<R> Size<R> for ()   { type Value = U0;  }
impl<R> Size<R> for f32  { type Value = U4;  }
impl<R> Size<R> for f64  { type Value = U8;  }
impl<R> Size<R> for i8   { type Value = U1;  }
impl<R> Size<R> for i16  { type Value = U2;  }
impl<R> Size<R> for i32  { type Value = U4;  }
impl<R> Size<R> for i64  { type Value = U8;  }
impl<R> Size<R> for i128 { type Value = U16; }
impl<R> Size<R> for u8   { type Value = U1;  }
impl<R> Size<R> for u16  { type Value = U2;  }
impl<R> Size<R> for u32  { type Value = U4;  }
impl<R> Size<R> for u64  { type Value = U8;  }
impl<R> Size<R> for u128 { type Value = U16; }

impl<R> Size<R> for HNil {
  /// The size of the nullary structure is one.
  type Value = U0;
}

impl<H, Tail> Size<Packed> for HCons<H, Tail>
where
  H: Size<Packed>,
  Tail: Size<Packed>,

  <H as Size<Packed>>::Value: Add<<Tail as Size<Packed>>::Value>,
  Sum<<H as Size<Packed>>::Value, <Tail as Size<Packed>>::Value>: Unsigned,
{
  type Value = Sum<<H as Size<Packed>>::Value, <Tail as Size<Packed>>::Value>;
}

impl<H, Tail> Size<C> for HCons<H, Tail>
where
  Self: SizeHelper + Align<C>,
  H: Align<C> + Size<C>,

  <Self as SizeHelper>::Value: Unsigned,

  <Self as SizeHelper>::Value:
    Add<
      Mod<
        <Self as SizeHelper>::Value,
        <Self as Align<C>>::Value>>,

  <Self as SizeHelper>::Value:
    Rem<<Self as Align<C>>::Value>,

  Sum<
    <Self as SizeHelper>::Value,
    Mod<
      <Self as SizeHelper>::Value,
      <Self as Align<C>>::Value>>:
    Unsigned,

{
  // size = current_offset + (current_offset % struct.alignment);
  type Value =
    Sum<
      <Self as SizeHelper>::Value,
      Mod<
        <Self as SizeHelper>::Value,
        <Self as Align<C>>::Value,
      >
    >;
}

pub trait SizeHelper
where
{
    type Value : Unsigned;
}

impl SizeHelper for HNil
where
{
    type Value = U0;
}

/// N % M, or 0 if M == 0.
pub trait TotalRem<Rhs> {
  type Output;
}

type TotalMod<A, B> = <A as TotalRem<B>>::Output;

impl<Ul: Unsigned, Bl: Bit, Ur: Unsigned, Br: Bit> TotalRem<UInt<Ur, Br>> for UInt<Ul, Bl>
where
    Self: Rem<UInt<Ur, Br>>
{
  type Output = Mod<Self, UInt<Ur, Br>>;
}

impl<Ul: Unsigned, Bl: Bit> TotalRem<UTerm> for UInt<Ul, Bl>
where
{
  type Output = U0;
}


impl<H, Tail> SizeHelper for HCons<H, Tail>
where
  H: Align<C> + Size<C>,
  Tail: SizeHelper,
  <H as Align<C>>::Value: TotalRem<<Tail as SizeHelper>::Value>,
  TotalMod<<H as Align<C>>::Value, <Tail as SizeHelper>::Value>: Add<<H as Size<C>>::Value>,
  Sum<TotalMod<<H as Align<C>>::Value, <Tail as SizeHelper>::Value>, <H as Size<C>>::Value>: Add<<Tail as SizeHelper>::Value>,
  Sum<Sum<TotalMod<<H as Align<C>>::Value, <Tail as SizeHelper>::Value>, <H as Size<C>>::Value>, <Tail as SizeHelper>::Value>: Unsigned
{
  type Value =
    Sum<
      Sum<
        TotalMod<
          <H as Align<C>>::Value,
          <Tail as SizeHelper>::Value
        >,
        <H as Size<C>>::Value
      >,
      <Tail as SizeHelper>::Value
    >;
}
