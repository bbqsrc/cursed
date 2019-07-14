use libc::c_void;

use std::sync::Arc as RealArc;

#[repr(transparent)]
pub struct Arc<T: ?Sized>(*const T);

impl<T> Arc<T> {
    pub fn new(item: T) -> Arc<T> {
        Arc::from(std::sync::Arc::new(item))
    }

    unsafe fn from_raw(ptr: *mut c_void) -> Arc<T> {
        Arc(ptr as *mut _)
    }

    #[doc(hidden)]
    fn as_ptr(&self) -> *const T {
        self.0
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn try_as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            Some(unsafe { &*self.0 })
        }
    }
}

impl<T: ?Sized> Drop for Arc<T> {
    fn drop(&mut self) {
        unsafe { RealArc::from_raw(self.0 as *mut c_void) };
    }
}

impl<T> From<RealArc<T>> for Arc<T> {
    fn from(arc: RealArc<T>) -> Arc<T> {
        Arc(RealArc::into_raw(arc))
    }
}
