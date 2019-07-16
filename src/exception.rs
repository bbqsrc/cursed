use crate::nullable::{null, Nullable};
use libc::c_char;
use std::convert::TryFrom;
use std::ffi::{CString, NulError};
use std::ptr::NonNull;

/// A newtype over a raw, owned CString for providing errors over the FFI.
#[repr(transparent)]
pub struct Exception(NonNull<c_char>);

impl Exception {
    pub unsafe fn from_raw(ptr: NonNull<c_char>) -> Exception {
        Exception(ptr)
    }

    pub fn into_c_string(self) -> CString {
        let ret = unsafe { CString::from_raw(self.0.as_ptr() as *mut _) };
        std::mem::forget(self);
        ret
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }

    pub fn into_raw(self) -> *mut Exception {
        let ret = self.as_ptr() as *mut _;
        std::mem::forget(self);
        ret
    }
}

impl Drop for Exception {
    fn drop(&mut self) {
        eprintln!("EXCEPTION DROPPED: {:?}", self.0.as_ptr());
        unsafe { eprintln!("was: {:?}", CString::from_raw(self.0.as_ptr() as *mut _)) };
    }
}

impl TryFrom<&str> for Exception {
    type Error = NulError;

    fn try_from(string: &str) -> Result<Exception, Self::Error> {
        let c_str = CString::new(string)?;
        Ok(Exception(NonNull::new(c_str.into_raw()).unwrap()))
    }
}

impl From<Exception> for CString {
    fn from(exception: Exception) -> CString {
        exception.into_c_string()
    }
}

#[inline]
pub fn throw_message<T, S: AsRef<str>>(
    msg: S,
    exception: &crate::inout::OutPtr<Exception>,
) -> Nullable<T> {
    if let Some(ptr) = exception.as_ptr() {
        let msg = Exception::try_from(msg.as_ref()).unwrap();
        unsafe { *ptr.as_ptr() = msg.into_raw() };
    }
    null()
}

#[inline]
pub fn throw<T>(
    e: impl std::fmt::Display,
    exception: &crate::inout::OutPtr<Exception>,
) -> Nullable<T> {
    if let Some(ptr) = exception.as_ptr() {
        let msg = Exception::try_from(&*format!("{}", e)).unwrap();
        unsafe { *ptr.as_ptr() = msg.into_raw() };
    }
    null()
}

#[cfg(test)]
mod tests {}
