extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;




fn compile_tpm2_tss() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    Command::new("./bootstrap")
        .current_dir("./tpm2-tss")
        .output()
        .expect("Failed to run bootstrap");
    Command::new("./configure")
        .current_dir("./tpm2-tss")
        .arg(format!("--exec-prefix={}/tpm", root_dir))
        .arg(format!("--prefix={}/tpm", root_dir))
        .arg("--enable-static")
        .output()
        .expect("Failed to run configure");
    Command::new("make")
        .current_dir("./tpm2-tss")
        .arg("-j$(nproc)")
        .output()
        .expect("Failed to run make");
    Command::new("make")
        .current_dir("./tpm2-tss")
        .arg("install")
        .output()
        .expect("Failed to run make install");
}

fn link_extra_static() {
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=json-c");
}

fn link_main_static() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-lib=static=tss2-tcti-device");
    println!("cargo:rustc-link-lib=static=tss2-rc");
    println!("cargo:rustc-link-lib=static=tss2-mu");
    #[cfg(feature = "sys")]
    println!("cargo:rustc-link-lib=static=tss2-sys");
    #[cfg(feature = "esys")]
    println!("cargo:rustc-link-lib=static=tss2-esys");
    #[cfg(feature = "fapi")]
    println!("cargo:rustc-link-lib=static=tss2-fapi");

    println!(
        "cargo:rustc-link-search=native={}",
        format!("{}/tpm/lib", root_dir)
    );
}

fn link_main_dynamic() {
    println!("cargo:rustc-link-lib=tss2-rc");
    println!("cargo:rustc-link-lib=tss2-mu");
    #[cfg(feature = "sys")]
    println!("cargo:rustc-link-lib=tss2-sys");
    #[cfg(feature = "esys")]
    println!("cargo:rustc-link-lib=tss2-esys");
    #[cfg(feature = "fapi")]
    println!("cargo:rustc-link-lib=tss2-fapi");
}

fn add_headers(builder: bindgen::Builder) -> bindgen::Builder {
    let mut mut_builder = builder.header("wrapper.h");

    let mut lib_path = "";

    if cfg!(feature = "static") {
        lib_path = "tpm/include/tss2/tss2_";
    }

    mut_builder = mut_builder.header(format!("{}{}", lib_path, "rc.h"));
    mut_builder = mut_builder.header(format!("{}{}", lib_path, "mu.h"));

    if cfg!(feature = "sys") {
        mut_builder = mut_builder.header(format!("{}{}", lib_path, "sys.h"));
    }
    if cfg!(feature = "esys") {
        mut_builder = mut_builder.header(format!("{}{}", lib_path, "esys.h"));
    }
    if cfg!(feature = "fapi") {
        mut_builder = mut_builder.header(format!("{}{}", lib_path, "fapi.h"));
    }

    return mut_builder;
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    if cfg!(feature = "static") {
        println!("cargo:rerun-if-changed=tpm2-tss");
        compile_tpm2_tss();
        link_main_static();
        link_extra_static();
    } else {
        link_main_dynamic();
    }


    let mut builder = bindgen::Builder::default();
    builder = add_headers(builder);

    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tss2.rs"))
        .expect("Couldn't write bindings!");
}
