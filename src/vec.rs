use alloc::{borrow::ToOwned, boxed::Box, string::String, sync::Arc, vec, vec::Vec as RealVec};
use core::{
    any::{Any, TypeId},
    marker::PhantomData,
};
use parking_lot::RwLock;
use alloc::format;
use log::debug;

use crate::{
    exception::Exception,
    inout::{In, InOut, InRaw, OutPtr},
    nullable::Nullable,
    sync::ArcPtr,
};

pub mod ffi;

pub type RawValue = dyn Any + 'static + Send + Sync;

#[derive(Debug, Clone)]
pub struct TaggedAny(TypeId, Arc<RawValue>);
unsafe impl Send for TaggedAny {}
unsafe impl Sync for TaggedAny {}

impl TaggedAny {
    fn resolve<T: 'static + Send + Sync>(&self) -> Result<Arc<T>, TaggedAny> {
        let x = self.clone();
        match x.1.downcast() {
            Ok(v) => Ok(v),
            Err(e) => Err(TaggedAny(x.0, e)),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AnyVec(Arc<RwLock<RealVec<TaggedAny>>>);

impl AnyVec {
    #[inline]
    pub fn new() -> AnyVec {
        AnyVec(Arc::new(RwLock::new(RealVec::new())))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.read().len()
    }

    #[inline]
    pub fn push<T: Send + Sync + 'static>(&mut self, item: T) {
        self.0
            .write()
            .push(TaggedAny(TypeId::of::<T>(), Arc::new(item)));
    }

    #[inline]
    pub fn pop<T: Send + Sync + 'static>(&mut self) -> Option<Result<Arc<T>, TaggedAny>> {
        let typed_void = self.0.write().pop()?;
        Some(typed_void.resolve())
    }

    #[inline]
    pub fn get<T: Send + Sync + 'static>(&self, index: usize) -> Option<Result<Arc<T>, TaggedAny>> {
        let guard = self.0.read();
        let typed_void = guard.get(index)?;
        Some(typed_void.resolve())
    }
}

#[derive(Debug, Clone)]
pub struct RawVec {
    vec: Arc<RwLock<RealVec<Arc<RawValue>>>>,

    #[cfg(debug_assertions)]
    ty: TypeId,
}

impl RawVec {
    #[inline]
    fn new<T: 'static>() -> RawVec {
        RawVec {
            vec: Arc::new(RwLock::new(RealVec::new())),

            #[cfg(debug_assertions)]
            ty: TypeId::of::<T>(),
        }
    }

    #[inline]
    fn iter<F, O>(&self, f: F) -> O
    where
        F: FnOnce(core::slice::Iter<Arc<RawValue>>) -> O,
    {
        f(self.vec.read().iter())
    }

    #[inline]
    fn len(&self) -> usize {
        self.vec.read().len()
    }

    #[inline]
    fn push(&mut self, item: Arc<RawValue>) {
        self.vec.write().push(item);
    }

    #[inline]
    fn pop(&mut self) -> Option<Arc<RawValue>> {
        self.vec.write().pop()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<Arc<RawValue>> {
        let guard = self.vec.read();
        let item = guard.get(index)?;
        Some(Arc::clone(&item))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Vec<T>(RawVec, PhantomData<T>);

impl<T: Send + Sync + 'static> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec(RawVec::new::<T>(), PhantomData)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, item: T) {
        self.0.push(Arc::new(item));
    }

    pub fn pop(&mut self) -> Option<Arc<T>> {
        self.0.pop().and_then(|v| v.downcast().ok())
    }

    pub fn get(&self, index: usize) -> Option<Arc<T>> {
        self.0.get(index).and_then(|v| v.downcast().ok())
    }

    pub fn into_raw(self) -> *const Vec<T> {
        Box::into_raw(Box::new(self.0)) as *const _
    }

    pub unsafe fn from_raw(ptr: *const Vec<T>) -> Vec<T> {
        let raw_vec: Box<RawVec> = Box::from_raw(ptr as *mut _);

        #[cfg(debug_assertions)]
        assert_eq!(raw_vec.ty, TypeId::of::<T>());

        Vec(*raw_vec, PhantomData)
    }

    pub fn to_vec(&self) -> RealVec<Arc<T>> {
        let mut out = vec![];
        for i in 0..self.len() {
            let v = self.get(i).expect("valid value");
            out.push(Arc::clone(&v));
        }
        out
    }

    pub fn to_owned_vec(&self) -> RealVec<T::Owned>
    where
        T: ToOwned,
    {
        let mut out = vec![];
        for i in 0..self.len() {
            let v = self.get(i).expect("valid value");
            out.push((*v).to_owned());
        }
        out
    }
}

impl<T: Send + Sync + 'static> From<RealVec<T>> for Vec<T> {
    fn from(vec: RealVec<T>) -> Vec<T> {
        let mut out = Vec::new();
        for item in vec.into_iter() {
            out.push(item);
        }
        out
    }
}

impl<T: Send + Sync + 'static + Clone> From<&[T]> for Vec<T> {
    fn from(vec: &[T]) -> Vec<T> {
        let mut out = Vec::new();
        for item in vec.iter() {
            out.push(item.clone());
        }
        out
    }
}

impl<T: Send + Sync + 'static + Clone> From<&RealVec<T>> for Vec<T> {
    fn from(vec: &RealVec<T>) -> Vec<T> {
        Vec::from(&**vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    #[test]
    fn constants() {
        debug!("{:?}", TYPE_STRING);
    }

    #[test]
    fn woo_test() {
        let mut vec = AnyVec::new();
        vec.push::<usize>(1usize);
        vec.push::<String>("haha this is so bad".into());

        assert_eq!(vec.len(), 2usize);
        match vec.pop::<String>() {
            Some(v) => match v {
                Ok(vv) => assert_eq!(*vv, "haha this is so bad"),
                Err(_) => panic!("ffs"),
            },
            None => panic!("must be a value"),
        };

        assert_eq!(*vec.pop::<usize>().unwrap().unwrap(), 1usize);
    }

    #[test]
    fn another_test() {
        let mut vec = AnyVec::new();
        vec.push::<String>("hello".into());
        assert_eq!(*vec.get::<String>(0).unwrap().unwrap(), "hello")
    }

    #[test]
    fn typesafe_life() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(42usize);
        vec.push(102390123usize);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(1), Some(Arc::new(42usize)));
    }

    #[test]
    fn ffi_life() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(42usize);
        vec.push(102390123usize);
        let raw_vec = vec.into_raw();
        unsafe { Vec::<usize>::from_raw(raw_vec) };
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn ffi_life2() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(42usize);
        vec.push(102390123usize);
        let raw_vec = vec.into_raw();
        unsafe { Vec::<String>::from_raw(raw_vec as *mut _) };
    }
}
