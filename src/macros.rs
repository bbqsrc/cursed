use core::convert::TryFrom;
use core::ptr::NonNull;

use crate::exception::Exception;

cfg_if::cfg_if! {
    if #[cfg(feature = "no-std")] {
        use alloc::format;
    } else {
        use std::format;
    }
}

#[inline]
pub fn not_null<T>(
    field: &str,
    ptr: Option<NonNull<T>>,
    exception: &crate::inout::OutPtr<Exception>,
) -> Option<NonNull<T>> {
    if ptr.is_none() {
        if let Some(exception) = exception.as_ptr() {
            unsafe {
                *exception.as_ptr() = Exception::try_from(&*format!("{} must not be null", field))
                    .unwrap()
                    .into_raw()
            };
        }
    }
    ptr
}

#[macro_export]
macro_rules! try_not_null {
    ($path:expr, $exception:expr) => {
        match $crate::macros::not_null(stringify!($path), $path, $exception) {
            Some(path) => path,
            None => return $crate::nullable::null(),
        }
    };

    ($path:expr, $exception:expr, $fallback:expr) => {
        match $crate::macros::not_null(stringify!($path), $path, $exception) {
            Some(v) => v,
            None => {
                return $fallback;
            }
        }
    };
}

#[macro_export]
macro_rules! try_as_ref {
    ($arc:expr, $exception:expr, $fallback:expr) => {
        match $arc.as_ref() {
            Some(r) => r,
            None => {
                let _: Nullable<()> = $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($arc)),
                    $exception,
                );

                return $fallback;
            }
        }
    };

    ($arc:expr, $exception:expr) => {
        match $arc.as_ref() {
            Some(r) => r,
            None => {
                return $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($arc)),
                    $exception,
                );
            }
        }
    };
}

#[macro_export]
macro_rules! try_as_mut_ref {
    ($thing:expr, $exception:expr, $fallback:expr) => {
        match $thing.as_mut_ref() {
            Some(v) => v,
            None => {
                let _: $crate::nullable::Nullable<()> = $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($thing)),
                    $exception,
                );
                return $fallback;
            }
        }
    };

    ($thing:expr, $exception:expr) => {
        match $thing.as_mut_ref() {
            Some(v) => v,
            None => {
                return $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($thing)),
                    $exception,
                );
            }
        }
    };
}

#[macro_export]
macro_rules! try_into_arc {
    ($arc:expr, $exception:expr) => {
        match $arc {
            None => {
                return $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($arc)),
                    $exception,
                );
            }
            Some(arc) => std::sync::Arc::from_raw(arc.as_ptr() as *const _),
        }
    };

    ($arc:expr, $exception:expr, $fallback:expr) => {
        match $arc {
            None => {
                let _: $crate::nullable::Nullable<()> = $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($arc)),
                    $exception,
                );
                return $fallback;
            }
            Some(arc) => std::sync::Arc::from_raw(arc.as_ptr() as *const _),
        }
    };
}

#[macro_export]
macro_rules! try_as_arc {
    ($inout:expr, $exception:expr) => {
        match $inout.as_arc() {
            None => {
                return $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($arc)),
                    $exception,
                );
            }
            Some(arc) => arc,
        }
    };

    ($inout:expr, $exception:expr, $fallback:expr) => {
        match $inout.as_arc() {
            None => {
                let _: $crate::nullable::Nullable<()> = $crate::exception::throw_message(
                    &*format!("{} must not be null", stringify!($arc)),
                    $exception,
                );
                return $fallback;
            }
            Some(arc) => arc,
        }
    };
}

#[macro_export]
macro_rules! try_as_str {
    ($ptr:expr, $exception:expr, $fallback:expr) => {
        match $crate::macros::not_null(stringify!($ptr), $ptr.as_ptr(), $exception) {
            Some(ptr) => match unsafe { std::ffi::CStr::from_ptr(ptr.as_ptr()).to_str() } {
                Ok(v) => v,
                Err(e) => {
                    let _: $crate::nullable::Nullable<()> = $crate::exception::throw(e, $exception);
                    return $fallback;
                }
            },
            None => return $fallback,
        }
    };
    ($ptr:expr, $exception:expr) => {
        match $crate::macros::not_null(stringify!($ptr), $ptr.as_ptr(), $exception) {
            Some(ptr) => match unsafe { std::ffi::CStr::from_ptr(ptr.as_ptr()).to_str() } {
                Ok(v) => v,
                Err(e) => return throw(e, $exception),
            },
            None => return $crate::nullable::null(),
        }
    };
}
