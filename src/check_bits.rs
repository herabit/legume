use core::{convert::Infallible, ptr::NonNull};

use crate::{Freeze, Pointee, Zeroable};

pub unsafe trait AnyBits: CheckBits<Error = Infallible, Bits = Self> + Zeroable {}

pub unsafe trait CheckBits: Pointee {
    type Error;
    type Bits: ?Sized + AnyBits<Metadata = Self::Metadata>;

    /// This method does the actual work of validating whether bits are valid.
    ///
    /// It must NOT mutate any data, and `ptr` must be valid for reads.
    #[track_caller]
    unsafe fn check_bits(ptr: NonNull<Self::Bits>) -> Result<NonNull<Self>, Self::Error>;

    #[track_caller]
    #[inline(always)]
    fn cast_ref(bits: &Self::Bits) -> Result<&Self, Self::Error>
    where
        Self: Freeze,
        Self::Bits: Freeze,
    {
        // SAFETY: We know that `bits` contains no interior mutability,
        //         so it is safe to read the underlying data without fear
        //         of something being changed, or there being a data race.
        match unsafe { Self::check_bits(bits.into()) } {
            // SAFETY: We know `bits` is valid!
            Ok(ptr) => Ok(unsafe { ptr.as_ref() }),
            Err(err) => Err(err),
        }
    }

    #[track_caller]
    #[inline(always)]
    fn cast_mut(bits: &mut Self::Bits) -> Result<&mut Self, Self::Error> {
        // SAFETY: We have a unique reference to `bits`, so even if it
        //         contains interior mutability, there is no change of
        //         someone else changing the value without us expecting
        //         it to.
        match unsafe { Self::check_bits(bits.into()) } {
            // SAFETY: We know `bits` is valid!
            Ok(mut ptr) => Ok(unsafe { ptr.as_mut() }),
            Err(err) => Err(err),
        }
    }

    #[track_caller]
    #[inline(always)]
    fn cast(mut bits: Self::Bits) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Bits: Sized,
    {
        match Self::cast_mut(&mut bits) {
            // SAFETY: We know `bits` is valid!
            Ok(_) => Ok(unsafe { crate::util::transmute_unchecked(bits) }),
            Err(err) => Err(err),
        }
    }
}

unsafe impl<T: AnyBits> AnyBits for [T] {}
unsafe impl<T: CheckBits<Bits: Sized>> CheckBits for [T] {
    type Error = T::Error;
    type Bits = [T::Bits];

    #[inline(always)]
    unsafe fn check_bits(ptr: NonNull<[T::Bits]>) -> Result<NonNull<Self>, T::Error> {
        // const {
        //     assert!(size_of::<T>() == size_of::<T::Bits>(), "size mismatch");
        //     assert!(
        //         align_of::<T>() == align_of::<T::Bits>(),
        //         "alignment mismatch"
        //     );
        // };

        // If we're working with a ZST, there's no bit pattern to check lol.
        if size_of::<T>() == 0 {
            return Ok(unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut [T]) });
        }

        let start = ptr.cast::<T::Bits>();
        let end = unsafe { start.add(ptr.len()) };

        let mut cur = start;
        let mut err = None;

        while cur < end {
            // This should assist autovectorization rather than just leaving early.
            err = err.or(unsafe { T::check_bits(cur) }.err());

            cur = unsafe { cur.add(1) };
        }

        match err {
            None => Ok(unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut [T]) }),
            Some(err) => Err(err),
        }
    }
}

unsafe impl<T: AnyBits, const N: usize> AnyBits for [T; N] {}
unsafe impl<T: CheckBits<Bits: Sized>, const N: usize> CheckBits for [T; N] {
    type Error = T::Error;
    type Bits = [T::Bits; N];

    #[inline(always)]
    unsafe fn check_bits(ptr: NonNull<Self::Bits>) -> Result<NonNull<Self>, Self::Error> {
        match unsafe { <[T]>::check_bits(ptr) } {
            Ok(ptr) => Ok(ptr.cast()),
            Err(err) => Err(err),
        }
    }
}

macro_rules! any_bits {
    ($($ty:ty),*) => {
        $(
            unsafe impl CheckBits for $ty {
                type Error = Infallible;
                type Bits = $ty;

                #[inline(always)]
                unsafe fn check_bits(ptr: NonNull<$ty>) -> Result<NonNull<$ty>, Infallible> {
                    Ok(ptr)
                }
            }

            unsafe impl AnyBits for $ty {}
        )*
    };
}

any_bits!(());

any_bits!(u8, u16, u32, u64, u128, usize);
any_bits!(i8, i16, i32, i64, i128, isize);
any_bits!(f32, f64);

unsafe impl CheckBits for bool {
    type Error = ();
    type Bits = u8;

    #[inline(always)]
    unsafe fn check_bits(ptr: NonNull<Self::Bits>) -> Result<NonNull<Self>, Self::Error> {
        match unsafe { ptr.read() } {
            0 | 1 => Ok(ptr.cast()),
            _ => Err(()),
        }
    }
}

unsafe impl CheckBits for char {
    type Error = ();
    type Bits = u32;

    #[inline(always)]
    unsafe fn check_bits(ptr: NonNull<Self::Bits>) -> Result<NonNull<Self>, Self::Error> {
        match char::from_u32(unsafe { ptr.read() }) {
            Some(_) => Ok(ptr.cast()),
            None => Err(()),
        }
    }
}

unsafe impl CheckBits for str {
    type Error = core::str::Utf8Error;
    type Bits = [u8];

    #[inline(always)]
    unsafe fn check_bits(ptr: NonNull<Self::Bits>) -> Result<NonNull<Self>, Self::Error> {
        match core::str::from_utf8(unsafe { ptr.as_ref() }) {
            Ok(_) => Ok(unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut str) }),
            Err(err) => Err(err),
        }
    }
}

unsafe impl CheckBits for core::cmp::Ordering {
    type Error = ();
    type Bits = i8;

    #[inline(always)]
    unsafe fn check_bits(ptr: NonNull<Self::Bits>) -> Result<NonNull<Self>, Self::Error> {
        match unsafe { ptr.read() } {
            -1 | 0 | 1 => Ok(ptr.cast()),
            _ => Err(()),
        }
    }
}
