
extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::{Language, Config};

fn main() {
    let package_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIEST_DIR");
    
    let target_dir = match env::var("CARGO_TARGET_DIR") {
        Ok(target) => PathBuf::from(target),
        Err(_) => PathBuf::from(&crate_dir).join("target"),
    }
    .join(env::var("TARGET").expect("TARGET"))
    .join(env::var("PROFILE").expect("PROFILE"));
    
    let output_file = target_dir
        .join(format!("{}.h", package_name))
        .display()
        .to_string();
    
    let config = Config {
        language: Language::C,
        no_includes: true,
        cpp_compat: true,
        // namespace: Some(String::from("ffi")),
        ..Default::default()
    };
    
    cbindgen::generate_with_config(&crate_dir, config)
        .expect("cbindgen: Failed to generate header files.")
        .write_to_file(&output_file);
}
