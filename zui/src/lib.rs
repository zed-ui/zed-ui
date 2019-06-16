//! ZedUI is a cross-platform framework for creating graphical user interfaces.

#![deny(missing_docs)]

pub extern crate zui_window as window;
pub extern crate zui_web_view as web_view;

pub mod os;

#[doc(inline)]
pub use self::{
    web_view::WebView,
    window::{Window, dpi},
};
