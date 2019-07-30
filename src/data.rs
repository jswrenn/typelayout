use crate::*;
use frunk_core::generic::Generic;

impl<T: Generic + ReprC> Align<C> for T
where
  <Self as Generic>::Repr: Align<C>
{
  type Value = <<Self as Generic>::Repr as Align<C>>::Value;
}

impl<T: Generic + ReprC> Size<C> for T
where
  <Self as Generic>::Repr: Size<C>
{
  type Value = <<Self as Generic>::Repr as Size<C>>::Value;
}
