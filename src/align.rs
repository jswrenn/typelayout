use typenum::*;
use frunk::hlist::*;
use crate::*;

/// A computer for the alignment of a type.
pub trait Align<R> {
  /// The alignment of a type, in bytes.
  type Value: Unsigned;
}

impl<R> Align<R> for ()   { type Value = U1;  }

impl<R> Align<R> for f32  { type Value = U4;  }
impl<R> Align<R> for f64  { type Value = U8;  }

impl<R> Align<R> for i8   { type Value = U1;  }
impl<R> Align<R> for i16  { type Value = U2;  }
impl<R> Align<R> for i32  { type Value = U4;  }
impl<R> Align<R> for i64  { type Value = U8;  }
impl<R> Align<R> for i128 { type Value = U16; }

impl<R> Align<R> for u8   { type Value = U1;  }
impl<R> Align<R> for u16  { type Value = U2;  }
impl<R> Align<R> for u32  { type Value = U4;  }
impl<R> Align<R> for u64  { type Value = U8;  }
impl<R> Align<R> for u128 { type Value = U16; }

impl Align<C> for HNil {
  type Value = U0;
}

impl<H, Tail> Align<C> for HCons<H, Tail>
where
  H: Align<C>,
  Tail: Align<C>,

  <H as Align<C>>::Value: Max<<Tail as Align<C>>::Value>,
  Maximum<<H as Align<C>>::Value, <Tail as Align<C>>::Value>: Unsigned,
{
  /// The alignment of a struct is the maximum alignment of its fields.
  type Value = Maximum<<H as Align<C>>::Value, <Tail as Align<C>>::Value>;
}
