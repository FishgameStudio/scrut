/* Build scripts */

use chrono;

fn main() {
    let build_time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rustc-env=BUILD_TIME={}", build_time); // Build time
    println!("cargo:rustc-env=TARGET_ARCH={}", arch); // Arch
    println!("cargo:rustc-env=TARGET_OS={}", os); // OS
}
