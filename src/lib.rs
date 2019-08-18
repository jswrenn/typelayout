pub extern crate typenum;
#[macro_use]
pub extern crate frunk;
extern crate frunk_core;

#[macro_use]
extern crate static_assertions;

mod layout;
use layout::*;

mod frombytes;
pub use frombytes::FromBytes;

mod alignedto;
pub use alignedto::AlignedTo;

pub use frunk::{Generic};

pub unsafe trait Repr<R: ReprMarker>: LayoutAlgorithm<R> + TypeLayout {}

/// Use a `repr(C)` packing rule.
pub struct C;

/// Use a `repr(packed)` packing rule.
pub struct Packed;

/// Use a `repr(transparent)` packing rule.
pub struct Transparent;

pub trait ReprMarker {}
impl ReprMarker for C {}
impl ReprMarker for Packed {}
impl ReprMarker for Transparent {}
