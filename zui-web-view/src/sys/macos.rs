use cocoa::{
    base::{id, nil},
    foundation::NSString,
};
use objc::rc::StrongPtr;
use crate::{
    content::{Content, ContentInner},
    WebViewBuildError,
    Window,
};

#[derive(Clone)]
pub struct WebView {
    pub wk_web_view: StrongPtr,
    pub window: Window,
}

impl WebView {
    unsafe fn set_content_ns_string(&self, content: id, base_url: id) {
        msg_send![*self.wk_web_view, loadHTMLString:content
                                            baseURL:base_url];
    }

    unsafe fn set_content_url(&self, url: id) {
        let req: id = msg_send![class!(NSURLRequest), requestWithURL:url];
        msg_send![*self.wk_web_view, loadRequest:req];
    }

    pub fn set_content(&self, content: &Content) {
        match content.0 {
            ContentInner::Html(html) => unsafe {
                let content = NSString::init_str(nil, html);
                self.set_content_ns_string(content, nil)
            },
            ContentInner::Url(url) => unsafe {
                let url = NSString::init_str(nil, url);
                let url = msg_send![class!(NSURL), URLWithString:url];
                self.set_content_url(url)
            },
            ContentInner::HtmlNSString { content, base_url } => unsafe {
                self.set_content_ns_string(content, base_url)
            },
            ContentInner::NSUrl(url) => unsafe {
                self.set_content_url(url)
            },
        }
    }
}

pub struct WebViewBuilder {
    pub wk_web_view_config: id,
    pub ui_delegate: id,
    pub nav_delegate: id,
}

impl Default for WebViewBuilder {
    #[inline]
    fn default() -> Self {
        WebViewBuilder {
            wk_web_view_config: nil,
            ui_delegate: nil,
            nav_delegate: nil,
        }
    }
}

impl<'a> crate::WebViewBuilder<'a> {
    pub(crate) fn sys_build(&self, _window: Window) -> Result<crate::WebView, WebViewBuildError> {
        unimplemented!("TODO: Build web view");
    }
}
