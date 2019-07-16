use crate::exception::Exception;
use std::convert::TryFrom;
use std::ptr::NonNull;

#[inline]
pub fn not_null<T>(
    field: &str,
    ptr: Option<NonNull<T>>,
    exception: Option<NonNull<Exception>>,
) -> Option<NonNull<T>> {
    if ptr.is_none() {
        if let Some(exception) = exception {
            unsafe {
                *exception.as_ptr() =
                    Exception::try_from(&*format!("{} must not be null", field)).unwrap()
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
    ($arc:expr, $exception:expr) => {
        match $arc.as_ref() {
            Some(v) => v,
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
macro_rules! try_as_str {
    ($ptr:expr, $exception:expr) => {
        match $crate::macros::not_null(stringify!($ptr), $ptr, $exception) {
            Some(ptr) => match std::ffi::CStr::from_ptr(ptr.as_ptr()).to_str() {
                Ok(v) => v,
                Err(e) => return throw(e, $exception),
            },
            None => return $crate::nullable::null(),
        }
    };
}
