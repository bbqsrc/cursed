use libc::c_void;

use std::sync::Arc as RealArc;

#[derive(Debug)]
#[repr(transparent)]
pub struct Arc<T: ?Sized>(*const T);

impl<T> Arc<T> {
    pub fn new(item: T) -> Arc<T> {
        Arc::from(std::sync::Arc::new(item))
    }

    pub(crate) unsafe fn from_raw(ptr: *mut c_void) -> Arc<T> {
        Arc(ptr as *mut _)
    }
}

impl<T: ?Sized> std::ops::Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T: ?Sized> Arc<T> {
    #[doc(hidden)]
    pub(crate) fn as_ptr(&self) -> *const T {
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

impl<T: ?Sized> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        let arc = unsafe { RealArc::from_raw(self.0) };
        let arc1 = RealArc::clone(&arc);
        std::mem::forget(arc);
        Arc(RealArc::into_raw(arc1))
    }
}

impl<T: ?Sized> Drop for Arc<T> {
    fn drop(&mut self) {
        unsafe { RealArc::from_raw(self.0 as *mut c_void) };
    }
}

impl<T: ?Sized> From<RealArc<T>> for Arc<T> {
    fn from(arc: RealArc<T>) -> Arc<T> {
        Arc(RealArc::into_raw(arc))
    }
}
