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

#[repr(transparent)]
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
}

pub type InRaw = In<c_void>;
pub type OutRaw = Out<c_void>;
pub type InOutRaw = InOut<c_void>;
