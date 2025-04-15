use core::{
    cell::{Cell, UnsafeCell},
    fmt, hash,
    mem::{transmute, ManuallyDrop},
    ptr::{self, NonNull},
};

/// Enum that determines whether the pointer metadata for a given type is
/// stored before, or after the address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetadataPosition {
    /// This indicates that the metadata is stored before the address.
    Before,
    /// This indicates that the metadata is stored after the address.
    After,
}

impl MetadataPosition {
    /// Get the metadata position of `[T]`.
    #[inline]
    #[must_use]
    pub const fn for_slice<T>() -> Self {
        const {
            let ptr = ptr::without_provenance::<T>(0xFF);
            let ptr = ptr::slice_from_raw_parts(ptr, 0x00);

            let [a, b]: [usize; 2] = unsafe { transmute(ptr) };

            match [a, b] {
                // If the length is first, then we know the metadata is before.
                [0x00, 0xFF] => MetadataPosition::Before,
                // If the length is second, then we know the metadata is after.
                [0xFF, 0x00] => MetadataPosition::After,
                _ => unreachable!(),
            }
        }
    }
}

/// Trait for doing stuff with pointer metadata.
pub unsafe trait Pointee {
    /// The pointer metadata.
    type Metadata: fmt::Debug + Copy + Send + Sync + Ord + hash::Hash + Unpin;

    /// Where the metadata is stored within a pointer to `Self` relative to the address.
    const METADATA_POSITION: MetadataPosition;
}

unsafe impl<T> Pointee for T {
    type Metadata = ();

    const METADATA_POSITION: MetadataPosition = MetadataPosition::Before;
}

unsafe impl<T> Pointee for [T] {
    type Metadata = usize;

    const METADATA_POSITION: MetadataPosition = MetadataPosition::for_slice::<T>();
}

unsafe impl Pointee for str {
    type Metadata = <[u8] as Pointee>::Metadata;

    const METADATA_POSITION: MetadataPosition = <[u8] as Pointee>::METADATA_POSITION;
}

// SAFETY: I'm pretty sure these will always be correct.
//
//         Pretty... Sure.

unsafe impl<T> Pointee for ManuallyDrop<[T]> {
    type Metadata = <[T] as Pointee>::Metadata;
    const METADATA_POSITION: MetadataPosition = <[T] as Pointee>::METADATA_POSITION;
}

unsafe impl Pointee for ManuallyDrop<str> {
    type Metadata = <str as Pointee>::Metadata;
    const METADATA_POSITION: MetadataPosition = <str as Pointee>::METADATA_POSITION;
}

unsafe impl<T> Pointee for UnsafeCell<[T]> {
    type Metadata = <[T] as Pointee>::Metadata;
    const METADATA_POSITION: MetadataPosition = <[T] as Pointee>::METADATA_POSITION;
}

unsafe impl Pointee for UnsafeCell<str> {
    type Metadata = <str as Pointee>::Metadata;
    const METADATA_POSITION: MetadataPosition = <str as Pointee>::METADATA_POSITION;
}

unsafe impl<T> Pointee for Cell<[T]> {
    type Metadata = <[T] as Pointee>::Metadata;
    const METADATA_POSITION: MetadataPosition = <[T] as Pointee>::METADATA_POSITION;
}

unsafe impl Pointee for Cell<str> {
    type Metadata = <str as Pointee>::Metadata;
    const METADATA_POSITION: MetadataPosition = <str as Pointee>::METADATA_POSITION;
}

/// Trait for types that are accessed through thin pointers.
pub trait Thin: Pointee<Metadata = ()> {}

impl<T: ?Sized + Pointee<Metadata = ()>> Thin for T {}

/// Get the metadata of a given pointer.
#[inline]
#[must_use]
pub const fn metadata<T: Pointee + ?Sized>(ptr: *const T) -> T::Metadata {
    let repr = PtrRepr { const_ptr: ptr };

    match T::METADATA_POSITION {
        MetadataPosition::Before => unsafe { repr.parts_before.meta },
        MetadataPosition::After => unsafe { repr.parts_after.meta },
    }
}

