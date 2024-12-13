//! The [`Forget`] trait exists to fill the gap between [`Copy`], [`!Drop`] and [`needs_drop`].
//!
//! It seeks to act as a hint to a programmer that calling [`drop`] and [`forget`]
//! results in the same behavior.
//!
//! This is not a guarantee, however, it is a mere ***hint*** that `T`'s destructor (if any) can
//! simply be disregarded.
//!
//! ## Safety
//!
//! Since the [`Forget`] trait cannot actually guarantee that a given `T` has no destructor,
//! and [`needs_drop`] can return `true` for a `T` that doesn't have a destructor,
//! users of this API should not rely on [`Forget`] to prove some kind of safety invariants.
//!
//! It is a **suggestion** that something probably doesn't drop, but not a guarantee.
//!
//! See the docs of [`forget`] for more information.
//!
//! [`forget`]: core::mem::forget
//! [`drop`]: core::mem::drop
//! [`needs_drop`]: core::mem::needs_drop
//! [`!Drop`]: core::ops::Drop
//! [`Copy`]: core::marker::Copy
//! [`Forget`]: crate::forget::Forget

use core::{
    cmp::{Ordering, Reverse},
    ffi::CStr,
    marker::{PhantomData, PhantomPinned},
    num::{NonZero, Saturating, Wrapping},
    ops::ControlFlow,
    ptr::NonNull,
    task::Poll,
};

use crate::macros::{impl_marker_fn, impl_marker_tuple};

/// A marker trait that suggests that a type has no destructor.
///
/// See the [`module-level documentation`](crate::forget) for more info
/// on how to safely and effectively use this trait.
pub trait Forget {}

impl Forget for u8 {}
impl Forget for u16 {}
impl Forget for u32 {}
impl Forget for u64 {}
impl Forget for u128 {}
impl Forget for usize {}

impl Forget for i8 {}
impl Forget for i16 {}
impl Forget for i32 {}
impl Forget for i64 {}
impl Forget for i128 {}
impl Forget for isize {}

impl Forget for f32 {}
impl Forget for f64 {}

impl Forget for bool {}
impl Forget for char {}
impl Forget for str {}

impl Forget for NonZero<u8> {}
impl Forget for NonZero<u16> {}
impl Forget for NonZero<u32> {}
impl Forget for NonZero<u64> {}
impl Forget for NonZero<u128> {}
impl Forget for NonZero<usize> {}

impl Forget for NonZero<i8> {}
impl Forget for NonZero<i16> {}
impl Forget for NonZero<i32> {}
impl Forget for NonZero<i64> {}
impl Forget for NonZero<i128> {}
impl Forget for NonZero<isize> {}

impl<T: Forget> Forget for Saturating<T> {}
impl<T: Forget> Forget for Wrapping<T> {}

impl<T: Forget> Forget for [T] {}
impl<T: Forget, const N: usize> Forget for [T; N] {}

impl<T: ?Sized> Forget for *const T {}
impl<T: ?Sized> Forget for *mut T {}
impl<T: ?Sized> Forget for NonNull<T> {}
impl<T: ?Sized> Forget for &T {}
impl<T: ?Sized> Forget for &mut T {}

impl<T: Forget> Forget for Option<T> {}
impl<T: Forget, E: Forget> Forget for Result<T, E> {}
impl<B: Forget, C: Forget> Forget for ControlFlow<B, C> {}
impl<T: Forget> Forget for Poll<T> {}

impl Forget for Ordering {}
impl<T: Forget> Forget for Reverse<T> {}

impl<T: ?Sized> Forget for PhantomData<T> {}
impl Forget for PhantomPinned {}

impl Forget for CStr {}

impl_marker_tuple!(impl Forget);
impl_marker_fn!(impl Forget);
