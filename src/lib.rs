#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod macros;
mod util;

mod check_bits;
pub use check_bits::*;

// mod any_bits;
// pub use any_bits::*;

mod zeroable;
pub use zeroable::*;

// mod zeroable_in_option;
// pub use zeroable_in_option::*;

// mod func;
// pub use func::*;

mod no_uninit;
pub use no_uninit::*;

mod no_uninit_in_option;
pub use no_uninit_in_option::*;

mod freeze;
pub use freeze::*;

mod unaligned;
pub use unaligned::*;

/// Module for handling pointers.
pub mod ptr;
#[doc(inline)]
pub use ptr::{Pointee, Thin};

#[inline]
#[must_use]
pub const fn bytes_of<T: NoUninit + Freeze + ?Sized>(src: &T) -> &[u8] {
    unsafe { util::bytes_of(src) }
}
