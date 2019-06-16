cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::*;
    } else {
        compile_error!("`zui-window` does not compile for this platform");
    }
}
