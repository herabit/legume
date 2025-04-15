use crate::zeroable::Zeroable;

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
