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

    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=slvs");
    println!("cargo:rustc-link-lib=static=mimalloc");
    println!(
        "cargo:rustc-link-search={}",
        PathBuf::from("/usr/lib/gcc/x86_64-linux-gnu/11")
            .to_str()
            .unwrap()
    );
    println!("cargo:rustc-link-lib=static=gomp");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rerun-if-changed={}", headers_path_str);

    // let obj_path = libdir_path.join("build/slvs.o");
    // let lib_path = libdir_path.join("build/libslvs.a");

    // if !std::process::Command::new("clang++")
    //     .arg("-c")
    //     .arg("-o")
    //     .arg(&obj_path)
    //     .arg(libdir_path.join("src/lib.cpp"))
    //     .output()
    //     .expect("Could not spawn `clang`.")
    //     .status
    //     .success()
    // {
    //     panic!("Could not compile object file.");
    // }

    // if !std::process::Command::new("ar")
    //     .arg("rcs")
    //     .arg(lib_path)
    //     .arg(obj_path)
    //     .output()
    //     .expect("Could not spawn `ar`.")
    //     .status
    //     .success()
    // {
    //     panic!("Could not emit library file.");
    // }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .opaque_type("std::.*")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("--target=wasm32-unknown-unknown")
        .clang_arg("-I/usr/include/")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings.");
}
