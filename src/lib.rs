//! A crate for messing with data with well defined layouts.

#![cfg_attr(not(test), no_std)]
#![allow(unused_macros)]
#![cfg_attr(feature = "nightly", feature(never_type))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[allow(unused_macros, unused_imports)]
pub(crate) mod macros;

/// Module for handling types that do not need drop glue.
pub mod forget;

/// Module for handling the inhabitability of types.
pub mod inhabit;

/// Module for array nonsense.
pub mod array;

#[doc(inline)]
pub use forget::Forget;
#[doc(inline)]
pub use inhabit::{Inhabited, Uninhabited};
