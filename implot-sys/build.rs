#![allow(dead_code)]

// This is taken pretty vanilla from
// https://github.com/Gekkio/imgui-rs/blob/master/imgui-sys/build.rs
// for now, but expected to diverge from that over time.
use std::{env, fs, io, path::Path};

use bindgen;

const CPP_FILES: [&str; 2] = [
    "third-party/cimplot/cimplot.cpp",
    "third-party/cimplot/implot/implot.cpp",
];

fn assert_file_exists(path: &str) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            panic!(
                "Can't access {}. Did you forget to fetch git submodules?",
                path
            );
        }
        Err(e) => Err(e),
    }
}

fn main() -> io::Result<()> {
    // --- Compile cimgui
    let mut build = cc::Build::new();
    build.cpp(true);

    // Take over imgui preprocessor defines from the imgui-sys crate.
    // Taken from https://github.com/aloucks/imguizmo-rs/blob/master/imguizmo-sys/build.rs
    for (key, val) in env::vars().filter(|(key, _)| key.starts_with("DEP_IMGUI_DEFINE_")) {
        let key = key.trim_start_matches("DEP_IMGUI_DEFINE_");
        let val = if !val.is_empty() {
            Some(val.as_str())
        } else {
            None
        };
        build.define(key, val);
    }

    let cimgui_include_path =
        env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined");
    let imgui_include_path = Path::new(&cimgui_include_path).join("imgui");
    build.include(&cimgui_include_path);
    build.include(&imgui_include_path);

    // Taken from the imgui-sys build as well
    build.flag_if_supported("-Wno-return-type-c-linkage");
    for path in &CPP_FILES {
        assert_file_exists(path)?;
        build.file(path);
    }

    build.compile("cimplot");

    // --- Create bindgen bindings
    // TODO(4bb4) move this out to separate shell script (see #1) so users don't have
    // to have clang installed to build this crate.
    let bindings = bindgen::Builder::default()
        .header(&(cimgui_include_path.into_string().unwrap() + "/cimgui.h"))
        .header("third-party/cimplot/cimplot.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=1")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
