extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/binding/btf.h");
    println!("cargo:rerun-if-changed=src/binding/bpf.h");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .ctypes_prefix("cty")
        .use_core()
        .header("src/binding/btf.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("btf.rs"))
        .expect("Couldn't write bindings!");
    bindgen::Builder::default()
        .ctypes_prefix("cty")
        .use_core()
        .header("src/binding/bpf.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bpf.rs"))
        .expect("Couldn't write bindings!");
}
