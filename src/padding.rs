use crate::*;
use typenum::*;
use frunk::generic::Generic;

pub trait Padding {
  type Value: Bit;
}

impl<T: Generic> Padding for T
where
  Self: Size<C>,
  <Self as Generic>::Repr: Size<Packed>,

  <Self as Size<C>>::Value
    : IsNotEqual<<<Self as Generic>::Repr as Size<Packed>>::Value>,
{
  /// A type has padding if the `C` and `packed` layout algorithms produce
  /// differently sized layouts.
  type Value =
    NotEq<
      <Self as Size<C>>::Value,
      <<Self as Generic>::Repr as Size<Packed>>::Value>;
}

/// Marker trait for types without any padding bytes.
pub unsafe trait NoPadding {}

unsafe impl<T: Padding<Value=False>> NoPadding for T {}
