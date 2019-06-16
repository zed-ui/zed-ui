//! ZedUI windowing utilities.

#![deny(missing_docs)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

extern crate zui_shared as shared;

use std::{
    fmt,
    marker::PhantomData
};
use shared::ZedString;

mod sys;
pub mod dpi;
pub mod os;

/// A handle to a window instance.
///
/// `Window` uses interior mutability and implements the [`Clone`] trait with
/// reference counting semantics.
///
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
#[derive(Clone)]
pub struct Window {
    sys: sys::Window,
    // !Send + !Sync
    _marker: PhantomData<*mut ()>,
}

#[doc(hidden)]
impl From<sys::Window> for Window {
    #[inline]
    fn from(sys: sys::Window) -> Self {
        Window { sys, _marker: PhantomData }
    }
}

impl fmt::Debug for Window {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.sys.fmt(f)
    }
}

impl Window {
    /// Creates a builder suitable for constructing a new `Window` instance.
    #[inline]
    pub fn builder() -> WindowBuilder {
        Default::default()
    }

    /// Sets the displayed title.
    #[inline]
    pub fn set_title<S: Into<ZedString>>(&self, title: S) {
        self.sys.set_title(title.into());
    }
}

/// A type for configuring how a `Window` instance should be constructed.
pub struct WindowBuilder {
    sys: sys::WindowBuilder,
    title: Option<ZedString>,
}

impl Default for WindowBuilder {
    #[inline]
    fn default() -> Self {
        WindowBuilder {
            sys: Default::default(),
            title: None,
        }
    }
}

impl WindowBuilder {
    /// Creates a builder suitable for constructing a new `Window` instance.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the window title.
    #[inline]
    pub fn title<S: Into<ZedString>>(&mut self, title: S) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    /// Creates a new instance, returning an error upon failure.
    pub fn build(&self) -> Result<Window, ()> {
        Ok(self.sys_build()?.into())
    }
}
