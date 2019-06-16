use std::fmt;
use cocoa::{
    appkit::{
        self,
        NSWindow,
        NSWindowStyleMask,
    },
    base::{id, NO},
    foundation::{
        NSPoint,
        NSRect,
        NSSize,
        NSUInteger,
    },
};
use objc::rc::StrongPtr;
use shared::{
    os::macos::PlatformStringExt,
    PlatformString,
};
use crate::os::macos::WindowExt;

pub fn is_main_thread() -> bool {
    unsafe { msg_send![class!(NSThread), isMainThread] }
}

#[derive(Clone)]
pub struct Window {
    pub ns_window: StrongPtr,
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Window")
            .field("ns_window", &*self.ns_window) // No `Debug` for `StrongPtr`
            .finish()
    }
}

impl Window {
    #[inline]
    pub fn set_title(&self, title: PlatformString) {
        unsafe { self.ns_window.setTitle_(**title.as_ns_string()) };
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

impl crate::WindowBuilder {
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

        let window = unsafe {
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

        if let Some(title) = &self.title {
            unsafe { WindowExt::set_title(&window, **title.as_ns_string()) };
        }

        if self.sys.titlebar_hidden {
            window.set_titlebar_appears_transparent(true);
        }

        Ok(window)
    }
}
