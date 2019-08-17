//! ## Example
//! ```rust
//! use typelayout::*;
//! use core::mem;
//!
//! #[derive(Generic)]
//! #[repr(C)]
//! pub struct Struct {
//!   first: u8,
//!   second: u32,
//! }
//!
//! // `Layout` is only implemented for `Struct` if it is `ReprC`.
//! unsafe impl Repr<C> for Struct {}
//!
//! assert_eq!(mem::align_of::<Struct>(), Struct::ALIGN); // 4
//! assert_eq!(mem::size_of::<Struct>(), Struct::SIZE);   // 8
//! ```
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
