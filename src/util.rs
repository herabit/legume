use core::{ptr, slice};

#[inline]
#[must_use]
pub const unsafe fn memset<T: ?Sized>(dst: &mut T, val: u8) {
    let size = size_of_val(dst);

    unsafe { ptr::from_mut(dst).cast::<u8>().write_bytes(val, size) }
}

#[inline]
#[must_use]
pub const unsafe fn bytes_of<T: ?Sized>(src: &T) -> &[u8] {
    let size = size_of_val(src);

    unsafe { slice::from_raw_parts(ptr::from_ref(src).cast::<u8>(), size) }
}

#[inline]
#[must_use]
pub const unsafe fn bytes_of_mut<T: ?Sized>(src: &mut T) -> &mut [u8] {
    let size = size_of_val(src);

    unsafe { slice::from_raw_parts_mut(ptr::from_mut(src).cast::<u8>(), size) }
}
