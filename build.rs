use std::env;

fn main() {

  #[cfg(target_os = "windows")]
  let dst = cmake::build("projectm");
  
  #[cfg(target_os = "linux")]
  let dst = cmake::build("projectm");

  #[cfg(target_os = "ios")]
  let dst = cmake::build("projectm");

  #[cfg(target_os = "emscripten")]
  let dst = cmake::build("projectm");

  println!("cargo:rustc-link-search=native={}/lib", dst.display());
  // println!("cargo:rustc-link-arg=-sMIN_WEBGL_VERSION=2 -sMAX_WEBGL_VERSION=2");

  #[cfg(target_os = "windows")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }

  #[cfg(target_os = "linux")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }

  #[cfg(target_os = "ios")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }
  
  #[cfg(target_os = "emscripten")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=static=projectM");
  } else {
    println!("cargo:rustc-link-lib=static=projectMd");
  }
  
}



