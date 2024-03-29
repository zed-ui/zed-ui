//! macOS-specific extensions.

use std::ffi::c_void;
use objc::{
    rc::StrongPtr,
    runtime,
};
use crate::{ZedString, sys};

/// macOS-specific extensions for [`ZedString`](../../struct.ZedString.html).
pub trait ZedStringExt {
    /// Creates an instance wrapped around an
    /// [`NSString`](https://developer.apple.com/documentation/foundation/nsstring).
    unsafe fn from_ns_string(ns_string: StrongPtr) -> Self;

    /// Returns a handle to the underlying `NSString` without changing ownership
    /// semantics.
    fn as_ns_string(&self) -> &StrongPtr;

    /// Transfers ownership of the `NSString` out of `self`.
    fn into_ns_string(self) -> StrongPtr;

    /// Returns a reference to a UTF-8 encoded string that may have a shorter
    /// lifetime than `self`.
    ///
    /// See the documentation for `UTF8String` on
    /// [`NSString`](https://developer.apple.com/documentation/foundation/nsstring).
    unsafe fn as_utf8_temp(&self) -> &str;

    /// Returns `self` encoded as UTF-8.
    #[inline]
    fn to_utf8(&self) -> String {
        unsafe { self.as_utf8_temp().into() }
    }
}

impl ZedStringExt for ZedString {
    #[inline]
    unsafe fn from_ns_string(ns_string: StrongPtr) -> Self {
        ZedString(sys::ZedString { ns_string })
    }

    #[inline]
    fn as_ns_string(&self) -> &StrongPtr {
        &self.0.ns_string
    }

    #[inline]
    fn into_ns_string(self) -> StrongPtr {
        self.0.ns_string
    }

    #[inline]
    unsafe fn as_utf8_temp(&self) -> &str {
        self.0.as_utf8_temp()
    }
}

/// An Objective-C autorelease pool.
#[derive(Debug, PartialEq, Eq)]
pub struct AutoreleasePool(*mut c_void);

impl Drop for AutoreleasePool {
    #[inline]
    fn drop(&mut self) {
        unsafe { runtime::objc_autoreleasePoolPop(self.0) };
    }
}

impl Default for AutoreleasePool {
    #[inline]
    fn default() -> Self {
        unsafe { AutoreleasePool(runtime::objc_autoreleasePoolPush()) }
    }
}

impl AutoreleasePool {
    /// Creates a new autorelease pool.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Calls `f` within an `AutoreleasePool` frame.
    #[inline]
    pub fn with<F, T>(f: F) -> T where F: FnOnce() -> T {
        let pool = AutoreleasePool::new();
        let value = f();
        drop(pool);
        value
    }
}
