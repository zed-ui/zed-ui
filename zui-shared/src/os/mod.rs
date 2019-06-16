//! OS-specific functionality.

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        pub mod macos;
    } else {
        compile_error!("`zui-shared` does not compile for this platform");
    }
}
