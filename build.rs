use std::env;
use std::path::PathBuf;

extern crate bindgen;
use bindgen::CargoCallbacks;
use cmake::Config;

fn main() {
    let libdir_path = PathBuf::from("solvespace")
        .canonicalize()
        .expect("Cannot canonicalize path.");

    let dst = Config::new("solvespace")
        .profile("Release")
        .build_target("slvs")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build/bin").display()
    );
    println!("cargo:rustc-link-lib=slvs");
    println!("cargo:rustc-link-lib=mimalloc");

    // cc::Build::new()
    //     .cpp(true)
    //     .flag("-DLIBRARY")
    //     .include(libdir_path.join("src"))
    //     .include(libdir_path.join("include"))
    //     .include(dst.join("build/src"))
    //     .include(libdir_path.join("extlib/eigen"))
    //     .include(libdir_path.join("extlib/mimalloc/include"))
    //     .flag("-Wno-unused-parameter")
    //     .flag("-Wno-missing-field-initializers")
    //     .files(
    //         [
    //             "src/util.cpp",
    //             "src/entity.cpp",
    //             "src/expr.cpp",
    //             "src/constraint.cpp",
    //             "src/constrainteq.cpp",
    //             "src/system.cpp",
    //             "src/platform/platform.cpp",
    //             "src/lib.cpp",
    //         ]
    //         .map(|file| libdir_path.join(file)),
    //     )
    //     .compile("slvs");

    // cc::Build::new()
    //     .include(libdir_path.join("extlib/mimalloc/include"))
    //     .files(
    //         [
    //             "extlib/mimalloc/src/stats.c",
    //             "extlib/mimalloc/src/random.c",
    //             "extlib/mimalloc/src/os.c",
    //             "extlib/mimalloc/src/bitmap.c",
    //             "extlib/mimalloc/src/arena.c",
    //             "extlib/mimalloc/src/segment-cache.c",
    //             "extlib/mimalloc/src/segment.c",
    //             "extlib/mimalloc/src/page.c",
    //             "extlib/mimalloc/src/alloc.c",
    //             "extlib/mimalloc/src/alloc-aligned.c",
    //             "extlib/mimalloc/src/alloc-posix.c",
    //             "extlib/mimalloc/src/heap.c",
    //             "extlib/mimalloc/src/options.c",
    //             "extlib/mimalloc/src/init.c",
    //         ]
    //         .map(|file| libdir_path.join(file)),
    //     )
    //     .compile("mimalloc");

    let bindings = bindgen::Builder::default()
        .opaque_type("std::.*")
        .header(libdir_path.join("include/slvs.h").to_str().unwrap())
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
