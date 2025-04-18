use core::convert::Infallible;

use crate::{CheckBits, Zeroable};

/// Marker trait for types that are valid for any bit pattern.
pub unsafe trait AnyBits: CheckBits<Error = Infallible> + Zeroable {}
