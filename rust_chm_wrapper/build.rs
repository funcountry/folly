use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Get Folly installation path from environment variable
    let folly_scratch_path = env::var("FOLLY_GETDEPS_SCRATCH_PATH")
        .expect("FOLLY_GETDEPS_SCRATCH_PATH environment variable not set. Run this via the build_rust_wrapper.sh script or set it manually.");
    let folly_install_path = PathBuf::from(folly_scratch_path).join("installed").join("folly");

    let folly_include_path = folly_install_path.join("include");
    let folly_lib_path = folly_install_path.join("lib");

    if !folly_include_path.exists() {
        panic!("Folly include path does not exist: {:?}", folly_include_path);
    }
    if !folly_lib_path.exists() {
        panic!("Folly lib path does not exist: {:?}", folly_lib_path);
    }

    // 2. Compile the C++ wrapper code using cxx-build
    cxx_build::bridge("src/lib.rs") // Point to the file with the #[cxx::bridge] module
        .file("src/wrapper.cpp")
        .flag_if_supported("-std=c++17") // Folly requires C++17
        .include(folly_include_path)     // Include Folly headers
        .include("include")              // Include our own wrapper header
        .compile("rust_chm_wrapper_cpp"); // Library name for the compiled C++ code

    // 3. Link against the pre-built Folly library and C++ standard library
    println!("cargo:rustc-link-search=native={}", folly_lib_path.display());
    println!("cargo:rustc-link-lib=static=folly"); // Link against libfolly.a
    // println!("cargo:rustc-link-lib=static=folly_base"); // folly links folly_base, so this might not be needed explicitly
    println!("cargo:rustc-link-lib=dylib=c++"); // Link against libc++ on macOS

    // Rerun build script if C++ files or bridge definition change
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=src/lib.rs"); // Update to lib.rs
}
