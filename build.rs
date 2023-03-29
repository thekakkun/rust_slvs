use std::env;
use std::path::PathBuf;

extern crate bindgen;
use bindgen::CargoCallbacks;
use dunce::canonicalize;

fn main() {
    let libdir_path = canonicalize(PathBuf::from("solvespace")).expect("Cannot canonicalize path.");

    println!(
        "cargo:rustc-link-search={}",
        PathBuf::from(r"C:\Windows\System32").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=shell32");

    cc::Build::new()
        .cpp(true)
        .include(libdir_path.join("src"))
        .include(libdir_path.join("include"))
        .include(libdir_path.join("extlib/eigen"))
        .include(libdir_path.join("src/SYSTEM"))
        .include(libdir_path.join("extlib/mimalloc/include"))
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers")
        .files(
            [
                "src/util.cpp",
                "src/entity.cpp",
                "src/expr.cpp",
                "src/constraint.cpp",
                "src/constrainteq.cpp",
                "src/system.cpp",
                "src/platform/platform.cpp",
                "src/lib.cpp",
            ]
            .map(|file| libdir_path.join(file)),
        )
        .define("CAIRO_WIN32_STATIC_BUILD", None)
        .define("ISOLATION_AWARE_ENABLED", None)
        .define("LIBRARY", None)
        .define("NOMINMAX", None)
        .define("UNICODE", None)
        .define("WIN32", None)
        .define("WIN32_LEAN_AND_MEAN", None)
        .define("WINVER", "0x0501")
        .define("_CRT_SECURE_NO_DEPRECATE", None)
        .define("_CRT_SECURE_NO_WARNINGS", None)
        .define("_SCL_SECURE_NO_WARNINGS", None)
        .define("_UNICODE", None)
        .define("_USE_MATH_DEFINES", None)
        .define("_WIN32_IE", "_WIN32_WINNT")
        .define("_WIN32_WINNT", "0x0501")
        .define("slvs_EXPORTS", None)
        .compile("slvs");

    cc::Build::new()
        .include(libdir_path.join("extlib/mimalloc/include"))
        .files(
            [
                "extlib/mimalloc/src/stats.c",
                "extlib/mimalloc/src/random.c",
                "extlib/mimalloc/src/os.c",
                "extlib/mimalloc/src/bitmap.c",
                "extlib/mimalloc/src/arena.c",
                "extlib/mimalloc/src/segment-cache.c",
                "extlib/mimalloc/src/segment.c",
                "extlib/mimalloc/src/page.c",
                "extlib/mimalloc/src/alloc.c",
                "extlib/mimalloc/src/alloc-aligned.c",
                "extlib/mimalloc/src/alloc-posix.c",
                "extlib/mimalloc/src/heap.c",
                "extlib/mimalloc/src/options.c",
                "extlib/mimalloc/src/init.c",
            ]
            .map(|file| libdir_path.join(file)),
        )
        .compile("mimalloc");

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
