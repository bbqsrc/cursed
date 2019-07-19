use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;

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
        let slice = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };

        // Safe because pointer is always a valid UTF8 string.
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

impl Drop for CCharPtr {
    fn drop(&mut self) {
        println!("Drop: {:?}", self.0);
        unsafe { Box::from_raw(self.0) };
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NulError(usize, Vec<u8>);

pub trait StringExt {
    fn into_c_char(self) -> Result<CCharPtr, NulError>;
    fn into_arc_c_char(self) -> Result<Arc<CCharPtr>, NulError>;
}

impl StringExt for String {
    fn into_c_char(self) -> Result<CCharPtr, NulError> {
        let mut inner = self.into_bytes();

        if let Some(i) = memchr::memchr(0, &inner) {
            return Err(NulError(i, inner));
        }

        inner.push(b'\0');
        
        Ok(unsafe {
            std::mem::transmute::<*mut libc::c_char, CCharPtr>(
                {
                    let p = Box::into_raw(
                        inner.into_boxed_slice()
                    ) as *mut libc::c_char;

                    println!("SIN!: {:?}", p);
                    p
                }
            )
        })
    }

    fn into_arc_c_char(self) -> Result<Arc<CCharPtr>, NulError> {
        Ok(Arc::new(self.into_c_char()?))
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

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<CCharPtr>(), std::mem::size_of::<Box<libc::c_char>>())
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
