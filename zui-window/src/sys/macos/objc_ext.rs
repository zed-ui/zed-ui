use std::ops::Deref;
use cocoa::{
    base::{id, nil},
    foundation::NSAutoreleasePool,
};

pub fn is_main_thread() -> bool {
    unsafe { msg_send![class!(NSThread), isMainThread] }
}

#[derive(Debug)]
pub struct NSObject(id);

impl Drop for NSObject {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);
            msg_send![self.0, release];
            pool.drain();
        }
    }
}

impl Clone for NSObject {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            msg_send![self.0, retain];
            Self::new(self.0)
        }
    }
}

impl Deref for NSObject {
    type Target = id;

    #[inline]
    fn deref(&self) -> &id {
        &self.0
    }
}

impl NSObject {
    #[inline]
    pub const unsafe fn new(id: id) -> Self {
        NSObject(id)
    }
}
