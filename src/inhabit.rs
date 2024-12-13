use crate::array::{Length, NonEmpty};
use core::{
    cell::{Cell, RefCell, UnsafeCell},
    ffi::CStr,
    mem::{ManuallyDrop, MaybeUninit},
    num::NonZero,
    ops::ControlFlow,
    ptr::NonNull,
    sync::atomic,
    task::Poll,
};

#[cfg(feature = "alloc")]
#[cfg(target_has_atomic = "8")]
use atomic::{AtomicBool, AtomicI8, AtomicU8};

#[cfg(target_has_atomic = "16")]
use atomic::{AtomicI16, AtomicU16};

#[cfg(target_has_atomic = "32")]
use atomic::{AtomicI32, AtomicU32};

#[cfg(target_has_atomic = "64")]
use atomic::{AtomicI64, AtomicU64};

#[cfg(target_has_atomic = "ptr")]
use atomic::{AtomicIsize, AtomicPtr, AtomicUsize};

// #[cfg(target_has_atomic = "32")]
// use core::s

/// The "never" type ([`!`](never)). (yes, the actual never type)
///
/// This indicates that some type is uninhabited, and is itself uninhabited.
pub type Never = <fn() -> ! as inner::FnOnce>::Output;

/// The "always"/"unit" type ([`()`](unit)).
///
/// This indicates that some type is inhabited, and is itself inhabited.
pub type Always = ();

/// Marker trait for types that indicate the inhabitability of some type.
pub unsafe trait Inhabit:
    Inhabitability<Output = Self> + inner::Sealed + 'static + Send + Sync + Copy + Eq + Ord
{
    /// Determine the opposite.
    ///
    /// - [`Always`] => [`Never`]
    /// - [`Never`]  => [`Always`]
    type Not: Inhabit<Not = Self>;

    /// Determine whether both is inhabited.
    ///
    /// - [`Always`] & [`Always`] => [`Always`]
    /// - [`Always`] & [`Never`] => [`Never`]
    /// - [`Never`] & [`Always`] => [`Never`]
    /// - [`Never`] & [`Never`] => [`Never`]
    type And<T: Inhabit>: Inhabit;

    /// Determine whether at least one is inhabited.
    ///
    /// - [`Always`] | [`Always`] => [`Always`]
    /// - [`Always`] | [`Never`]  => [`Always`]
    /// - [`Never`] | [`Always`] => [`Always`]
    /// - [`Never`]  | [`Never`] => [`Never`]
    type Or<T: Inhabit>: Inhabit;

    /// Boolean indicating whether this is inhabited.
    const INHABITED: bool;
}

// pub trait InhabitExt: Inhabit {
//     /// Determine whether only one is inhabited.
//     ///
//     /// This returns [`Always`] if both are not equal.
//     ///
//     /// - [`Always`] ^ [`Always`] => [`Never`]
//     /// - [`Always`] ^ [`Never`] => [`Always`]
//     /// - [`Never`] ^ [`Always`] => [`Always`]
//     /// - [`Never`] ^ [`Never`] => [`Never`]
//     type Xor<T: Inhabit>: Inhabit;

//     /// Determine whether all or none is inhabited.
//     ///
//     /// This returns [`Always`] if both are equal.
//     ///
//     /// - [`Always`] ^ [`Always`] => [`Never`]
//     /// - [`Always`] ^ [`Never`] => [`Always`]
//     /// - [`Never`] ^ [`Always`] => [`Always`]
//     /// - [`Never`] ^ [`Never`] => [`Never`]
// }

/// Marker trait that indicates whether this type is inhabited or uninhabited.
pub unsafe trait Inhabitability {
    /// This is either [`Always`] or [`Never`].
    ///
    /// - [`Always`] indicates that this type is inhabited.
    /// - [`Never`] indicates that this type is uninhabited.
    type Output: Inhabit;
}

/// Marker trait that currently only indicates the [`Inhabitability`] of references.
///
/// This exists mostly to act as a stand-in for changing the inhabitability
/// of references and reference-like types for when the exact details
/// of whether they're inhabited is formalized.
///
/// Right now, it is not formalzied, so references of a type
/// inherit the same inhabitability.
pub unsafe trait InhabitabilityExt: Inhabitability {
    /// The inhabitability of references to [`Self`].
    type Ref: Inhabit;

    /// The inhabitability of mutable references to [`Self`].
    ///
    /// It is unlikely this will ever differ from `Ref`.
    type Mut: Inhabit;
}

unsafe impl<T: Inhabitability + ?Sized> InhabitabilityExt for T {
    type Ref = T::Output;
    type Mut = Self::Ref;
}

