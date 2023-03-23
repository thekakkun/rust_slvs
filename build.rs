extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let libdir_path = PathBuf::from("solvespace")
        .canonicalize()
        .expect("Cannot canonicalize path.");

    let headers_path = libdir_path.join("include/slvs.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string.");
    let lib_path = libdir_path.join("build").join("bin");
    // println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    // println!("cargo:rustc-link-lib=static=slvs");
    // println!("cargo:rustc-link-lib=static=mimalloc");

    // let stdlib_path = PathBuf::from("/usr/lib/gcc/x86_64-linux-gnu/11");
    // println!("cargo:rustc-link-search={}", stdlib_path.to_str().unwrap());
    // println!("cargo:rustc-link-lib=static=gomp");
    // println!("cargo:rustc-link-lib=static=stdc++");

    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=slvs");
    println!("cargo:rustc-link-lib=mimalloc");
    
    // println!("cargo:rustc-link-search={}", stdlib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=gomp");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rerun-if-changed={}", headers_path_str);

    let bindings = bindgen::Builder::default()
        .opaque_type("std::.*")
        .header(headers_path_str)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings.");
}
