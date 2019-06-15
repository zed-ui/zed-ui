use cocoa::{
    appkit::{
        self,
        NSWindow,
        NSWindowStyleMask,
    },
    base::{id, nil, NO},
    foundation::{
        NSPoint,
        NSRect,
        NSSize,
        NSString,
        NSUInteger,
    },
};
use objc::rc::StrongPtr;

use crate::os::macos::WindowExt;

pub fn is_main_thread() -> bool {
    unsafe { msg_send![class!(NSThread), isMainThread] }
}

#[derive(Clone)]
pub struct Window {
    pub ns_window: StrongPtr,
}

impl Window {
    #[inline]
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title = StrongPtr::new(NSString::init_str(nil, title));
            self.ns_window.setTitle_(*title);
        }
    }
}

pub struct WindowBuilder {
    pub style_mask: Option<NSUInteger>,
    pub full_size_content_view: bool,
    pub titlebar_hidden: bool,
}

impl Default for WindowBuilder {
    #[inline]
    fn default() -> Self {
        WindowBuilder {
            style_mask: None,
            full_size_content_view: false,
            titlebar_hidden: false,
        }
    }
}

impl WindowBuilder {
    fn ns_window_style_mask(&self) -> NSUInteger {
        use NSWindowStyleMask as M;

        let mut mask = self.style_mask.unwrap_or_else(|| {
            M::NSClosableWindowMask |
            M::NSMiniaturizableWindowMask |
            M::NSResizableWindowMask |
            M::NSTitledWindowMask
        }.bits());

        if self.titlebar_hidden {
            mask |= M::NSFullSizeContentViewWindowMask.bits();
        }

        mask
    }
}

impl crate::WindowBuilder<'_> {
    fn content_rect(&self) -> NSRect {
        NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(800.0, 600.0)
        )
    }

    pub(crate) fn sys_build(&self) -> Result<crate::Window, ()> {
        if !is_main_thread() {
            return Err(());
        }

        let content_rect = self.content_rect();
        let style_mask = self.sys.ns_window_style_mask();
        let backing = appkit::NSBackingStoreBuffered as NSUInteger;

        let mut window = unsafe {
            let ns_window: id = msg_send![class!(NSWindow), alloc];
            let ns_window: id = msg_send![
                ns_window,
                initWithContentRect:content_rect
                          styleMask:style_mask
                            backing:backing
                              defer:NO
            ];
            let ns_window = StrongPtr::new(ns_window);

            crate::Window::from(Window { ns_window })
        };

        if let Some(title) = self.title {
            window.set_title(title);
        }

        if self.sys.titlebar_hidden {
            window.set_titlebar_appears_transparent(true);
        }

        Ok(window)
    }
}
