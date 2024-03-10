extern crate lazy_static;

use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

mod build_bindgen;
use crate::build_bindgen::bindgen;

fn update_submodules() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .stdout(Stdio::inherit()) // Optionally output stdout/stderr to help with debugging
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Submodule update failed",
        )));
    }

    Ok(())
}

fn main() {
    let projectm_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libprojectM");

    // Ensure the submodule is updated and initialized
    if !projectm_path.exists() {
        println!("cargo:warning=The libprojectM submodule is not checked out. Please run 'git submodule update --init --recursive' and try building again.");
        std::process::exit(1);
    }

    // Attempt to update and initialize submodules recursively
    if let Err(e) = update_submodules() {
        println!("cargo:warning=Failed to update submodules: {}", e);
        std::process::exit(1);
    }

    // Feature: enable-playlist
    fn enable_playlist() -> String {
        if cfg!(feature = "playlist") {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    #[cfg(target_os = "windows")]
    let dst = cmake::Config::new(&*projectm_path)
        .generator("Visual Studio 17 2022")
        .define(
            "CMAKE_TOOLCHAIN_FILE",
            format!(
                "{}/scripts/buildsystems/vcpkg.cmake",
                env::var("VCPKG_INSTALLATION_ROOT").unwrap()
            ),
        )
        .define("VCPKG_TARGET_TRIPLET", "x64-windows-static-md")
        .define(
            "CMAKE_MSVC_RUNTIME_LIBRARY",
            "MultiThreaded$<$<CONFIG:Debug>:Debug>DLL",
        )
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .build();

    #[cfg(target_os = "linux")]
    let dst = cmake::Config::new(&*projectm_path)
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .build();

    #[cfg(target_os = "macos")]
    let dst = cmake::Config::new(&*projectm_path)
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .build();

    #[cfg(target_os = "emscripten")]
    let dst = cmake::Config::new(&*projectm_path)
        .define("ENABLE_PLAYLIST", enable_playlist().as_str())
        .define("ENABLE_EMSCRIPTEN", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    #[cfg(target_os = "windows")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=dylib=projectM-4");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlist");
    } else {
        println!("cargo:rustc-link-lib=dylib=projectM-4d");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlistd");
    }

    #[cfg(target_os = "linux")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=dylib=libprojectM=4");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlist");
    } else {
        println!("cargo:rustc-link-lib=dylib=projectM-4d");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlistd");
    }

    #[cfg(target_os = "macos")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=dylib=projectM-4");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlist");
    } else {
        println!("cargo:rustc-link-lib=dylib=projectM-4d");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlistd");
    }

    #[cfg(target_os = "emscripten")]
    if Ok("release".to_owned()) == env::var("PROFILE") {
        println!("cargo:rustc-link-lib=static=projectM-4");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlist");
    } else {
        println!("cargo:rustc-link-lib=static=projectM-4d");

        #[cfg(feature = "playlist")]
        println!("cargo:rustc-link-lib=dylib=projectM-4-playlistd");
    }

    bindgen()
}
