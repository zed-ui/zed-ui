//! ZedUI is a cross-platform framework for creating graphical user interfaces.

#![deny(missing_docs)]

pub extern crate zui_window as window;

pub mod os;

#[doc(inline)]
pub use window::{Window, dpi};
