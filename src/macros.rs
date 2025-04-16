#![allow(unused_imports, unused_macro_rules, unused_macros)]

macro_rules! impl_for_atomic {
    (
        $(unsafe $($unsafe:lifetime)?)? impl $trait:path
    ) => {
        #[cfg(target_has_atomic = "8")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicBool {}
        #[cfg(target_has_atomic = "8")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicU8 {}
        #[cfg(target_has_atomic = "8")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicI8 {}


        #[cfg(target_has_atomic = "16")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicU16 {}
        #[cfg(target_has_atomic = "16")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicI16 {}


        #[cfg(target_has_atomic = "32")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicU32 {}
        #[cfg(target_has_atomic = "32")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicI32 {}


        #[cfg(target_has_atomic = "64")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicU64 {}
        #[cfg(target_has_atomic = "64")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicI64 {}


        #[cfg(target_has_atomic = "ptr")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicUsize {}
        #[cfg(target_has_atomic = "ptr")]
        $(unsafe $($unsafe)?)? impl $trait for ::core::sync::atomic::AtomicIsize {}

        #[cfg(target_has_atomic = "ptr")]
        $(unsafe $($unsafe)?)? impl<T> $trait for ::core::sync::atomic::AtomicPtr<T> {}
    };
}

pub(crate) use impl_for_atomic;