/// Marker trait that indicates that some type is inhabited.
pub trait Inhabited: Inhabitability<Output = Always> {}

/// Marker trait that indicates that some type is uninhabited.
pub trait Uninhabited: Inhabitability<Output = Never> {}

unsafe impl Inhabitability for Never {
    type Output = Never;
}

unsafe impl Inhabitability for Always {
    type Output = Always;
}

unsafe impl Inhabit for Never {
    type Not = Always;
    type And<T: Inhabit> = Never;
    type Or<T: Inhabit> = T;

    const INHABITED: bool = false;
}

unsafe impl Inhabit for Always {
    type Not = Never;
    type And<T: Inhabit> = T;
    type Or<T: Inhabit> = Always;

    const INHABITED: bool = true;
}

impl<T: Inhabitability<Output = Always> + ?Sized> Inhabited for T {}
impl<T: Inhabitability<Output = Never> + ?Sized> Uninhabited for T {}

// structs and tuples
macro_rules! product {
    () => {
        $crate::inhabit::Always
    };

    ($ty:ty $(, $rest:ty)* $(,)?) => {
        <<$ty as $crate::inhabit::Inhabitability>::Output as $crate::inhabit::Inhabit>::And<$crate::inhabit::product!($($rest),*)>
    }
}

#[allow(unused_imports)]
pub(crate) use product;

// enums, maybe unions
macro_rules! sum {
    () => {
        $crate::inhabit::Never
    };

    ($ty:ty $(, $rest:ty)* $(,)?) => {
        <<$ty as $crate::inhabit::Inhabitability>::Output as $crate::inhabit::Inhabit>::Or<$crate::inhabit::sum!($($rest),*)>
    }
}

#[allow(unused_imports)]
pub(crate) use sum;

