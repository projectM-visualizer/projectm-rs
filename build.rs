use std::env;

fn main() {

  let dst = cmake::build("projectm");
  println!("cargo:rustc-link-search=native={}/lib", dst.display());

  if Ok("release".to_owned()) == env::var("PROFILE") {
    println!("cargo:rustc-link-lib=dylib=projectM");
  } else {
    println!("cargo:rustc-link-lib=dylib=projectMd");
  }
  
}