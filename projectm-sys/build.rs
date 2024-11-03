use std::env;
use std::path::PathBuf;

mod build_bindgen;
use crate::build_bindgen::bindgen;

// Functions to determine feature flags
fn enable_playlist() -> &'static str {
    if cfg!(feature = "playlist") {
        "ON"
    } else {
        "OFF"
    }
}

fn enable_static_link() -> &'static str {
    if cfg!(feature = "static") {
        "ON"
    } else {
        "OFF"
    }
}

fn main() {
    // Path to the projectM source code
    let projectm_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libprojectM");

    // Verify the existence of the libprojectM directory
    if !projectm_path.exists() {
        println!("cargo:warning=The libprojectM source code is missing.");
        println!(
            "cargo:warning=If you are building from a git clone, please run 'git submodule update --init --recursive'."
        );
        println!("cargo:warning=If you downloaded this crate from crates.io, please ensure that the crate was packaged correctly.");
        std::process::exit(1);
    }

    // Determine feature flags
    let enable_playlist_flag = enable_playlist();
    let enable_static_flag = enable_static_link();

    let dst;

    // Platform-specific CMake configurations
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

        // Define the installation path for vcpkg
        let vcpkg_installed = vcpkg_root.join("installed").join("x64-windows-static-md");

        // Configure and build libprojectM using CMake for Windows
        let mut cmake_config = cmake::Config::new(&projectm_path)
            .generator("Visual Studio 17 2022")
            .define("CMAKE_TOOLCHAIN_FILE", &vcpkg_toolchain)
            .define("VCPKG_TARGET_TRIPLET", "x64-windows-static-md")
            .define(
                "CMAKE_MSVC_RUNTIME_LIBRARY",
                "MultiThreaded$<$<CONFIG:Debug>:Debug>DLL",
            )
            .define("ENABLE_PLAYLIST", enable_playlist_flag)
            .define(
                "projectM_Eval_DIR",
                &projectm_path.join("vendor").join("projectm-eval"),
            )
            .define("CMAKE_PREFIX_PATH", &vcpkg_installed)
            .define("CMAKE_VERBOSE_MAKEFILE", "ON")
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .define("BUILD_SHARED_LIBS", enable_static_flag); // Integrated static linking option

        dst = cmake_config.build();
    } else if cfg!(target_os = "emscripten") {
        // Configure and build libprojectM using CMake for Emscripten
        dst = cmake::Config::new(&projectm_path)
            .define("ENABLE_PLAYLIST", enable_playlist_flag)
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .define("ENABLE_EMSCRIPTEN", "ON")
            .define("BUILD_SHARED_LIBS", enable_static_flag) // Integrated static linking option
            .build();
    } else {
        // Configure and build libprojectM using CMake for other platforms (Linux, macOS)
        dst = cmake::Config::new(&projectm_path)
            .define("ENABLE_PLAYLIST", enable_playlist_flag)
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .define("BUILD_SHARED_LIBS", enable_static_flag) // Integrated static linking option
            .build();
    }

    // Specify the library search path
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Determine the build profile (release or debug)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Platform and feature-specific library linking
    if cfg!(target_os = "windows") || cfg!(target_os = "emscripten") {
        // Static linking for Windows and Emscripten if the 'static' feature is enabled
        if cfg!(feature = "static") {
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
            // Dynamic linking if 'static' feature is not enabled
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
    } else {
        // For other platforms (Linux, macOS)
        if cfg!(feature = "static") {
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
    }

    // Generate Rust bindings using bindgen
    bindgen();
}