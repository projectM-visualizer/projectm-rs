use std::{env, path::PathBuf};

pub fn bindgen() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("bindings.rs");

    fn get_header() -> String {
        if cfg!(feature = "playlist") {
            "bindgen/playlist.h".to_string()
        } else {
            "bindgen/default.h".to_string()
        }
    }

    let bindings = bindgen::Builder::default()
        .header(get_header())
        .allowlist_function("projectm_.*")
        .clang_arg(format!("-I{}/include", out_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
