use core::{
    fmt, hash,
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut},
};

use crate::{Freeze, NoUninit, Zeroable};

/// Trait for types that have no alignment requirement.
pub unsafe trait Unaligned {}

unsafe impl<T: Unaligned> Unaligned for [T] {}
unsafe impl<T: Unaligned, const N: usize> Unaligned for [T; N] {}

unsafe impl<T: Unaligned> Unaligned for core::num::Saturating<T> {}
unsafe impl<T: Unaligned> Unaligned for core::num::Wrapping<T> {}

unsafe impl<T: Unaligned> Unaligned for core::cmp::Reverse<T> {}

unsafe impl<T: Unaligned + ?Sized> Unaligned for core::mem::ManuallyDrop<T> {}
unsafe impl<T: Unaligned> Unaligned for core::mem::MaybeUninit<T> {}

unsafe impl<T: Unaligned + ?Sized> Unaligned for core::cell::UnsafeCell<T> {}
unsafe impl<T: Unaligned + ?Sized> Unaligned for core::cell::Cell<T> {}

unsafe impl<T: ?Sized> Unaligned for core::marker::PhantomData<T> {}

unsafe impl Unaligned for () {}

unsafe impl Unaligned for u8 {}
unsafe impl Unaligned for core::num::NonZero<u8> {}
#[cfg(target_has_atomic = "8")]
unsafe impl Unaligned for core::sync::atomic::AtomicU8 {}

unsafe impl Unaligned for i8 {}
unsafe impl Unaligned for core::num::NonZero<i8> {}
#[cfg(target_has_atomic = "8")]
unsafe impl Unaligned for core::sync::atomic::AtomicI8 {}

unsafe impl Unaligned for bool {}
#[cfg(target_has_atomic = "8")]
unsafe impl Unaligned for core::sync::atomic::AtomicBool {}

unsafe impl Unaligned for str {}

unsafe impl Unaligned for core::cmp::Ordering {}
unsafe impl Unaligned for core::marker::PhantomPinned {}

/// Trait for types that have no alignment requirement when wrapped
/// in an [`Option`].
pub unsafe trait UnalignedInOption: Sized {}

unsafe impl<T: UnalignedInOption> Unaligned for Option<T> {}

unsafe impl UnalignedInOption for core::num::NonZero<u8> {}
unsafe impl UnalignedInOption for core::num::NonZero<i8> {}

unsafe impl<T: UnalignedInOption> UnalignedInOption for core::num::Saturating<T> {}
unsafe impl<T: UnalignedInOption> UnalignedInOption for core::num::Wrapping<T> {}

unsafe impl<T: UnalignedInOption> UnalignedInOption for core::cmp::Reverse<T> {}

unsafe impl<T: UnalignedInOption> UnalignedInOption for core::mem::ManuallyDrop<T> {}

/// Type that forces some `T` to be unaligned.
#[repr(C, packed(1))]
pub struct Unalign<T>(pub T);

impl<T> Unalign<T> {
    #[inline]
    #[must_use]
    pub const fn new(x: T) -> Unalign<T> {
        Unalign(x)
    }

    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> T {
        unsafe { crate::util::transmute_unchecked(self) }
    }

    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        &raw const self.0
    }

    #[inline]
    #[must_use]
    pub const fn as_ptr_mut(&mut self) -> *mut T {
        &raw mut self.0
    }

    #[inline]
    #[must_use]
    pub const unsafe fn as_ref_unchecked(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }

    #[inline]
    #[must_use]
    pub const unsafe fn as_mut_unchecked(&mut self) -> &mut T {
        unsafe { &mut *self.as_ptr_mut() }
    }

    #[inline]
    #[must_use]
    pub fn is_aligned(&self) -> bool {
        self.as_ptr().is_aligned()
    }

    #[inline]
    #[must_use]
    pub fn as_ref_checked(&self) -> Option<&T> {
        if self.is_aligned() {
            Some(unsafe { self.as_ref_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn as_mut_checked(&mut self) -> Option<&mut T> {
        if self.is_aligned() {
            Some(unsafe { self.as_mut_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub const fn replace(&mut self, value: T) -> T {
        mem::replace(self, Unalign(value)).into_inner()
    }

    #[inline]
    #[must_use]
    pub fn set(&mut self, value: T) {
        self.0 = value;
    }

    #[inline]
    #[must_use]
    pub fn update<O, F: FnOnce(&mut T) -> O>(&mut self, f: F) -> O {
        if let Some(this) = self.as_mut_checked() {
            f(this)
        } else {
            struct OnDrop<T> {
                data: ManuallyDrop<T>,
                ptr: *mut Unalign<T>,
            }

            impl<T> Drop for OnDrop<T> {
                #[inline]
                fn drop(&mut self) {
                    let data = unsafe { ManuallyDrop::take(&mut self.data) };

                    unsafe { self.ptr.write(Unalign(data)) }
                }
            }

            let ptr = &raw mut *self;

            let mut dropper = OnDrop {
                data: ManuallyDrop::new(unsafe { ptr.read() }.into_inner()),
                ptr,
            };

            let return_value = f(&mut dropper.data);

            drop(dropper);

            return_value
        }
    }
}

impl<T: Copy> Unalign<T> {
    #[inline]
    #[must_use]
    pub const fn get(&self) -> T {
        self.0
    }
}

impl<T: Unaligned> Unalign<T> {
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        unsafe { self.as_ref_unchecked() }
    }

    #[inline]
    #[must_use]
    pub const fn as_mut(&mut self) -> &mut T {
        unsafe { self.as_mut_unchecked() }
    }
}

impl<T: Copy> Clone for Unalign<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> Copy for Unalign<T> {}

impl<T: Unaligned> Deref for Unalign<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: Unaligned> DerefMut for Unalign<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Unaligned + PartialEq> PartialEq for Unalign<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.deref().ne(other)
    }
}

impl<T: Unaligned + Eq> Eq for Unalign<T> {}

impl<T: Unaligned + PartialOrd> PartialOrd for Unalign<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(other)
    }
}

impl<T: Unaligned + Ord> Ord for Unalign<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deref().cmp(other)
    }
}

impl<T: Unaligned + hash::Hash> hash::Hash for Unalign<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl<T: Unaligned + fmt::Debug> fmt::Debug for Unalign<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: Unaligned + fmt::Display> fmt::Display for Unalign<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

unsafe impl<T> Unaligned for Unalign<T> {}

unsafe impl<T: Zeroable> Zeroable for Unalign<T> {}
unsafe impl<T: NoUninit> NoUninit for Unalign<T> {}
unsafe impl<T: Freeze> Freeze for Unalign<T> {}
