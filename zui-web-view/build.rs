fn main() {
    let target_os = std::env::var_os("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("rustc-link-lib=framework=WebKit");
    }
}
