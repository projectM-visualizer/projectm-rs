use std::env;
use std::path::PathBuf;

mod build_bindgen;
use crate::build_bindgen::bindgen;

fn main() {
    let projectm_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libprojectM");

    // Check if the libprojectM source code exists
    if !projectm_path.exists() {
        println!("cargo:warning=The libprojectM source code is missing.");
        println!("cargo:warning=If you are building from a git clone, please run 'git submodule update --init --recursive'.");
        println!("cargo:warning=If you downloaded this crate from crates.io, please ensure that the crate was packaged correctly.");
        std::process::exit(1);
    }

    // Determine if the 'playlist' feature is enabled
    let enable_playlist = if cfg!(feature = "playlist") {
        "ON"
    } else {
        "OFF"
    };

    // Configure and build libprojectM using CMake
    let mut cmake_config = cmake::Config::new(&projectm_path);
    cmake_config.define("ENABLE_PLAYLIST", enable_playlist);

    // Platform-specific configurations
    if cfg!(target_os = "windows") {
        // Ensure VCPKG is set up correctly
        let vcpkg_root = env::var("VCPKG_INSTALLATION_ROOT").expect("VCPKG_INSTALLATION_ROOT is not set");
        cmake_config
            .generator("Visual Studio 17 2022")
            .define(
                "CMAKE_TOOLCHAIN_FILE",
                format!("{}/scripts/buildsystems/vcpkg.cmake", vcpkg_root),
            )
            .define("VCPKG_TARGET_TRIPLET", "x64-windows-static-md")
            .define(
                "CMAKE_MSVC_RUNTIME_LIBRARY",
                "MultiThreaded$<$<CONFIG:Debug>:Debug>DLL",
            );
    } else if cfg!(target_os = "emscripten") {
        cmake_config.define("ENABLE_EMSCRIPTEN", "ON");
    }

    let dst = cmake_config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Determine the library name based on the build profile
    let profile = env::var("PROFILE").unwrap();
    let lib_suffix = if profile == "release" { "" } else { "d" };
    let lib_name = format!("projectM-4{}", lib_suffix);

    println!("cargo:rustc-link-lib=dylib={}", lib_name);

    // Handle the 'playlist' feature
    if cfg!(feature = "playlist") {
        let playlist_lib_name = format!("{}-playlist", lib_name);
        println!("cargo:rustc-link-lib=dylib={}", playlist_lib_name);
    }

    // Run bindgen to generate Rust bindings
    bindgen();
}