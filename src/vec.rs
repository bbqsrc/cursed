use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::sync::Arc as RealArc;
use std::vec::Vec as RealVec;

use crate::exception::Exception;
use crate::inout::{In, InOut, InRaw, Out};
use crate::nullable::Nullable;
use crate::sync::Arc;

#[derive(Debug, Clone)]
pub struct TypedVoid(TypeId, RealArc<dyn Any + 'static + Send + Sync>);
unsafe impl Send for TypedVoid {}
unsafe impl Sync for TypedVoid {}

impl TypedVoid {
    fn resolve<T: 'static + Send + Sync>(&self) -> Result<RealArc<T>, TypedVoid> {
        let x = self.clone();
        match x.1.downcast() {
            Ok(v) => Ok(v),
            Err(e) => Err(TypedVoid(x.0, e)),
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct ThreadSafePtr<T>(*const T);
unsafe impl<T> Send for ThreadSafePtr<T> {}
unsafe impl<T> Sync for ThreadSafePtr<T> {}
impl<T> std::ops::Deref for ThreadSafePtr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AnyVec(RealArc<RwLock<RealVec<TypedVoid>>>);

impl AnyVec {
    #[inline]
    pub fn new() -> AnyVec {
        AnyVec(RealArc::new(RwLock::new(RealVec::new())))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.read().len()
    }

    #[inline]
    pub fn push<T: Send + Sync + 'static>(&mut self, item: T) {
        self.0
            .write()
            .push(TypedVoid(TypeId::of::<T>(), RealArc::new(item)));
    }

    #[inline]
    pub fn pop<T: Send + Sync + 'static>(&mut self) -> Option<Result<RealArc<T>, TypedVoid>> {
        let typed_void = self.0.write().pop()?;
        Some(typed_void.resolve())
    }

    #[inline]
    pub fn get<T: Send + Sync + 'static>(
        &self,
        index: usize,
    ) -> Option<Result<RealArc<T>, TypedVoid>> {
        let guard = self.0.read();
        let typed_void = guard.get(index)?;
        Some(typed_void.resolve())
    }
}

type RawValue = dyn Any + 'static + Send + Sync;
#[derive(Debug, Clone)]
pub struct RawVec {
    vec: RealArc<RwLock<RealVec<RealArc<RawValue>>>>,

    #[cfg(debug_assertions)]
    ty: TypeId,
}

