extern crate bindgen;

use pkg_config::Config;
use std::env;
use std::path::PathBuf;

fn link(lib: &str) {
  println!("cargo:rustc-link-lib={}", lib);
}
fn link_path(path: &str){
  println!("cargo:rustc-link-search={}", path);
}

fn main() {
  let cfg = Config::new().probe("javascriptcoregtk-4.0").unwrap();
  let include_paths = cfg
    .include_paths
    .into_iter()
    .map(|x: PathBuf| format!("-I{}", x.to_str().unwrap()));
  for path in cfg.link_paths {
    link_path(path.to_str().unwrap())
  }
  for lib in cfg.libs.iter() {
    link(lib);
  }
  println!("cargo:rerun-if-changed=wrapper.h");

  let bindings = bindgen::builder()
    .clang_args(include_paths)
    /*.clang_arg("-I/usr/include/webkitgtk-4.0")
    .clang_arg("-I/usr/include/glib-2.0")
    .clang_arg("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include")*/
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings");
}
