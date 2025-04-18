use crate::{macros::impl_for_atomic, Pointee};

// #[doc(inline)]
// pub use crate::no_uninit_in_option::*;

/// Trait for types that contain no uninitialized bytes.
///
/// This trait makes no guarantees about whether these bytes can be safely accessed,
/// for additional guarantees use the [`crate::Freeze`] trait.
pub unsafe trait NoUninit {}

unsafe impl<T: NoUninit> NoUninit for [T] {}
unsafe impl<T: NoUninit, const N: usize> NoUninit for [T; N] {}

// SAFETY: According to [workingjubilee](https://github.com/workingjubilee) when talking with them,
//         the implementations for these pointer types should be sound.

unsafe impl<T: Pointee + ?Sized> NoUninit for *const T {}
unsafe impl<T: Pointee + ?Sized> NoUninit for *mut T {}

unsafe impl<T: Pointee + ?Sized> NoUninit for &T {}
unsafe impl<T: Pointee + ?Sized> NoUninit for &mut T {}

unsafe impl<T: Pointee + ?Sized> NoUninit for core::ptr::NonNull<T> {}

unsafe impl<T: NoUninit> NoUninit for core::num::Saturating<T> {}
unsafe impl<T: NoUninit> NoUninit for core::num::Wrapping<T> {}

unsafe impl<T: NoUninit> NoUninit for core::cmp::Reverse<T> {}

unsafe impl<T: NoUninit + ?Sized> NoUninit for core::mem::ManuallyDrop<T> {}

unsafe impl<T: NoUninit + ?Sized> NoUninit for core::cell::UnsafeCell<T> {}
unsafe impl<T: NoUninit + ?Sized> NoUninit for core::cell::Cell<T> {}

unsafe impl<T: ?Sized> NoUninit for core::marker::PhantomData<T> {}

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
no_uninit!(str);
no_uninit!(core::cmp::Ordering);
no_uninit!(core::marker::PhantomPinned);

impl_for_atomic!(unsafe impl NoUninit);
