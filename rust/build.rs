//extern crate cbindgen;

//use std::env;

fn main() {
  /*
   expand 会死循环

  let package_name = env::var("CARGO_PKG_NAME").unwrap();
  let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  cbindgen::Builder::new()
    .with_config(cbindgen::Config {
      include_guard: Some("cbindgen_rust_h".to_string()),
      language: cbindgen::Language::C,
      ..cbindgen::Config::default()
    })
    .with_include("../quickjs/quickjs.h")
    .with_parse_expand(&[(package_name).as_str()])
    //.with_parse_expand(&["rust_macro"])
    .with_pragma_once(true)
    .with_crate(crate_dir)
    .generate()
    .expect("❌ cbindgen")
    .write_to_file("rust.h");
  */
}