/// Create a pointer to a `T` from an address and the pointer metadata.
#[inline]
#[must_use]
pub const fn from_raw_parts<T: Pointee + ?Sized>(addr: *const (), meta: T::Metadata) -> *const T {
    let repr = match T::METADATA_POSITION {
        MetadataPosition::Before => PtrRepr {
            parts_before: Before { addr, meta },
        },
        MetadataPosition::After => PtrRepr {
            parts_after: After { addr, meta },
        },
    };

    unsafe { repr.const_ptr }
}

/// Create a mutable pointer to a `T` from an address and the pointer metadata.
#[inline]
#[must_use]
pub const fn from_raw_parts_mut<T: Pointee + ?Sized>(addr: *mut (), meta: T::Metadata) -> *mut T {
    let repr = match T::METADATA_POSITION {
        MetadataPosition::Before => PtrRepr {
            parts_before: Before { addr, meta },
        },
        MetadataPosition::After => PtrRepr {
            parts_after: After { addr, meta },
        },
    };

    unsafe { repr.mut_ptr }
}

/// Create a [`NonNull`] pointer to a `T` from an address and the pointer metadata.
#[inline]
#[must_use]
pub const fn from_raw_parts_nonnull<T: Pointee + ?Sized>(
    addr: NonNull<()>,
    meta: T::Metadata,
) -> NonNull<T> {
    let repr = match T::METADATA_POSITION {
        MetadataPosition::Before => PtrRepr {
            parts_before: Before {
                addr: addr.as_ptr(),
                meta,
            },
        },
        MetadataPosition::After => PtrRepr {
            parts_after: After {
                addr: addr.as_ptr(),
                meta,
            },
        },
    };

    unsafe { repr.nonnull_ptr }
}

/// Get the address and metadata of a pointer.
#[inline]
#[must_use]
pub const fn to_raw_parts<T: Pointee + ?Sized>(ptr: *const T) -> (*const (), T::Metadata) {
    (ptr.cast(), metadata(ptr))
}

/// Get the address and metadata of a mutable pointer.
#[inline]
#[must_use]
pub const fn to_raw_parts_mut<T: Pointee + ?Sized>(ptr: *mut T) -> (*mut (), T::Metadata) {
    (ptr.cast(), metadata(ptr))
}

/// Get the address and metadata of a [`NonNull`] pointer.
#[inline]
#[must_use]
pub const fn to_raw_parts_nonnull<T: Pointee + ?Sized>(
    ptr: NonNull<T>,
) -> (NonNull<()>, T::Metadata) {
    (ptr.cast(), metadata(ptr.as_ptr()))
}

/// Create a pointer to a `U` with the address of `addr` and the metadata of `meta`.
#[inline]
#[must_use]
pub const fn with_metadata_of<T: ?Sized, U: Pointee + ?Sized>(
    addr: *const T,
    meta: *const U,
) -> *const U {
    from_raw_parts(addr.cast(), metadata(meta))
}

/// Create a mutable pointer to a `U` with the address of `addr` and the metadata of `meta`.
#[inline]
#[must_use]
pub const fn with_metadata_of_mut<T: ?Sized, U: Pointee + ?Sized>(
    addr: *mut T,
    meta: *const U,
) -> *mut U {
    from_raw_parts_mut(addr.cast(), metadata(meta))
}

/// Create a [`NonNull`] pointer to a `U` with the address of `addr` and the metadata of `meta`.
#[inline]
#[must_use]
pub const fn with_metadata_of_nonnull<T: ?Sized, U: Pointee + ?Sized>(
    addr: NonNull<T>,
    meta: *const U,
) -> NonNull<U> {
    from_raw_parts_nonnull(addr.cast(), metadata(meta))
}

#[repr(C)]
union PtrRepr<T: ?Sized + Pointee> {
    const_ptr: *const T,
    mut_ptr: *mut T,
    nonnull_ptr: NonNull<T>,

    parts_before: Before<T>,
    parts_after: After<T>,
}

#[repr(C)]
struct Before<T: ?Sized + Pointee> {
    meta: T::Metadata,
    addr: *const (),
}

impl<T: ?Sized + Pointee> Copy for Before<T> {}
impl<T: ?Sized + Pointee> Clone for Before<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
struct After<T: ?Sized + Pointee> {
    addr: *const (),
    meta: T::Metadata,
}

impl<T: ?Sized + Pointee> Copy for After<T> {}
impl<T: ?Sized + Pointee> Clone for After<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
