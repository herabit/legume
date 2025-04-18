#![allow(dead_code)]

use core::{mem::ManuallyDrop, ptr, slice};

#[inline(always)]
#[must_use]
pub const unsafe fn memset<T: ?Sized>(dst: &mut T, val: u8) {
    let size = size_of_val(dst);

    unsafe { ptr::from_mut(dst).cast::<u8>().write_bytes(val, size) }
}

#[inline(always)]
#[must_use]
pub const unsafe fn bytes_of<T: ?Sized>(src: &T) -> &[u8] {
    let size = size_of_val(src);

    unsafe { slice::from_raw_parts(ptr::from_ref(src).cast::<u8>(), size) }
}

#[inline(always)]
#[must_use]
pub const unsafe fn bytes_of_mut<T: ?Sized>(src: &mut T) -> &mut [u8] {
    let size = size_of_val(src);

    unsafe { slice::from_raw_parts_mut(ptr::from_mut(src).cast::<u8>(), size) }
}

#[inline(always)]
#[must_use]
pub const unsafe fn transmute_unchecked<Src, Dst>(src: Src) -> Dst {
    #[repr(C)]
    union Transmute<Src, Dst> {
        src: ManuallyDrop<Src>,
        dst: ManuallyDrop<Dst>,
    }

    unsafe {
        ManuallyDrop::into_inner(
            Transmute::<Src, Dst> {
                src: ManuallyDrop::new(src),
            }
            .dst,
        )
    }
}
