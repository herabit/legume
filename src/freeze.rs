use crate::Pointee;

/// Trait for types that contain no interior mutability, ignoring through indirection.
pub unsafe trait Freeze {}

unsafe impl<T: Freeze> Freeze for [T] {}
unsafe impl<T: Freeze, const N: usize> Freeze for [T; N] {}

// SAFETY: Pointers are indirection.

unsafe impl<T: Pointee + ?Sized> Freeze for *const T {}
unsafe impl<T: Pointee + ?Sized> Freeze for *mut T {}

unsafe impl<T: Pointee + ?Sized> Freeze for &T {}
unsafe impl<T: Pointee + ?Sized> Freeze for &mut T {}

unsafe impl<T: Pointee + ?Sized> Freeze for core::ptr::NonNull<T> {}

unsafe impl<T: Freeze> Freeze for core::num::Saturating<T> {}
unsafe impl<T: Freeze> Freeze for core::num::Wrapping<T> {}

unsafe impl<T: Freeze> Freeze for core::cmp::Reverse<T> {}

unsafe impl<T: Freeze + ?Sized> Freeze for core::mem::ManuallyDrop<T> {}
unsafe impl<T: Freeze> Freeze for core::mem::MaybeUninit<T> {}

unsafe impl<T: ?Sized> Freeze for core::marker::PhantomData<T> {}

unsafe impl<T: Freeze> Freeze for Option<T> {}
unsafe impl<T: Freeze, E: Freeze> Freeze for Result<T, E> {}

macro_rules! freeze {
    ($($ty:ty),*) => {
        $(unsafe impl Freeze for $ty {})*
    };
}

macro_rules! integer {
    ($($ty:ty),*) => {
        freeze!($($ty),*);
        freeze!($(core::num::NonZero<$ty>),*);
    };
}

integer!(u8, u16, u32, u64, u128, usize);
integer!(i8, i16, i32, i64, i128, isize);
freeze!(f32, f64);
freeze!(bool);
freeze!(char);
freeze!(str);
freeze!(core::cmp::Ordering);
freeze!(core::marker::PhantomPinned);

macro_rules! freeze_tuple {
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
            unsafe impl<$($name: Freeze $( $($rest)* )?),*> Freeze for ($($name,)*) {}
        )*
    };
}

freeze_tuple!(
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
