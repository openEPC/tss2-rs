extern crate bindgen;

use std::env;
use std::path::PathBuf;

macro_rules! add_header {
    ($builder:ident, $header:literal) => {
        $builder = $builder.header($header);
    }
}

fn main() {
    println!("cargo:rustc-link-lib=tss2-rc");
    println!("cargo:rustc-link-lib=tss2-mu");
    #[cfg(feature = "sys")]
    println!("cargo:rustc-link-lib=tss2-sys");
    #[cfg(feature = "esys")]
    println!("cargo:rustc-link-lib=tss2-esys");
    #[cfg(feature = "fapi")]
    println!("cargo:rustc-link-lib=tss2-fapi");

    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h");
    #[cfg(feature = "sys")]
    add_header!(builder, "sys.h");
    #[cfg(feature = "esys")]
    add_header!(builder, "esys.h");
    #[cfg(feature = "fapi")]
    add_header!(builder, "fapi.h");
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tss2.rs"))
        .expect("Couldn't write bindings!");
}
