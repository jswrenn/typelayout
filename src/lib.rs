//! Type-level layout information.
//! ## Example
//! ```rust
//! use typelayout::{ReprC, Generic, Layout};
//!
//! #[derive(Generic)]
//! #[repr(C)]
//! pub struct Tree {
//!   height: u8,
//!   age: u32,
//! }
//!
//! unsafe impl ReprC for Tree {}
//!
//! assert_eq!(4, <Tree as Layout>::ALIGN);
//! assert_eq!(8, <Tree as Layout>::SIZE);
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
