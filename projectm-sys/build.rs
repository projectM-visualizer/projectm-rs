use std::env;
use std::path::PathBuf;

mod build_bindgen;
use crate::build_bindgen::bindgen;

fn main() {
    // Get the path to the projectM source code
    let projectm_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libprojectM");

    // Check if the libprojectM source code exists
    if !projectm_path.exists() {
        println!("cargo:warning=The libprojectM source code is missing.");
        println!(
            "cargo:warning=If you are building from a git clone, please run 'git submodule update --init --recursive'."
        );
        println!("cargo:warning=If you downloaded this crate from crates.io, please ensure that the crate was packaged correctly.");
        std::process::exit(1);
    }

    // Determine if the 'playlist' feature is enabled
    let enable_playlist = if cfg!(feature = "playlist") { "ON" } else { "OFF" };

    let dst;

    // Platform-specific configurations
    if cfg!(target_os = "windows") {
        // Ensure VCPKG installation root is set
        let vcpkg_root = match env::var("VCPKG_INSTALLATION_ROOT") {
            Ok(val) => val,
            Err(_) => {
                println!("cargo:warning=VCPKG_INSTALLATION_ROOT is not set. Please set it to your VCPKG installation directory.");
                std::process::exit(1);
            }
        };

        let vcpkg_root = PathBuf::from(vcpkg_root);
        let vcpkg_toolchain = vcpkg_root
            .join("scripts")
            .join("buildsystems")
            .join("vcpkg.cmake");

        if !vcpkg_toolchain.exists() {
            println!(
                "cargo:warning=The vcpkg toolchain file was not found at: {}",
                vcpkg_toolchain.display()
            );
            std::process::exit(1);
        }

        // Set VCPKG_ROOT for CMake
        env::set_var("VCPKG_ROOT", &vcpkg_root);

        // Optionally set CMAKE_PREFIX_PATH
        let vcpkg_installed = vcpkg_root.join("installed").join("x64-windows-static-md");

        // Configure and build libprojectM using CMake for Windows
        dst = cmake::Config::new(&projectm_path)
            .generator("Visual Studio 17 2022")
            .define(
                "CMAKE_TOOLCHAIN_FILE",
                &vcpkg_toolchain,
            )
            .define("VCPKG_TARGET_TRIPLET", "x64-windows-static-md")
            .define(
                "CMAKE_MSVC_RUNTIME_LIBRARY",
                "MultiThreaded$<$<CONFIG:Debug>:Debug>DLL",
            )
            .define("ENABLE_PLAYLIST", enable_playlist)
            .define(
                "projectM_Eval_DIR",
                &projectm_path.join("vendor").join("projectm-eval"),
            )
            .define("CMAKE_PREFIX_PATH", &vcpkg_installed)
            .define("CMAKE_VERBOSE_MAKEFILE", "ON")
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .build();
    } else if cfg!(target_os = "emscripten") {
        // Configure and build libprojectM using CMake for Emscripten
        dst = cmake::Config::new(&projectm_path)
            .define("ENABLE_PLAYLIST", enable_playlist)
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .define("ENABLE_EMSCRIPTEN", "ON")
            .build();
    } else {
        // Configure and build libprojectM using CMake for other platforms
        dst = cmake::Config::new(&projectm_path)
            .define("ENABLE_PLAYLIST", enable_playlist)
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .build();
    }

    // Specify the library search path
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Determine the library name based on the build profile
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Platform-specific library linking
    if cfg!(target_os = "windows") || cfg!(target_os = "emscripten") {
        // Use static linking for Windows and Emscripten
        if profile == "release" {
            println!("cargo:rustc-link-lib=static=projectM-4");
            if cfg!(feature = "playlist") {
                println!("cargo:rustc-link-lib=static=projectM-4-playlist");
            }
        } else {
            println!("cargo:rustc-link-lib=static=projectM-4d");
            if cfg!(feature = "playlist") {
                println!("cargo:rustc-link-lib=static=projectM-4-playlistd");
            }
        }
    } else {
        // Use dynamic linking for other platforms (Linux, macOS)
        if profile == "release" {
            println!("cargo:rustc-link-lib=dylib=projectM-4");
            if cfg!(feature = "playlist") {
                println!("cargo:rustc-link-lib=dylib=projectM-4-playlist");
            }
        } else {
            println!("cargo:rustc-link-lib=dylib=projectM-4d");
            if cfg!(feature = "playlist") {
                println!("cargo:rustc-link-lib=dylib=projectM-4-playlistd");
            }
        }
    }

    // Run bindgen to generate Rust bindings
    bindgen();
}