macro_rules! inhabit {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl $crate::inhabit::Inhabitability for $ty {
                type Output = $crate::inhabit::Always;
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use inhabit;

macro_rules! uninhabit {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl $crate::inhabit::Inhabitability for $ty {
                type Output = $crate::inhabit::Never;
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use uninhabit;

// Fucking pain in the ass this will be.
#[cfg(not(feature = "nightly"))]
uninhabit!(core::convert::Infallible);

unsafe impl<T, E> Inhabitability for Result<T, E>
where
    T: Inhabitability,
    E: Inhabitability,
{
    type Output = sum!(T, E);
}

unsafe impl<B, C> Inhabitability for ControlFlow<B, C>
where
    B: Inhabitability,
    C: Inhabitability,
{
    type Output = sum!(B, C);
}

// `Option`s are always inhabited. Always, as it is more or less
// equivalent to a `Result<T, ()>`, and `()` is literally the `Always` type.
unsafe impl<T> Inhabitability for Option<T> {
    type Output = Always;
}

// `Poll` is essentially a specialized option.
unsafe impl<T> Inhabitability for Poll<T> {
    type Output = Always;
}

// Arrays of length `0` are always inhabited as it is essentially equivalent to
// the unit type (but with the alignment of `T` ig).
unsafe impl<T> Inhabitability for [T; 0] {
    type Output = Always;
}

// Arrays of lengths besides `0` are only
// inhabited if their element is.
unsafe impl<T: Inhabitability, const N: usize> Inhabitability for [T; N]
where
    Length<N>: NonEmpty,
{
    type Output = <T as Inhabitability>::Output;
}

// I checked rustc, slices are always considered to be inhabited.
unsafe impl<T> Inhabitability for [T] {
    type Output = Always;
}

// Most of our marker types are in fact inhabited.
unsafe impl<const N: usize> Inhabitability for Length<N> {
    type Output = Always;
}

// Pointers are always inhabited
unsafe impl<T: ?Sized> Inhabitability for *const T {
    type Output = Always;
}
unsafe impl<T: ?Sized> Inhabitability for *mut T {
    type Output = Always;
}

unsafe impl<T: ?Sized> Inhabitability for NonNull<T> {
    type Output = Always;
}

// It is not yet specified whether references are always
// inhabited or not, so I'll be safe and say that
// references (and smart pointers that over some valid T),
// are only inhabited if their pointee (T) is.
//
// Note that for types such as `Vec<T>`, it is considered
// inhabited as their pointee is not actually `T` but `[T]`.
//
// For weak references that can be safely constructed without
// an instance of `T`, they're also always considered inhabited,
// as they act more as `Option<T>` than `T`.
unsafe impl<T: Inhabitability + ?Sized> Inhabitability for &T {
    type Output = <T as InhabitabilityExt>::Ref;
}

unsafe impl<T: Inhabitability + ?Sized> Inhabitability for &mut T {
    type Output = <T as InhabitabilityExt>::Mut;
}

#[cfg(feature = "alloc")]
mod _alloc {
    use super::*;

    use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
        ffi::CString,
        rc::{self, Rc},
        string::String,
        sync::{self, Arc},
        vec::Vec,
    };

    unsafe impl<T> Inhabitability for Vec<T> {
        type Output = Always;
    }

    unsafe impl<T> Inhabitability for VecDeque<T> {
        type Output = Always;
    }

    unsafe impl<T> Inhabitability for BinaryHeap<T> {
        type Output = Always;
    }

    unsafe impl<T> Inhabitability for LinkedList<T> {
        type Output = Always;
    }

    unsafe impl<K, V> Inhabitability for BTreeMap<K, V> {
        type Output = Always;
    }

    unsafe impl<T> Inhabitability for BTreeSet<T> {
        type Output = Always;
    }

    unsafe impl<T: Inhabitability + ?Sized> Inhabitability for Box<T> {
        type Output = <T as InhabitabilityExt>::Mut;
    }

    unsafe impl<T: Inhabitability + ?Sized> Inhabitability for Rc<T> {
        type Output = <T as InhabitabilityExt>::Ref;
    }

    unsafe impl<T: ?Sized> Inhabitability for rc::Weak<T> {
        type Output = Always;
    }

    unsafe impl<T: Inhabitability + ?Sized> Inhabitability for Arc<T> {
        type Output = <T as InhabitabilityExt>::Ref;
    }

    unsafe impl<T: ?Sized> Inhabitability for sync::Weak<T> {
        type Output = Always;
    }

    unsafe impl Inhabitability for String {
        type Output = Always;
    }

    unsafe impl Inhabitability for CString {
        type Output = Always;
    }

    unsafe impl<'a, B> Inhabitability for Cow<'a, B>
    where
        B: 'a + ToOwned + ?Sized + Inhabitability,
        B::Owned: Inhabitability,
    {
        type Output = sum!(&'a B, B::Owned);
    }
}

// Primitives besides `never` are always inhabited.
inhabit!(u8, u16, u32, u64, u128, usize);
inhabit!(i8, i16, i32, i64, i128, isize);
inhabit!(f32, f64);
inhabit!(bool, char, str);

inhabit!(
    NonZero<u8>,
    NonZero<u16>,
    NonZero<u32>,
    NonZero<u64>,
    NonZero<u128>,
    NonZero<usize>
);

inhabit!(
    NonZero<i8>,
    NonZero<i16>,
    NonZero<i32>,
    NonZero<i64>,
    NonZero<i128>,
    NonZero<isize>
);

// Atomics are always inhabited.
inhabit!(atomic::Ordering);

#[cfg(target_has_atomic = "8")]
inhabit!(AtomicBool, AtomicI8, AtomicU8);

#[cfg(target_has_atomic = "16")]
inhabit!(AtomicI16, AtomicU16);

#[cfg(target_has_atomic = "32")]
inhabit!(AtomicI32, AtomicU32);

#[cfg(target_has_atomic = "64")]
inhabit!(AtomicI64, AtomicU64);

#[cfg(target_has_atomic = "ptr")]
inhabit!(AtomicIsize, AtomicUsize);

#[cfg(target_has_atomic = "ptr")]
unsafe impl<T> Inhabitability for AtomicPtr<T> {
    type Output = Always;
}

inhabit!(CStr);

// Interior mutability types are just wrappers around `T`, so they are
// only inhabited if T is.

unsafe impl<T: Inhabitability + ?Sized> Inhabitability for UnsafeCell<T> {
    type Output = T::Output;
}

unsafe impl<T: Inhabitability + ?Sized> Inhabitability for Cell<T> {
    type Output = T::Output;
}

unsafe impl<T: Inhabitability + ?Sized> Inhabitability for RefCell<T> {
    type Output = T::Output;
}

// `MaybeUninit<T>` is always inhabited when uninitialized.
unsafe impl<T> Inhabitability for MaybeUninit<T> {
    type Output = Always;
}

// `ManuallyDrop` is a transparent wrapper over `T`, so it is only inhabited
// if `T` is.
unsafe impl<T: Inhabitability + ?Sized> Inhabitability for ManuallyDrop<T> {
    type Output = T::Output;
}

mod inner {
    pub trait FnOnce {
        type Output: ?Sized;
    }

    impl<R: ?Sized> FnOnce for fn() -> R {
        type Output = R;
    }

    pub trait Sealed {}

    impl Sealed for super::Never {}
    impl Sealed for super::Always {}
}
