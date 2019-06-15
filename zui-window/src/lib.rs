//! ZedUI windowing utilities.

#![deny(missing_docs)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use std::marker::PhantomData;

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

impl Window {
    /// Creates a builder suitable for constructing a new `Window` instance.
    #[inline]
    pub fn builder<'a>() -> WindowBuilder<'a> {
        Default::default()
    }

    /// Sets the displayed title.
    #[inline]
    pub fn set_title(&self, title: &str) {
        self.sys.set_title(title);
    }
}

/// A type for configuring how a `Window` instance should be constructed.
pub struct WindowBuilder<'a> {
    sys: sys::WindowBuilder,
    title: Option<&'a str>,
}

impl Default for WindowBuilder<'_> {
    #[inline]
    fn default() -> Self {
        WindowBuilder {
            sys: Default::default(),
            title: None,
        }
    }
}

impl<'a> WindowBuilder<'a> {
    /// Creates a builder suitable for constructing a new `Window` instance.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the window title.
    #[inline]
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Creates a new instance, returning an error upon failure.
    pub fn build(&self) -> Result<Window, ()> {
        Ok(self.sys_build()?.into())
    }
}
