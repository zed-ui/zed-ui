//! ZedUI web view utilities.

#![deny(missing_docs)]
#![doc(html_logo_url = "https://zed-ui.dev/static/logos/zed-ui.svg")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

extern crate zui_window as window;
use window::Window;

mod sys;
mod builder;
pub mod content;
pub mod os;

#[doc(inline)]
pub use self::{
    builder::*,
    content::Content,
};

/// A view used to display an [HTML] document.
///
/// [HTML]: https://en.wikipedia.org/wiki/HTML
#[derive(Clone)]
pub struct WebView {
    sys: sys::WebView,
}

#[doc(hidden)]
impl From<sys::WebView> for WebView {
    #[inline]
    fn from(sys: sys::WebView) -> Self {
        WebView { sys }
    }
}

impl WebView {
    /// Creates a builder suitable for constructing a new `Window` instance.
    #[inline]
    pub fn builder<'a>() -> WebViewBuilder<'a> {
        Default::default()
    }

    /// Creates a web view and attaches it to `window`.
    #[inline]
    pub fn new(window: Window) -> Result<Self, WebViewBuildError> {
        Self::builder().build(window)
    }

    /// Returns the window associated with `self`.
    #[inline]
    pub fn window(&self) -> &Window {
        &self.sys.window
    }

    /// Sets the content displayed within `self`.
    #[inline]
    pub fn set_content(&self, content: &Content) {
        self.sys.set_content(content);
    }
}
