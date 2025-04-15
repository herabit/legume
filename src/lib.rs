#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod macros;
mod util;
mod zeroable_in_option;

pub mod func;
pub mod ptr;
pub mod zeroable;
