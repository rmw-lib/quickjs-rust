extern crate bindgen;

use std::{env, path::PathBuf};

fn main() {
  let bindings = bindgen::Builder::default()
    .header("c.h")
    .generate()
    .expect("❌ rust-bindgen c.h");

  let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("src/c.rs"))
    .expect("❌ rust-bindgen c.rs");
}
