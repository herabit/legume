macro_rules! create_tuple {
    ($dol:tt;
        $(
            (
              $($field:ident),*
              $(,)?
            )
            $(,)?
        )*
    ) => {
        macro_rules! impl_marker_tuple {
            (impl $dol path:path) => {
                $(
                    impl<
                        $($field: $dol path),*
                    > $dol path for ($($field,)*) {}
                )*
            };


            (unsafe impl $dol path:path) => {
                $(
                    unsafe impl<
                        $($field: $dol path),*
                    > $dol path for ($($field,)*) {}
                )*
            };
        }


        pub(crate) use impl_marker_tuple;

        // See https://doc.rust-lang.org/beta/nightly-rustc/src/rustc_target/spec/mod.rs.html#2774-2838
        macro_rules! impl_marker_fn {
            (
                $dol ( unsafe $dol($dol unsafe:lifetime)? )?
                impl $dol path:path) => {
                $(
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "Rust" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "Rust"  fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "C" fn($($field),*) -> R {}

                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "C" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "C-unwind" fn($($field),*) -> R {}

                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "C-unwind" fn($($field),*) -> R {}

                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "system" fn($($field),*) -> R {}

                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "system" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "system-unwind" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "system-unwind" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "cdecl" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "cdecl" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "cdecl-unwind" fn($($field),*) -> R {}


                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "cdecl-unwind" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "win64" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "win64" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "win64-unwind" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "win64-unwind" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "sysv64" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "sysv64" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "sysv64-unwind" fn($($field),*) -> R {}



                    #[cfg(target_arch = "x86_64")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "sysv64-unwind" fn($($field),*) -> R {}



                    #[cfg(target_arch = "x86")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "thiscall" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "thiscall" fn($($field),*) -> R {}



                    #[cfg(target_arch = "x86")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "thiscall-unwind" fn($($field),*) -> R {}


                    #[cfg(target_arch = "x86")]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "thiscall-unwind" fn($($field),*) -> R {}

                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "stdcall" fn($($field),*) -> R {}


                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "stdcall" fn($($field),*) -> R {}


                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "stdcall-unwind" fn($($field),*) -> R {}


                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "stdcall-unwind" fn($($field),*) -> R {}



                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "fastcall" fn($($field),*) -> R {}


                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "fastcall" fn($($field),*) -> R {}

                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for extern "fastcall-unwind" fn($($field),*) -> R {}


                    #[cfg(any(
                        windows,
                        target_arch = "x86",
                    ))]
                    $dol (unsafe $dol ($dol unsafe)? )?
                    impl<R: ?Sized $(, $field: ?Sized)* >
                    $dol path for unsafe extern "fastcall-unwind" fn($($field),*) -> R {}
                )*
            }
        }

        pub(crate) use impl_marker_fn;
    };
}

create_tuple!($;
    (),
    (T0),
    (T0, T1),
    (T0, T1, T2),
    (T0, T1, T2, T3),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5, T6, T7),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),
);
