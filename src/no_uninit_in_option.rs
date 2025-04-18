use crate::no_uninit::NoUninit;

/// Trait for types that contain no uninitialized in an option.
pub unsafe trait NoUninitInOption: Sized + NoUninit {}

unsafe impl<T: NoUninitInOption> NoUninit for Option<T> {}
