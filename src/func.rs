/// A trait for function pointers.
pub trait FnPtr: Sized + Copy {
    type Args;
    type Return;

    const CALL_CONV: CallConv;

    /// Get the address of the function pointer.
    #[track_caller]
    fn to_addr(self) -> *const ();

    /// Call the function pointer not knowing whether it's safe or not to.
    #[track_caller]
    unsafe fn call_unchecked(self, args: Self::Args) -> Self::Return;
}

/// A trait for function pointers that are always safe to call.
pub unsafe trait SafeFnPtr: FnPtr {
    /// Call the function pointer knowing that it is safe to call.
    #[track_caller]
    #[inline]
    fn call(self, args: Self::Args) -> Self::Return {
        unsafe { self.call_unchecked(args) }
    }
}

macro_rules! define_calling_conv {
    ($dol:tt =>
        $(#[$attr:meta])*
        pub enum $name:ident {
            $(
                $(#[$conv_attr:meta])*
                $conv:ident($conv_name:tt)
            ),+

            $(,)?
        }
    ) => {
        $(#[$attr])*
        pub enum $name {
            $(
                $(#[$conv_attr])*
                $conv,
            )*
        }

        impl $name {
            /// Get the name of the calling convention.
            #[inline]
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $( Self::$conv => $conv_name, )*
                }
            }
        }

        #[macro_export]
        macro_rules! call_conv {
            $(
                ($conv_name) => { $dol crate::func::CallConv::$conv };
            )*
        }
    };
}

define_calling_conv!($ =>
    /// An enum for the various calling conventions that Rust supports.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    #[non_exhaustive]
    pub enum CallConv {
        /// The `Rust` calling convention.
        #[default]
        Rust("Rust"),

        /// The `C` calling convention.
        C("C"),
        /// The `C-unwind` calling convention.
        CUnwind("C-unwind"),

        /// The `system` calling convention.
        System("system"),
        /// The `system-unwind`
        SystemUnwind("system-unwind"),

        /// The `cdecl` calling convention.
        Cdecl("cdecl"),
        /// The `cdecl-unwind` calling convention.
        CdeclUnwind("cdecl-unwind"),

        /// The `stdcall` calling convention.
        Stdcall("stdcall"),
        /// The `stdcall-unwind` calling convention.
        StdcallUnwind("stdcall-unwind"),

        /// The `win64` calling convention.
        Win64("win64"),
        /// The `win64-unwind` calling convention.
        Win64Unwind("win64-unwind"),

        /// The `sysv64` calling convention.
        Sysv64("sysv64"),
        /// The `sysv64-unwind` calling convention.
        Sysv64Unwind("sysv64-unwind"),

        /// The `aapcs` calling convention.
        Aapcs("aapcs"),
        /// The `aapcs-unwind` calling convention.
        AapcsUnwind("aapcs-unwind"),

        /// The `fastcall` calling convention.
        Fastcall("fastcall"),
        /// The `fastcall-unwind` calling convention.
        FastcallUnwind("fastcall-unwind"),

        /// The `thiscall` calling convention.
        Thiscall("thiscall"),
        /// The `thiscall-unwind` calling convention.
        ThiscallUnwind("thiscall-unwind"),
    }
);

impl CallConv {
    /// Returns whether this calling conventon can unwind into its caller.
    #[inline]
    #[must_use]
    pub const fn can_unwind(self) -> bool {
        match self {
            CallConv::Rust
            | CallConv::CUnwind
            | CallConv::SystemUnwind
            | CallConv::CdeclUnwind
            | CallConv::StdcallUnwind
            | CallConv::Win64Unwind
            | CallConv::Sysv64Unwind
            | CallConv::AapcsUnwind
            | CallConv::FastcallUnwind
            | CallConv::ThiscallUnwind => true,

            CallConv::C
            | CallConv::System
            | CallConv::Cdecl
            | CallConv::Stdcall
            | CallConv::Win64
            | CallConv::Sysv64
            | CallConv::Aapcs
            | CallConv::Fastcall
            | CallConv::Thiscall => false,
        }
    }
}

macro_rules! create_fn_ptr {
    ($dol:tt => (
        $(
            (
                $($name:ident),* $(,)?
            )
        ),* $(,)?
    )) => {
        macro_rules! fn_ptr {
            (
                $dol ($dol call_conv:tt),*
            ) => {
                $dol(
                $(
                    // Handle safe functions.

                    impl<Ret $(, $name )*> FnPtr for extern $dol call_conv fn($(_: $name),*) -> Ret {
                        type Args = ($($name,)*);
                        type Return = Ret;

                        const CALL_CONV: CallConv = call_conv!($dol call_conv);

                        #[inline]
                        fn to_addr(self) -> *const () {
                            self as _
                        }

                        #[inline]
                        unsafe fn call_unchecked(self, args: Self::Args) -> Self::Return
                        {
                            #[allow(non_snake_case)]
                            let ($($name,)*) = args;

                            self($($name),*)
                        }
                    }

                    unsafe impl<Ret $(, $name )*> SafeFnPtr for extern $dol call_conv fn($(_: $name),*) -> Ret {}

                    // Handle unsafe functions.

                    impl<Ret $(, $name)*> FnPtr for unsafe extern $dol call_conv fn($(_: $name),*) -> Ret {
                        type Args = ($($name,)*);
                        type Return = Ret;

                        const CALL_CONV: CallConv = call_conv!($dol call_conv);

                        #[inline]
                        fn to_addr(self) -> *const () {
                            self as _
                        }

                        #[inline]
                        unsafe fn call_unchecked(self, args: Self::Args) -> Self::Return {
                            #[allow(non_snake_case)]
                            let ($($name,)*) = args;

                            unsafe { self($($name),*) }
                        }
                    }
                )*)*
            };
        }
    };
}

create_fn_ptr!($ => (
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8),
    (T0, T1, T2, T3, T4, T5, T6, T7),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3),
    (T0, T1, T2),
    (T0, T1),
    (T0),
    (),
));

fn_ptr!("Rust", "C", "C-unwind", "system", "system-unwind");

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn_ptr!("cdecl", "cdecl-unwind");

#[cfg(target_arch = "x86_64")]
fn_ptr!("sysv64", "sysv64-unwind");

#[cfg(target_arch = "arm")]
fn_ptr!("aapcs", "aapcs-unwind");
