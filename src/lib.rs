#![feature(const_type_id)]
#![feature(proc_macro_hygiene)]

#![no_std]

#[cfg(not(feature = "no-std"))]
extern crate std;
extern crate alloc;

#[macro_use]
pub mod macros;
pub mod exception;
#[cfg(feature = "futures")]
pub mod future;
pub mod inout;
pub mod nullable;
pub mod sync;
pub mod vec;
mod vendor;

pub mod prelude {
    pub use crate::exception::*;
    #[cfg(feature = "futures")]
    pub use crate::future::*;
    pub use crate::inout::*;
    pub use crate::macros::*;
    pub use crate::nullable::*;
    pub use crate::sync::*;
    pub use crate::vec::*;
    pub use crate::{try_as_arc, try_as_ref, try_as_str, try_into_arc, try_not_null};
}
