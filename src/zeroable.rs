use crate::{macros::impl_for_atomic, ptr::Pointee, util};

// #[doc(inline)]
// pub use crate::zeroable_in_option::*;

/// Create a new `T` filled with all zeroes.
#[inline]
#[must_use]
pub const fn zeroed<T: Zeroable>() -> T {
    unsafe { core::mem::zeroed() }
}

/// Fill some `T` filled with all zeroes.
#[inline]
#[must_use]
pub const fn fill_zeroes<T: Zeroable + ?Sized>(dst: &mut T) -> &mut T {
    unsafe { util::memset(dst, 0) }

    dst
}

/// Trait for types that can be filled with all zeroes.
///
/// Implementors must be inhabited.
pub unsafe trait Zeroable {}

unsafe impl<T: Zeroable> Zeroable for [T] {}
unsafe impl<T: Zeroable, const N: usize> Zeroable for [T; N] {}

unsafe impl<T: Pointee<Metadata: Zeroable> + ?Sized> Zeroable for *const T {}
unsafe impl<T: Pointee<Metadata: Zeroable> + ?Sized> Zeroable for *mut T {}

unsafe impl<T: Zeroable> Zeroable for core::num::Saturating<T> {}
unsafe impl<T: Zeroable> Zeroable for core::num::Wrapping<T> {}

unsafe impl<T: Zeroable> Zeroable for core::cmp::Reverse<T> {}

unsafe impl<T: Zeroable + ?Sized> Zeroable for core::mem::ManuallyDrop<T> {}
unsafe impl<T> Zeroable for core::mem::MaybeUninit<T> {}

unsafe impl<T: Zeroable + ?Sized> Zeroable for core::cell::UnsafeCell<T> {}
unsafe impl<T: Zeroable + ?Sized> Zeroable for core::cell::Cell<T> {}

unsafe impl<T: ?Sized> Zeroable for core::marker::PhantomData<T> {}

macro_rules! zeroable {
    ($($ty:ty),*) => {
        $(
            unsafe impl Zeroable for $ty {}
        )*
    };
}

zeroable!(u8, u16, u32, u64, u128, usize);
zeroable!(i8, i16, i32, i64, i128, isize);
zeroable!(f32, f64);
zeroable!(bool);
zeroable!(char);
zeroable!(str);
zeroable!(core::cmp::Ordering);
zeroable!(core::marker::PhantomPinned);

macro_rules! zeroable_tuple {
    (
        $(
            (
                $( $name:ident
                    $( : ( $( $rest:tt )* ) )?
                ),*
            )
        ),* $(,)?
    ) => {
        $(
            unsafe impl<$($name: Zeroable $( $($rest)* )? ),*> Zeroable for ($($name,)*) {}
        )*
    };
}

zeroable_tuple!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6, T7: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5, T6: (+ ?Sized)),
    (T0, T1, T2, T3, T4, T5: (+ ?Sized)),
    (T0, T1, T2, T3, T4: (+ ?Sized)),
    (T0, T1, T2, T3: (+ ?Sized)),
    (T0, T1, T2: (+ ?Sized)),
    (T0, T1: (+ ?Sized)),
    (T0: (+ ?Sized)),
    (),
);

impl_for_atomic!(unsafe impl Zeroable);

/// Trait that is automatically implemented for all [`Zeroable`] types,
/// providing some
pub trait ZeroableExt: Zeroable {
    /// Calls [`zeroed`].
    #[inline]
    #[must_use]
    fn zeroed() -> Self
    where
        Self: Sized,
    {
        zeroed()
    }

    /// Calls [`fill_zeroes`].
    #[inline]
    #[must_use]
    fn fill_zeroes(&mut self) -> &mut Self {
        fill_zeroes(self)
    }
}

impl<T: Zeroable + ?Sized> ZeroableExt for T {}

/// Trait for types that when wrapped in an [`Option`] are zeroable.
pub unsafe trait ZeroableInOption: Sized {}

unsafe impl<T: ZeroableInOption> Zeroable for Option<T> {}

macro_rules! nonzero {
    ($($prim:ident),*) => {
        $(
            unsafe impl ZeroableInOption for core::num::NonZero<$prim> {}
        )*
    };
}

nonzero!(u8, u16, u32, u64, u128, usize);
nonzero!(i8, i16, i32, i64, i128, isize);

unsafe impl<T: ZeroableInOption> ZeroableInOption for core::mem::ManuallyDrop<T> {}

unsafe impl<T: ZeroableInOption> ZeroableInOption for core::num::Saturating<T> {}
unsafe impl<T: ZeroableInOption> ZeroableInOption for core::num::Wrapping<T> {}

unsafe impl<T: ZeroableInOption> ZeroableInOption for core::cmp::Reverse<T> {}

unsafe impl<T: ?Sized> ZeroableInOption for core::ptr::NonNull<T> {}
unsafe impl<T: ?Sized> ZeroableInOption for &T {}
unsafe impl<T: ?Sized> ZeroableInOption for &mut T {}
