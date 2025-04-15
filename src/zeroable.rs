use crate::{ptr::Pointee, util};
use core::{
    cell::{Cell, UnsafeCell},
    cmp::{Ordering, Reverse},
    marker::{PhantomData, PhantomPinned},
    mem::{ManuallyDrop, MaybeUninit},
    num::{Saturating, Wrapping},
};

#[doc(inline)]
pub use crate::zeroable_in_option::*;

#[inline]
#[must_use]
pub const fn zeroed<T: Zeroable>() -> T {
    unsafe { core::mem::zeroed() }
}

#[inline]
#[must_use]
pub const fn write_zeroes<T: Zeroable + ?Sized>(dst: &mut T) {
    unsafe { util::memset(dst, 0) }
}

/// Trait for types that can be filled with all zeroes.
///
/// Implementors must be inhabited.
pub unsafe trait Zeroable {}

unsafe impl<T: Zeroable> Zeroable for [T] {}
unsafe impl<T: Zeroable, const N: usize> Zeroable for [T; N] {}

unsafe impl<T: Pointee<Metadata: Zeroable> + ?Sized> Zeroable for *const T {}
unsafe impl<T: Pointee<Metadata: Zeroable> + ?Sized> Zeroable for *mut T {}

unsafe impl<T: Zeroable> Zeroable for Saturating<T> {}
unsafe impl<T: Zeroable> Zeroable for Wrapping<T> {}

unsafe impl<T: Zeroable> Zeroable for Reverse<T> {}

unsafe impl<T: Zeroable + ?Sized> Zeroable for ManuallyDrop<T> {}
unsafe impl<T> Zeroable for MaybeUninit<T> {}

unsafe impl<T: Zeroable + ?Sized> Zeroable for UnsafeCell<T> {}
unsafe impl<T: Zeroable + ?Sized> Zeroable for Cell<T> {}

unsafe impl<T: ?Sized> Zeroable for PhantomData<T> {}

macro_rules! zeroable {
    ($($prim:ident),*) => {
        $(
            unsafe impl Zeroable for $prim {}
        )*
    };
}

zeroable!(u8, u16, u32, u64, u128, usize);
zeroable!(i8, i16, i32, i64, i128, isize);
zeroable!(f32, f64);
zeroable!(bool);
zeroable!(char);
zeroable!(str);
zeroable!(Ordering);
zeroable!(PhantomPinned);

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
