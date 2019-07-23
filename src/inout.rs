use alloc::sync::Arc;
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ptr::NonNull;

#[repr(transparent)]
#[derive(Debug)]
pub struct In<T: ?Sized>(*const T);
unsafe impl<T: ?Sized> Sync for In<T> {}
unsafe impl<T: ?Sized> Send for In<T> {}

impl<T: ?Sized> From<Box<T>> for In<T> {
    fn from(boxed: Box<T>) -> In<T> {
        In(Box::into_raw(boxed))
    }
}

impl<T: ?Sized> In<T> {
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[inline]
    pub fn as_ptr(&self) -> Option<NonNull<T>> {
        NonNull::new(self.0 as *mut _)
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> Option<&T> {
        match self.is_null() {
            false => Some(&*self.0),
            true => None,
        }
    }
}

#[doc(hidden)]
fn as_arc_internal<T, U>(arc: &In<U>) -> Option<Arc<T>> {
    if let Some(arc) = arc.as_ptr() {
        let arc = unsafe { Arc::from_raw(arc.as_ptr() as *const _) };
        let arc1: Arc<T> = Arc::clone(&arc);
        core::mem::forget(arc);
        Some(arc1)
    } else {
        None
    }
}

impl<T> In<Arc<T>> {
    pub fn as_arc(&self) -> Option<Arc<T>> {
        as_arc_internal(self)
    }
}

impl<T> In<crate::sync::ArcPtr<T>> {
    pub fn as_arc(&self) -> Option<Arc<T>> {
        as_arc_internal(self)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Out<T: ?Sized>(*mut T);
unsafe impl<T: ?Sized> Sync for Out<T> {}
unsafe impl<T: ?Sized> Send for Out<T> {}

impl<T: ?Sized> Out<T> {
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[inline]
    pub fn as_ptr(&self) -> Option<NonNull<T>> {
        NonNull::new(self.0)
    }

    #[inline]
    pub unsafe fn as_mut_ref(&mut self) -> Option<&mut T> {
        match self.is_null() {
            false => Some(&mut *self.0),
            true => None,
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct OutPtr<T: ?Sized>(*mut *mut T);
unsafe impl<T: ?Sized> Sync for OutPtr<T> {}
unsafe impl<T: ?Sized> Send for OutPtr<T> {}

impl<T: ?Sized> OutPtr<T> {
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[inline]
    pub fn as_ptr(&self) -> Option<NonNull<*mut T>> {
        NonNull::new(self.0)
    }

    #[inline]
    pub unsafe fn as_mut_ref(&mut self) -> Option<&mut *mut T> {
        match self.is_null() {
            false => Some(&mut *self.0),
            true => None,
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct InOut<T: ?Sized>(*mut T);
unsafe impl<T: ?Sized> Sync for InOut<T> {}
unsafe impl<T: ?Sized> Send for InOut<T> {}

impl<T: ?Sized> InOut<T> {
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    #[inline]
    pub fn as_ptr(&self) -> Option<NonNull<T>> {
        NonNull::new(self.0)
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> Option<&T> {
        match self.is_null() {
            false => Some(&*self.0),
            true => None,
        }
    }

    #[inline]
    pub unsafe fn as_mut_ref(&mut self) -> Option<&mut T> {
        match self.is_null() {
            false => Some(&mut *self.0),
            true => None,
        }
    }

    #[inline]
    pub fn to_in(&self) -> In<T> {
        In(self.0)
    }

    #[inline]
    pub fn to_out(&self) -> Out<T> {
        Out(self.0)
    }
}

pub type InRaw = In<c_void>;
pub type OutRaw = Out<c_void>;
pub type InOutRaw = InOut<c_void>;
