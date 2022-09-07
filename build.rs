use std::env;

fn main() {

  #[cfg(feature = "windows-x64")]
  let dst = cmake::build("projectm");
  
  #[cfg(feature = "linux-x64")]
  let dst = cmake::build("projectm");

  #[cfg(feature = "macos-x64")]
  let dst = cmake::build("projectm");

  #[cfg(feature = "emscripten-x32")]
  let dst = cmake::build("projectm");

  println!("cargo:rustc-link-search=native={}/lib", dst.display());

  #[cfg(feature = "windows-x64")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }

  #[cfg(feature = "linux-x64")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }

  #[cfg(feature = "macos-x64")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }
  
  #[cfg(feature = "emscripten-x32")]
  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=static=projectM");
  } else {
    println!("cargo:rustc-link-lib=static=projectMd");
  }
  
}



