//! macOS-specific extensions.

use cocoa::base::id;
use objc::rc::StrongPtr;
use crate::{
    content::{Content, ContentInner},
    sys::WebView as SysWebView,
    WebView,
    WebViewBuilder,
};
use window::{
    Window,
    os::macos::WindowExt,
};

/// macOS-specific extensions for [`WebView`](../../struct.WebView.html).
pub trait WebViewExt {
    /// Creates an instance from an existing
    /// [`WKWebView`](https://developer.apple.com/documentation/webkit/wkwebview).
    unsafe fn from_wk_web_view(wk_web_view: StrongPtr) -> Self;

    /// Returns the
    /// [`WKWebView`](https://developer.apple.com/documentation/webkit/wkwebview)
    /// handle for `self`.
    fn wk_web_view(&self) -> id;
}

impl WebViewExt for WebView {
    unsafe fn from_wk_web_view(wk_web_view: StrongPtr) -> Self {
        let ns_window = StrongPtr::new(msg_send![*wk_web_view, window]);
        let window = Window::from_ns_window(ns_window);
        SysWebView { wk_web_view, window }.into()
    }

    #[inline]
    fn wk_web_view(&self) -> id {
        *self.sys.wk_web_view
    }
}

/// macOS-specific extensions for
/// [`WebViewBuilder`](../../struct.WebViewBuilder.html).
pub trait WebViewBuilderExt {
    /// Sets `UIDelegate` for the `WKWebView`.
    unsafe fn ui_delegate(&mut self, ui_delegate: id) -> &mut Self;

    /// Sets `navigationDelegate` for the `WKWebView`.
    unsafe fn nav_delegate(&mut self, nav_delegate: id) -> &mut Self;
}

impl<'a> WebViewBuilderExt for WebViewBuilder<'a> {
    #[inline]
    unsafe fn ui_delegate(&mut self, ui_delegate: id) -> &mut Self {
        self.sys.ui_delegate = ui_delegate;
        self
    }

    #[inline]
    unsafe fn nav_delegate(&mut self, nav_delegate: id) -> &mut Self {
        self.sys.nav_delegate = nav_delegate;
        self
    }
}

/// macOS-specific extensions for
/// [`Content`](../../struct.Content.html).
pub trait ContentExt {
    /// Creates a new instance from `content` and `base_url`.
    ///
    /// This method results in calling `loadHTMLString:baseURL:` on `WKWebView`.
    unsafe fn html(content: id, base_url: id) -> Self;

    /// Creates a new instance from an
    /// [`NSURL`](https://developer.apple.com/documentation/foundation/nsurl).
    unsafe fn url(ns_url: id) -> Self;
}

impl ContentExt for Content<'_> {
    #[inline]
    unsafe fn html(content: id, base_url: id) -> Self {
        Content(ContentInner::HtmlNSString { content, base_url })
    }

    #[inline]
    unsafe fn url(url: id) -> Self {
        Content(ContentInner::NSUrl(url))
    }
}
