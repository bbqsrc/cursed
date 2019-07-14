use libc::c_char;
use std::ptr::NonNull;
use std::ffi::{CString, NulError};
use std::convert::TryFrom;

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
}

impl Drop for Exception {
    fn drop(&mut self) {
        unsafe { CString::from_raw(self.0.as_ptr() as *mut _) };
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

#[cfg(test)]
mod tests {

}