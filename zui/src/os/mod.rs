//! OS-specific functionality.

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        /// macOS-specific extensions.
        pub mod macos {
            #[doc(inline)]
            pub use window::os::macos::*;
        }
    } else {
        compile_error!("`zui` does not compile for this platform");
    }
}
