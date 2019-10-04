extern crate cbindgen;

use std::env;
use cbindgen::{Config, Language};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate_with_config(&crate_dir, Config {
        language: Language::C,
        ..Default::default()
    })
        .unwrap()
        .write_to_file("rust_bindings.h");
}
