#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod macros;
mod util;

mod zeroable;
pub use zeroable::*;

mod zeroable_in_option;
pub use zeroable_in_option::*;

mod func;
pub use func::*;

mod no_uninit;
pub use no_uninit::*;

mod no_uninit_in_option;
pub use no_uninit_in_option::*;

/// Module for handling pointers.
pub mod ptr;
#[doc(inline)]
pub use ptr::{Pointee, Thin};
