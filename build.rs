fn main() {
    // Tell Rust that the 'kani' cfg is expected (used for formal verification)
    println!("cargo::rustc-check-cfg=cfg(kani)");

    #[cfg(feature = "gui")]
    tauri_build::build();
}
