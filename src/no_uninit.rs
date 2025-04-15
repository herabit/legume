use crate::Pointee;

/// Trait for types that contain no uninitialized bytes.
pub unsafe trait NoUninit {}

unsafe impl<T: NoUninit> NoUninit for [T] {}
unsafe impl<T: NoUninit, const N: usize> NoUninit for [T; N] {}

unsafe impl<T: Pointee + ?Sized> NoUninit for *const T {}
unsafe impl<T: Pointee + ?Sized> NoUninit for *mut T {}

unsafe impl<T: Pointee + ?Sized> NoUninit for &T {}
unsafe impl<T: Pointee + ?Sized> NoUninit for &mut T {}

unsafe impl<T: Pointee + ?Sized> NoUninit for core::ptr::NonNull<T> {}

unsafe impl<T: NoUninit> NoUninit for core::num::Saturating<T> {}
unsafe impl<T: NoUninit> NoUninit for core::num::Wrapping<T> {}

macro_rules! no_uninit {
    ($($ty:ty),*) => {
        $(
            unsafe impl NoUninit for $ty {}
        )*
    };
}

macro_rules! integer {
    ($($ty:ty),*) => {
        no_uninit!($($ty),*);
        no_uninit!($(core::num::NonZero<$ty>),*);
    };
}

no_uninit!(());
integer!(u8, u16, u32, u64, u128, usize);
integer!(i8, i16, i32, i64, i128, isize);
no_uninit!(f32, f64);
no_uninit!(bool);
no_uninit!(char);
no_uninit!(core::cmp::Ordering);
no_uninit!(core::marker::PhantomPinned);
