use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // 1. Get Folly installation path from environment variable
    let folly_scratch_path = env::var("FOLLY_GETDEPS_SCRATCH_PATH")
        .expect("FOLLY_GETDEPS_SCRATCH_PATH environment variable not set. Run this via the build_rust_wrapper.sh script or set it manually.");
    let base_install_path = PathBuf::from(folly_scratch_path).join("installed");

    if !base_install_path.exists() {
        panic!("Base install path does not exist: {:?}", base_install_path);
    }

    // Dynamically find folly-* and boost-* directories
    let mut folly_install_path = None;
    let mut boost_install_path = None;

    for entry in fs::read_dir(&base_install_path).expect("Failed to read base install directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            if dir_name.starts_with("folly-") {
                folly_install_path = Some(path.clone());
            } else if dir_name.starts_with("boost-") {
                boost_install_path = Some(path.clone());
            }
        }
    }

    let folly_install_path = folly_install_path.expect("Could not find folly-* directory in base install path");
    let boost_install_path = boost_install_path.expect("Could not find boost-* directory in base install path");

    // Construct include and lib paths using the found directories
    let folly_include_path = folly_install_path.join("include");
    let folly_lib_path = folly_install_path.join("lib");
    let boost_include_path = boost_install_path.join("include");

    if !folly_include_path.exists() {
        panic!("Folly include path does not exist: {:?}", folly_include_path);
    }
    if !boost_include_path.exists() {
        panic!("Boost include path does not exist: {:?}. Ensure Boost was built by getdeps.py in the same scratch path.", boost_include_path);
    }
    if !folly_lib_path.exists() {
        panic!("Folly lib path does not exist: {:?}", folly_lib_path);
    }

    // 2. Compile the C++ wrapper code using cxx-build
    cxx_build::bridge("src/lib.rs") // Point to the file with the #[cxx::bridge] module
        .file("src/wrapper.cpp")
        .flag_if_supported("-std=c++17") // Folly requires C++17
        .include(&folly_include_path)     // Include Folly headers
        .include(&boost_include_path)     // Include Boost headers
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
