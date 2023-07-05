use std::env;
use std::path::PathBuf;

extern crate bindgen;
use bindgen::callbacks::ParseCallbacks;
use dunce::canonicalize;

fn main() {
    let libdir_path = canonicalize(PathBuf::from("solvespace")).expect("Cannot canonicalize path.");
    let target = env::var("TARGET").unwrap();

    // Build solvespace library
    let mut slvs_cfg = cc::Build::new();

    // Things necessary for Windows but not Linux, dunno about building on Mac OS.
    if target.contains("windows") {
        println!(
            "cargo:rustc-link-search={}",
            PathBuf::from(r"C:\Windows\System32").to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=shell32");

        slvs_cfg.define("_CRT_SECURE_NO_DEPRECATE", None);
        slvs_cfg.define("_CRT_SECURE_NO_WARNINGS", None);
        slvs_cfg.define("_SCL_SECURE_NO_WARNINGS", None);
        slvs_cfg.define("WINVER", "0x0501");
        slvs_cfg.define("_WIN32_WINNT", "0x0501");
        slvs_cfg.define("_WIN32_IE", "_WIN32_WINNT");
        slvs_cfg.define("ISOLATION_AWARE_ENABLED", None);
        slvs_cfg.define("WIN32", None);
        slvs_cfg.define("WIN32_LEAN_AND_MEAN", None);
        slvs_cfg.define("UNICODE", None);
        slvs_cfg.define("_UNICODE", None);
        slvs_cfg.define("NOMINMAX", None);
        slvs_cfg.define("_USE_MATH_DEFINES", None);
    }

    slvs_cfg
        .cpp(true)
        .define("LIBRARY", None)
        .includes(
            [
                "src",
                "include",
                "extlib/eigen",
                "src/SYSTEM",
                "extlib/mimalloc/include",
            ]
            .map(|file| libdir_path.join(PathBuf::from(file))),
        )
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
            .map(|file| libdir_path.join(PathBuf::from(file))),
        )
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers")
        .compile("slvs");

    // Build mimalloc
    let mut mimalloc_cfg = cc::Build::new();

    mimalloc_cfg
        .include(libdir_path.join(PathBuf::from("extlib/mimalloc/include")))
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
            .map(|file| libdir_path.join(PathBuf::from(file))),
        )
        .compile("mimalloc");

    // Generate bindings to library header
    let bindings = bindgen::Builder::default()
        .opaque_type("std::.*")
        .allowlist_var("SLVS_.*")
        .allowlist_type("Slvs_.*")
        .allowlist_function("Slvs_.*")
        .header(
            libdir_path
                .join(PathBuf::from("include/slvs.h"))
                .to_str()
                .unwrap(),
        )
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-fvisibility=default")
        .parse_callbacks(Box::new(Callback))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings.");
}

#[derive(Debug)]
struct Callback;

impl ParseCallbacks for Callback {
    fn add_derives(&self, _info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        match _info.name {
            "Slvs_Param" | "Slvs_Entity" | "Slvs_Constraint" => {
                vec!["serde::Serialize".into(), "serde::Deserialize".into()]
            }
            _ => vec![],
        }
    }
}
