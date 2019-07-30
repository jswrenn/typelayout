//! Type-level layout information.
//! ## Example
//! ```rust
//! use typelayout::{ReprC, Generic, Layout};
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
//! unsafe impl ReprC for Struct {}
//!
//! assert_eq!(mem::align_of::<Struct>(), Struct::ALIGN); // 4
//! assert_eq!(mem::size_of::<Struct>(), Struct::SIZE);   // 8
//! ```
pub extern crate typenum;
pub extern crate frunk;
extern crate frunk_core;

mod align;
use crate::align::*;

mod data;

mod layout;
pub use crate::layout::Layout;

mod packing;
use crate::packing::*;

mod padding;
pub use padding::NoPadding;

mod size;
use crate::size::*;

pub use frunk::Generic;

mod fromzeros;
pub use fromzeros::FromZeros;

/// A marker trait for types that are ReprC
pub unsafe trait ReprC {}
