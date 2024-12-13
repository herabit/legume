/// Type for doing checks on array lengths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Length<const N: usize>;

/// Trait for array lengths that are not zero (aka the array is not empty).
pub trait NonEmpty: sealed::Sealed {}

mod sealed {
    pub trait Sealed {}

    impl<const N: usize> Sealed for super::Length<N> {}
}

mod _len {
    include!(concat!(env!("OUT_DIR"), "/arr_len.rs"));
}
