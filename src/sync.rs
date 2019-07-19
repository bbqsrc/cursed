use alloc::sync::Arc;
use core::ffi::c_void;

use crate::nullable::{null, Nullable};

#[derive(Debug)]
#[repr(transparent)]
pub struct ArcPtr<T: ?Sized>(*const T);

impl<T: ?Sized> ArcPtr<T> {
    #[doc(hidden)]
    pub(crate) fn as_ptr(&self) -> *const T {
        self.0
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            Some(unsafe { &*self.0 })
        }
    }

    pub fn into_arc(self) -> Arc<T> {
        unsafe { Arc::from_raw(self.0 as *mut T) }
    }
}

impl<T: ?Sized> core::ops::Deref for ArcPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T: ?Sized> Clone for ArcPtr<T> {
    fn clone(&self) -> ArcPtr<T> {
        let arc = unsafe { Arc::from_raw(self.0) };
        let arc1 = Arc::clone(&arc);
        core::mem::forget(arc);
        ArcPtr(Arc::into_raw(arc1))
    }
}

impl<T: ?Sized> Drop for ArcPtr<T> {
    fn drop(&mut self) {
        unsafe { Arc::from_raw(self.0 as *mut c_void) };
    }
}

impl<T: ?Sized> From<Arc<T>> for ArcPtr<T> {
    fn from(arc: Arc<T>) -> ArcPtr<T> {
        ArcPtr(Arc::into_raw(arc))
    }
}

impl<T> From<T> for ArcPtr<T> {
    fn from(item: T) -> ArcPtr<T> {
        ArcPtr(Arc::into_raw(Arc::new(item)))
    }
}

pub fn nullable_arc<T>(thing: T) -> Nullable<ArcPtr<T>> {
    Nullable::from(ArcPtr::from(thing))
}

#[no_mangle]
pub extern "C" fn arc_clone(arc: ArcPtr<c_void>) -> Nullable<ArcPtr<c_void>> {
    match arc.is_null() {
        true => null(),
        false => Nullable::from(arc.clone()),
    }
}

#[no_mangle]
pub extern "C" fn arc_drop(arc: ArcPtr<c_void>) -> bool {
    match arc.is_null() {
        true => false,
        false => {
            arc.into_arc();
            true
        }
    }
}
