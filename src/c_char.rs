use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

#[repr(transparent)]
#[derive(Debug)]
pub struct CCharPtr(*mut libc::c_char);

impl CCharPtr {
    /// This length *does not include* the NUL byte.
    pub fn len(&self) -> usize {
        unsafe { libc::strlen(self.0) }
    }

    pub fn as_ptr(&self) -> *mut libc::c_char {
        self.0
    }
    
    pub fn as_str<'a>(&'a self) -> &'a str {
        // Safe because pointer is always a valid C string.
        let slice = unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };

        // Safe because pointer is always a valid UTF8 string.
        unsafe { core::str::from_utf8_unchecked(slice) }
    }
}

impl Drop for CCharPtr {
    fn drop(&mut self) {
        log::debug!("Drop: {:?}", self.0);
        unsafe { Box::from_raw(self.0) };
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NulError(usize, Vec<u8>);

pub trait StringExt {
    fn into_c_char(self) -> Result<*mut libc::c_char, NulError>;
    fn into_arc_c_char(self) -> Result<Arc<libc::c_char>, NulError>;
}

impl StringExt for String {
    fn into_c_char(self) -> Result<*mut libc::c_char, NulError> {
        let mut inner = self.into_bytes();

        if let Some(i) = memchr::memchr(0, &inner) {
            return Err(NulError(i, inner));
        }

        inner.push(b'\0');
        let p = Box::into_raw(
            inner.into_boxed_slice()
        ) as *mut libc::c_char;
        Ok(p)
        
        // Ok(CCharPtr(p))

        
        // Ok(unsafe {
        //     core::mem::transmute::<*mut libc::c_char, CCharPtr>(
        //         {
        //             let p = Box::into_raw(
        //                 inner.into_boxed_slice()
        //             ) as *mut libc::c_char;

        //             log::debug!("SIN!: {:?}", p);
        //             p
        //         }
        //     )
        // })
    }

    fn into_arc_c_char(self) -> Result<Arc<*mut libc::c_char>, NulError> {
        Ok(Arc::new(self.into_c_char()?))
        
        // Ok(unsafe {
        //     core::mem::transmute::<*mut libc::c_char, CCharPtr>(
        //         {
        //             let p = Box::into_raw(Arc::new(
        //                 inner.into_boxed_slice()
        //             ) as *mut libc::c_char;

        //             log::debug!("SIN!: {:?}", p);
        //             p
        //         }
        //     )
        // })
    }
}

impl<'a> From<&'a CCharPtr> for &'a str {
    fn from(c_char_ptr: &'a CCharPtr) -> &'a str {
        c_char_ptr.as_str()
    }
}

impl From<CCharPtr> for String {
    fn from(c_char_ptr: CCharPtr) -> String {
        String::from(c_char_ptr.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::CCharPtr;
    use super::StringExt;
    use alloc::string::String;
    use alloc::boxed::Box;

    #[test]
    fn size() {
        assert_eq!(core::mem::size_of::<CCharPtr>(), core::mem::size_of::<Box<libc::c_char>>())
    }

    #[test]
    fn that_unsafety_tho() {
        let s = String::from("this is a\0 sample");
        let x = s.into_c_char();
        assert!(x.is_err())
    }
    
    #[test]
    fn that_unsafe_arcty_tho() {
        let s = String::from("this is a sample");
        let t = s.clone();
        let x = s.into_arc_c_char();
        assert_eq!(x.unwrap().as_str(), &*t)
    }
}
