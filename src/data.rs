use crate::*;
use frunk_core::generic::Generic;
use core::marker::PhantomData;

/// A generic representation of a struct + packing rule.
#[allow(dead_code)]
pub struct Struct<F, P> {
  data: PhantomData<F>,
  pack: PhantomData<P>,
}

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
