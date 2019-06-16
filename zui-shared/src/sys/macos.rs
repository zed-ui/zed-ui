
use std::{
    ffi::CStr,
    fmt,
};
use cocoa::{
    base::nil,
    foundation::NSString,
};
use objc::rc::StrongPtr;

#[derive(Clone)]
pub struct PlatformString {
    pub ns_string: StrongPtr
}

impl fmt::Debug for PlatformString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { self.as_utf8_temp().fmt(f) }
    }
}

impl PlatformString {
    #[inline]
    pub fn from_utf8(s: &str) -> Self {
        unsafe {
            let ns_string = StrongPtr::new(NSString::alloc(nil).init_str(s));
            PlatformString { ns_string }
        }
    }

    #[inline]
    pub unsafe fn as_utf8_temp(&self) -> &str {
        let c_str = CStr::from_ptr(self.ns_string.UTF8String());
        std::str::from_utf8_unchecked(c_str.to_bytes())
    }
}
