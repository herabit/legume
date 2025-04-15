use core::ptr;

#[inline]
#[must_use]
pub const unsafe fn memset<T: ?Sized>(dst: &mut T, val: u8) {
    let size = size_of_val(dst);

    unsafe { ptr::from_mut(dst).cast::<u8>().write_bytes(val, size) }
}