impl RawVec {
    #[inline]
    fn new<T: 'static>() -> RawVec {
        RawVec {
            vec: RealArc::new(RwLock::new(RealVec::new())),

            #[cfg(debug_assertions)]
            ty: TypeId::of::<T>(),
        }
    }

    #[inline]
    fn iter<F, O>(&self, f: F) -> O
    where
        F: FnOnce(std::slice::Iter<RealArc<RawValue>>) -> O,
    {
        f(self.vec.read().iter())
    }

    #[inline]
    fn len(&self) -> usize {
        self.vec.read().len()
    }

    #[inline]
    fn push(&mut self, item: RealArc<RawValue>) {
        self.vec.write().push(item);
    }

    #[inline]
    fn pop(&mut self) -> Option<RealArc<RawValue>> {
        self.vec.write().pop()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<RealArc<RawValue>> {
        let guard = self.vec.read();
        let item = guard.get(index)?;
        Some(RealArc::clone(&item))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Vec<T>(RawVec, std::marker::PhantomData<T>);

impl<T: Send + Sync + 'static> Vec<T> {
    pub fn new() -> Vec<T> {
        println!("New vec: {:?}", TypeId::of::<T>());
        Vec(RawVec::new::<T>(), std::marker::PhantomData)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, item: T) {
        self.0.push(RealArc::new(item));
    }

    pub fn pop(&mut self) -> Option<RealArc<T>> {
        self.0.pop().and_then(|v| v.downcast().ok())
    }

    pub fn get(&self, index: usize) -> Option<RealArc<T>> {
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

    pub fn to_vec(&self) -> RealVec<RealArc<T>> {
        let mut out = vec![];
        for i in 0..self.len() {
            let v = self.get(i).expect("valid value");
            out.push(RealArc::clone(&v));
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

impl<T: Send + Sync + 'static> From<std::vec::Vec<T>> for Vec<T> {
    fn from(vec: std::vec::Vec<T>) -> Vec<T> {
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

impl<T: Send + Sync + 'static + Clone> From<&std::vec::Vec<T>> for Vec<T> {
    fn from(vec: &std::vec::Vec<T>) -> Vec<T> {
        Vec::from(&**vec)
    }
}

#[no_mangle]
pub extern "C" fn vec_len(handle: In<RawVec>, exception: Out<Exception>) -> usize {
    let handle = try_not_null!(handle.as_ptr(), exception.as_ptr(), 0usize);
    let handle = unsafe { &*handle.as_ptr() };
    handle.len()
}

#[no_mangle]
pub extern "C" fn vec_push(mut handle: InOut<RawVec>, value: InRaw, exception: Out<Exception>) {
    let handle = unsafe { try_as_mut_ref!(handle, exception.as_ptr(), ()) };
    handle.push(RealArc::new(value));
}

macro_rules! vec_nullable {
    ($thing:expr) => {
        match $thing {
            Some(v) => {
                let arc = $crate::sync::Arc::from(v);
                $crate::nullable::Nullable::new(arc.as_ptr() as *const _)
            }
            None => $crate::nullable::null(),
        }
    };
}

#[no_mangle]
pub extern "C" fn vec_pop(
    mut handle: InOut<RawVec>,
    exception: Out<Exception>,
) -> Nullable<Arc<RawValue>> {
    let handle = unsafe { try_as_mut_ref!(handle, exception.as_ptr()) };
    vec_nullable!(handle.pop())
    // item)

    // .into().into::<Nullable<_>>()
}

#[no_mangle]
pub extern "C" fn vec_get(
    handle: In<RawVec>,
    index: u64,
    exception: Out<Exception>,
) -> Nullable<Arc<RawValue>> {
    let handle = unsafe { try_as_ref!(handle, exception.as_ptr()) };
    // let handle = unsafe { handle.as_mut() };
    vec_nullable!(handle.get(index as usize))
    // match handle.get(index as usize) {
    //     Some(v) => {
    //         println!("Got: {:?}", v);
    //         let arc = Arc::from(v);
    //         Nullable::new(arc.as_ptr() as *const _)
    //     },
    //     None => {
    //         println!("Nothing for index {}", index);
    //         crate::null()
    //     }
    // }
}

#[macro_export]
macro_rules! vec_constructor {
    { $( $ty_name:ident => $ty:ty ),* } => {
        $(
            #[no_mangle]
            pub static $ty_name: std::any::TypeId = std::any::TypeId::of::<$ty>();
        )*

        /// A constructor for `Vec` for the C FFI, accepting types provided from generated constants.
        #[no_mangle]
        pub extern "C" fn vec_new(ty: TypeId) -> $crate::nullable::Nullable<std::ffi::c_void> {
            println!("{:?}", ty);
            $(
                if &ty == &$ty_name { return $crate::nullable::Nullable::new(Vec::<$ty>::new().into_raw() as *mut std::ffi::c_void) }
            )*
            $crate::nullable::null()
        }

        /// A function to free vectors.
        #[no_mangle]
        pub extern "C" fn vec_free(handle: *mut RawVec, ty: TypeId, exception: *mut Exception) {
            println!("{:?}", ty);
            // TODO: check handle isn't null
            if handle.is_null() {
                return;
            }

            $(
                if &ty == &$ty_name { unsafe { $crate::vec::Vec::<$ty>::from_raw(handle as *mut _); } }
            )*
            // TODO: handle exception
        }

        #[no_mangle]
        pub extern "C" fn vec_debug_print(handle: *mut RawVec, ty: TypeId) {
            println!("{:?}", ty);
            // TODO: check handle isn't null
            if handle.is_null() {
                return;
            }

            $(
                if &ty == &$ty_name {
                    unsafe {
                        let raw_vec = &*handle;
                        let v = raw_vec.iter(|x| x.map(|x| std::mem::transmute::<_, &$ty>(x)).collect::<std::vec::Vec<_>>());

                        // let ptr = std::mem::transmute::<&RawVec, &Vec<$ty>>(raw_vec);
                        println!("{:?}", &v);
                    }
                }
            )*
            // TODO: handle exception
        }
    };
}

use libc::c_char;

vec_constructor! {
    TYPE_STRING => String,
    TYPE_U64 => u64
}

#[no_mangle]
pub extern "C" fn string_new(c_str: *const c_char) -> String {
    let c_str = unsafe { std::ffi::CStr::from_ptr(c_str) };
    println!("EH: {:?}", &c_str);
    c_str.to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants() {
        println!("{:?}", TYPE_STRING);
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
        assert_eq!(vec.get(1), Some(RealArc::new(42usize)));
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
