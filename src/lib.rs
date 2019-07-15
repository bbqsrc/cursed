#![feature(const_type_id)]
#![feature(proc_macro_hygiene)]

#[macro_use]
pub mod macros;
pub mod nullable;
pub mod sync;
pub mod exception;
pub mod vec;

pub mod prelude {
    pub use crate::nullable::*;
    pub use crate::sync::*;
    pub use crate::exception::*;
    pub use crate::vec::*;
    pub use crate::macros::*;
    pub use crate::{try_as_ref, try_not_null, not_null_or_return};
}
