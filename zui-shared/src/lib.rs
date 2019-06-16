//! Internal functionality shared between the ZedUI family of crates.

#![deny(missing_docs)]

mod sys;
pub mod os;

/// A string type specific to the targeted platform.
#[derive(Clone, Debug)]
pub struct PlatformString(pub(crate) sys::PlatformString);

impl From<&str> for PlatformString {
    #[inline]
    fn from(s: &str) -> Self {
        PlatformString::from_utf8(s)
    }
}

impl PlatformString {
    /// Creates a new instance from a UTF-8 encoded string.
    #[inline]
    pub fn from_utf8(s: &str) -> Self {
        PlatformString(sys::PlatformString::from_utf8(s))
    }
}
