use crate::*;
use typenum::Unsigned;

/// The layout of a type.
pub trait Layout {
  /// The size of the type, in bytes.
  const SIZE: usize = <Self as Layout>::Size::USIZE;

  /// The alignment of the type, in bytes.
  const ALIGN: usize = <Self as Layout>::Align::USIZE;

  /// The size of the type, in bytes.
  type Size: Unsigned;

  /// The alignment of the type, in bytes.
  type Align: Unsigned;
}

impl<T: Align<C> + Size<C>> Layout for T
{
  type Size = <Self as Size<C>>::Value;
  type Align = <Self as Align<C>>::Value;
}
