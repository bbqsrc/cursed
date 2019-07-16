use std::ffi::c_void;
use std::ptr::NonNull;

#[repr(transparent)]
#[derive(Debug)]
pub struct In<T>(*const T);
unsafe impl<T> Sync for In<T> {}
unsafe impl<T> Send for In<T> {}

impl<T> In<T> {
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
fn as_arc_internal<T, U>(arc: &In<U>) -> Option<std::sync::Arc<T>> {
    if let Some(arc) = arc.as_ptr() {
        let arc = unsafe { std::sync::Arc::from_raw(arc.as_ptr() as *const _) };
        let arc1: std::sync::Arc<T> = std::sync::Arc::clone(&arc);
        std::mem::forget(arc);
        Some(arc1)
    } else {
        None
    }
}

impl<T> In<std::sync::Arc<T>> {
    pub fn as_arc(&self) -> Option<std::sync::Arc<T>> {
        as_arc_internal(self)
    }
}

impl<T> In<crate::sync::ArcPtr<T>> {
    pub fn as_arc(&self) -> Option<std::sync::Arc<T>> {
        as_arc_internal(self)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Out<T>(*mut T);
unsafe impl<T> Sync for Out<T> {}
unsafe impl<T> Send for Out<T> {}

impl<T> Out<T> {
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
pub struct InOut<T>(*mut T);
unsafe impl<T> Sync for InOut<T> {}
unsafe impl<T> Send for InOut<T> {}

impl<T> InOut<T> {
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
