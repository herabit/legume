//! A crate for messing with data with well defined layouts.

#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[allow(unused_macros, unused_imports)]
pub(crate) mod macros;

/// Module for handling types that do not need drop glue.
pub mod forget;
