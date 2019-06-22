//! Internal functionality shared between the ZedUI family of crates.

#![deny(missing_docs)]
#![doc(html_logo_url = "https://zed-ui.dev/static/logos/zed-ui.svg")]

mod sys;
pub mod os;

/// A string type specific to the targeted ZedUI platform.
#[derive(Clone, Debug)]
pub struct ZedString(sys::ZedString);

impl From<&str> for ZedString {
    #[inline]
    fn from(s: &str) -> Self {
        ZedString::from_utf8(s)
    }
}

impl ZedString {
    /// Creates a new instance from a UTF-8 encoded string.
    #[inline]
    pub fn from_utf8(s: &str) -> Self {
        ZedString(sys::ZedString::from_utf8(s))
    }
}
