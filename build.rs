extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=tss2-rc");
    println!("cargo:rustc-link-lib=tss2-esys");
    println!("cargo:rustc-link-lib=tss2-mu");
    println!("cargo:rustc-link-lib=tss2-fapi");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tss2.rs"))
        .expect("Couldn't write bindings!");
}
