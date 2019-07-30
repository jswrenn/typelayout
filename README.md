# typelayout

[**Documentation**](https://docs.rs/typelayout)

**This is an experiment and a work-in-progress. The implementation of the `repr(C)` layout algorithm has not been thoroughly tested.**

An experiment in embedding layout computations in the type system. This crate encodes the [layout algorithm for `repr(C)` structs](https://doc.rust-lang.org/reference/type-layout.html#reprc-structs) as a type-level computation, using [frunk](https://github.com/lloydmeta/frunk) to compute over the type structure of structs and [typenum](https://github.com/paholg/typenum) to perform the calculations. The `Layout` trait is implemented for types that know their own size at type-checking time. For instance:

```rust
use typelayout::{ReprC, Generic, Layout};
use core::mem;

#[derive(Generic)]
#[repr(C)]
pub struct Struct {
  first: u8,
  second: u32,
}

unsafe impl ReprC for Struct {}

assert_eq!(mem::align_of::<Struct>(), Struct::ALIGN); // 4
assert_eq!(mem::size_of::<Struct>(), Struct::SIZE);   // 8
```

## Layout Invariants

The purpose of this experiment is to express type layout invariants in the typesystem to enable safe abstractions for unsafe code that relies on layout.

For instance, [`mem::zeroed()`](https://doc.rust-lang.org/core/mem/fn.zeroed.html) is only valid to call on types for which a sequence of zeroed bits is a valid instance of the type. This function is unsafe to use to initialize structures that have padding bits, since rust is free to assume that padding bits have a _particular_ value.

This library's `NoPadding` trait is implemented for types in which the `#[repr(packed)]` layout algorithm and `#[repr(C)]` algorithm produce layouts of exactly the same size. Using this, we implement a `FromZeros` trait for structs meeting the criteria of being safe to initialize with `mem::zeroed`:

```rust
unsafe impl<T: Generic + ReprC> FromZeros for T
where
  T: NoPadding,
  Struct<<Self as Generic>::Repr>: FromZeros,
{}
```

Given this implementation, **this will compile**:

```rust
use typelayout::{ReprC, Generic, Layout, FromZeros};

#[derive(Generic, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Struct {
  first: u8,
  second: u8,
}

unsafe impl ReprC for Struct {}

assert_eq!(<Struct as Default>::default(), <Struct as FromZeros>::zeroed());
```

...but **this will not**:

```rust
use typelayout::{ReprC, Generic, Layout, FromZeros};

#[derive(Generic, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Struct {
  first: u8,
  second: u16, // padding will be inserted between `first` and `second`
}

unsafe impl ReprC for Struct {}

// `Struct` does not implement `FromZeros`, because it has a padding byte!
assert_eq!(<Struct as Default>::default(), <Struct as FromZeros>::zeroed());
```
