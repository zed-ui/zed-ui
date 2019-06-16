//! macOS-specific extensions.

use cocoa::{
    appkit::{
        NSWindow,
        NSWindowStyleMask,
    },
    base::id,
    foundation::{
        NSPoint,
        NSSize,
        NSUInteger,
    },
};
use objc::rc::StrongPtr;
use crate::{
    dpi::*,
    Window,
    WindowBuilder,
    sys::Window as SysWindow,
};

/// macOS-specific extensions for [`Window`](../../struct.Window.html).
pub trait WindowExt {
    /// Creates a new instance from an `NSWindow` handle.
    unsafe fn from_ns_window(ns_window: StrongPtr) -> Self;

    /// Returns the `NSWindow` handle for `self`.
    fn ns_window(&self) -> id;

    /// Returns the title of `self` as an `NSString`.
    #[inline]
    fn title(&self) -> id {
        unsafe { self.ns_window().title() }
    }

    /// Sets the title of `self` to an `NSString`.
    #[inline]
    unsafe fn set_title(&self, title: id) {
        self.ns_window().setTitle_(title);
    }

    /// Sets the titlebar transparency and makes the content view full-size
    /// according to `hidden`.
    #[inline]
    fn set_titlebar_hidden(&self, hidden: bool) {
        self.set_full_size_content_view(hidden);
        self.set_titlebar_appears_transparent(hidden);
    }

    /// Calls `setTitlebarAppearsTransparent:` on `self`.
    #[inline]
    fn set_titlebar_appears_transparent(&self, transparent: bool) {
        unsafe {
            self.ns_window().setTitlebarAppearsTransparent_(transparent as _);
        }
    }

    /// Sets whether the content view consumes the full size of the `NSWindow`.
    ///
    /// This enables or disables `NSFullSizeContentViewWindowMask`.
    #[inline]
    fn set_full_size_content_view(&self, full_size: bool) {
        let bits = NSWindowStyleMask::NSFullSizeContentViewWindowMask.bits();
        self.update_style_mask(|mask| {
            if full_size {
                mask | bits
            } else {
                mask & !bits
            }
        })
    }

    /// Returns a handle to the content view of `self`.
    #[inline]
    fn content_view(&self) -> id {
        unsafe { self.ns_window().contentView() }
    }

    /// Sets the content view of `self`.
    #[inline]
    unsafe fn set_content_view(&self, content_view: id) {
        self.ns_window().setContentView_(content_view)
    }

    /// Returns the `NSWindowStyleMask` of the `NSWindow`.
    #[inline]
    fn style_mask(&self) -> NSUInteger {
        unsafe { msg_send![self.ns_window(), styleMask] }
    }

    /// Sets the `NSWindowStyleMask` used by the `NSWindow`.
    #[inline]
    fn set_style_mask<M: Into<NSUInteger>>(&self, mask: M) {
        unsafe { msg_send![self.ns_window(), setStyleMask:mask.into()] };
    }

    /// Sets the `NSWindowStyleMask` used by the `NSWindow` to the value of `f`.
    #[inline]
    fn update_style_mask<F, M>(&self, f: F)
    where
        F: FnOnce(NSUInteger) -> M,
        M: Into<NSUInteger>,
    {
        self.set_style_mask(f(self.style_mask()));
    }
}

impl WindowExt for Window {
    #[inline]
    unsafe fn from_ns_window(ns_window: StrongPtr) -> Self {
        SysWindow { ns_window }.into()
    }

    #[inline]
    fn ns_window(&self) -> id {
        *self.sys.ns_window
    }
}

/// macOS-specific extensions for
/// [`WindowBuilder`](../../struct.WindowBuilder.html).
pub trait WindowBuilderExt {
    /// Sets the `NSWindowStyleMask` used to construct the window.
    ///
    /// Note that other options may override bits set in `mask`.
    fn style_mask<M: Into<NSUInteger>>(&mut self, mask: M) -> &mut Self;

    /// Sets whether the titlebar for the `NSWindow` should be hidden.
    ///
    /// This makes the titlebar transparent and enables
    /// `NSFullSizeContentViewWindowMask`.
    fn titlebar_hidden(&mut self, hidden: bool) -> &mut Self;
}

impl<'a> WindowBuilderExt for WindowBuilder<'a> {
    #[inline]
    fn style_mask<M: Into<NSUInteger>>(&mut self, mask: M) -> &mut Self {
        self.sys.style_mask = Some(mask.into());
        self
    }

    #[inline]
    fn titlebar_hidden(&mut self, hidden: bool) -> &mut Self {
        self.sys.titlebar_hidden = hidden;
        self
    }
}

impl From<NSPoint> for PhysicalPosition {
    #[inline]
    fn from(NSPoint { x, y }: NSPoint) -> Self {
        PhysicalPosition { x, y }
    }
}

impl From<NSPoint> for LogicalPosition {
    #[inline]
    fn from(NSPoint { x, y }: NSPoint) -> Self {
        LogicalPosition { x, y }
    }
}

impl From<PhysicalPosition> for NSPoint {
    #[inline]
    fn from(PhysicalPosition { x, y }: PhysicalPosition) -> Self {
        NSPoint { x, y }
    }
}

impl From<LogicalPosition> for NSPoint {
    #[inline]
    fn from(LogicalPosition { x, y }: LogicalPosition) -> Self {
        NSPoint { x, y }
    }
}

impl From<NSSize> for PhysicalSize {
    #[inline]
    fn from(NSSize { width, height }: NSSize) -> Self {
        PhysicalSize { width, height }
    }
}

impl From<NSSize> for LogicalSize {
    #[inline]
    fn from(NSSize { width, height }: NSSize) -> Self {
        LogicalSize { width, height }
    }
}

impl From<PhysicalSize> for NSSize {
    #[inline]
    fn from(PhysicalSize { width, height }: PhysicalSize) -> Self {
        NSSize { width, height }
    }
}

impl From<LogicalSize> for NSSize {
    #[inline]
    fn from(LogicalSize { width, height }: LogicalSize) -> Self {
        NSSize { width, height }
    }
}
