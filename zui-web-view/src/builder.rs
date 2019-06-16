use crate::{
    content::*,
    sys,
    WebView,
    Window,
};

/// A type for configuring how a `WebView` instance should be constructed.
pub struct WebViewBuilder<'a> {
    pub(crate) sys: sys::WebViewBuilder,
    pub(crate) content: Content<'a>,
}

impl Default for WebViewBuilder<'_> {
    #[inline]
    fn default() -> Self {
        WebViewBuilder {
            sys: Default::default(),
            content: Default::default(),
        }
    }
}

impl<'a> WebViewBuilder<'a> {
    /// Creates a builder suitable for constructing a new `WebView` instance.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the content of the web view to HTML.
    #[inline]
    pub fn html(&mut self, html: &'a str) -> &mut Self {
        self.content(Content::html(html))
    }

    /// Sets the content of the web view to a URL.
    #[inline]
    pub fn url(&mut self, url: &'a str) -> &mut Self {
        self.content(Content::url(url))
    }

    /// Sets the content displayed by the web view.
    #[inline]
    pub fn content(&mut self, content: Content<'a>) -> &mut Self {
        self.content = content;
        self
    }

    /// Creates a new web view, returning an error upon failure.
    ///
    /// The web view is attached to `window`.
    pub fn build(&self, window: Window) -> Result<WebView, WebViewBuildError> {
        self.sys_build(window)
    }
}

/// The error returned when
/// [`WebViewBuilder::build`](struct.WebViewBuilder.html#build) fails.
pub enum WebViewBuildError {
}
