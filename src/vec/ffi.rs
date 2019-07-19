use alloc::sync::Arc;

use super::RawValue;
use crate::{
    exception::Exception,
    inout::{In, InOut, InRaw, OutPtr},
    nullable::Nullable,
    sync::ArcPtr,
    vec::RawVec,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "no-std")] {
        use alloc::format;
    } else {
        use std::format;
    }
}

#[no_mangle]
pub extern "C" fn vec_len(handle: In<RawVec>, exception: OutPtr<Exception>) -> usize {
    let handle = try_not_null!(handle.as_ptr(), &exception, 0usize);
    let handle = unsafe { &*handle.as_ptr() };
    handle.len()
}

#[no_mangle]
pub extern "C" fn vec_push(mut handle: InOut<RawVec>, value: InRaw, exception: OutPtr<Exception>) {
    let handle = unsafe { try_as_mut_ref!(handle, &exception, ()) };
    handle.push(Arc::new(value));
}

macro_rules! vec_nullable {
    ($thing:expr) => {
        match $thing {
            Some(v) => {
                let arc = $crate::sync::ArcPtr::from(v);
                $crate::nullable::Nullable::new(&arc)
            }
            None => $crate::nullable::null(),
        }
    };
}

#[no_mangle]
pub extern "C" fn vec_pop(
    mut handle: InOut<RawVec>,
    exception: OutPtr<Exception>,
) -> Nullable<ArcPtr<RawValue>> {
    let handle = unsafe { try_as_mut_ref!(handle, &exception) };
    vec_nullable!(handle.pop())
}

#[no_mangle]
pub extern "C" fn vec_get(
    handle: In<RawVec>,
    index: u64,
    exception: OutPtr<Exception>,
) -> Nullable<ArcPtr<RawValue>> {
    let handle = unsafe { try_as_ref!(handle, &exception) };
    vec_nullable!(handle.get(index as usize))
}

#[macro_export]
macro_rules! generate_vec_ffi {
    { $( $ty_name:ident => $ty:ty ),* } => {
        $(
            #[no_mangle]
            pub static $ty_name: core::any::TypeId = core::any::TypeId::of::<$ty>();
        )*

        /// A constructor for `Vec` for the C FFI, accepting types provided from generated constants.
        #[no_mangle]
        pub extern "C" fn vec_new(ty: core::any::TypeId) -> $crate::nullable::Nullable<core::ffi::c_void> {
            log::debug!("{:?}", ty);
            $(
                if &ty == &$ty_name { return $crate::nullable::Nullable::new($crate::vec::Vec::<$ty>::new().into_raw() as *mut core::ffi::c_void) }
            )*
            $crate::nullable::null()
        }

        /// A function to free vectors.
        #[no_mangle]
        pub extern "C" fn vec_free(handle: *mut RawVec, ty: core::any::TypeId, exception: *mut Exception) {
            log::debug!("{:?}", ty);
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
        pub extern "C" fn vec_debug_print(handle: *mut RawVec, ty: core::any::TypeId) {
            log::debug!("{:?}", ty);
            // TODO: check handle isn't null
            if handle.is_null() {
                return;
            }

            $(
                if &ty == &$ty_name {
                    unsafe {
                        let raw_vec = &*handle;
                        let v = raw_vec.iter(|x| x.map(|x| core::mem::transmute::<_, &$ty>(x)).collect::<alloc::vec::Vec<_>>());
                        log::debug!("{:?}", &v);
                    }
                }
            )*
            // TODO: handle exception
        }
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "demo")] {
        generate_vec_ffi! {
            TYPE_U64 => u64,
            TYPE_STRING => std::string::String
        }
    }
}
