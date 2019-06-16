use cocoa::{
    base::nil,
    foundation::NSString,
};
use objc::rc::StrongPtr;

#[derive(Clone)]
pub struct PlatformString {
    pub ns_string: StrongPtr
}

impl PlatformString {
    #[inline]
    pub fn from_utf8(s: &str) -> Self {
        let ns_string = unsafe { StrongPtr::new(NSString::init_str(nil, s)) };
        PlatformString { ns_string }
    }
}
