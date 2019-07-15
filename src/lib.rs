#![feature(const_type_id)]
#![feature(proc_macro_hygiene)]

mod nullable;
mod sync;
mod exception;
mod vec;

pub use nullable::*;
pub use sync::*;
pub use exception::*;
pub use vec::*;

use std::ptr::NonNull;
use std::convert::TryFrom;

#[inline]
pub fn not_null<T>(field: &str, ptr: *const T, exception: *mut Exception) -> Option<NonNull<T>> {
    NonNull::new(ptr as *mut _).or_else(|| {
        if !exception.is_null() {
            unsafe {
                *exception =
                    Exception::try_from(&*format!("{} must not be null", field)).unwrap()
            };
        }
        None
    })
}

#[macro_export]
macro_rules! try_not_null {
    ($path:tt, $exception:tt) => {
        match $crate::not_null(stringify!($path), $path, $exception) {
            Some(v) => v,
            None => return crate::null(),
        }
    };
}

#[macro_export]
macro_rules! not_null_or_return {
    ($path:tt, $exception:tt, $fallback:tt) => {
        match $crate::not_null(stringify!($path), $path, $exception) {
            Some(v) => v,
            None => { return $fallback; }
        }
    };

    ($path:tt, $exception:tt) => {
        match $crate::not_null(stringify!($path), $path, $exception) {
            Some(v) => v,
            None => { return; }
        }
    };
}
