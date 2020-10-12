#![allow(dead_code)]

// This is taken pretty vanilla from
// https://github.com/Gekkio/imgui-rs/blob/master/imgui-sys/build.rs
// for now, but expected to diverge from that over time.
use std::{env, fs, io, path::Path};

const CPP_FILES: &[&str] = &[
    "third-party/cimplot/cimplot.cpp",
    "third-party/cimplot/implot/implot.cpp",
    "third-party/cimplot/implot/implot_items.cpp",
    "third-party/cimplot/implot/implot_demo.cpp", // Could remove this if demo not used
];

const IMPLOT_INCLUDE_DIRECTORIES: &[&str] = &["third-party/cimplot/implot/"];

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
    for path in IMPLOT_INCLUDE_DIRECTORIES {
        build.include(path);
    }

    // Taken from the imgui-sys build as well
    build.flag_if_supported("-Wno-return-type-c-linkage");
    build.flag_if_supported("-Wno-unused-parameter");
    build.flag_if_supported("-std=c++11");
    for path in CPP_FILES {
        assert_file_exists(path)?;
        build.file(path);
    }
    build.compile("cimplot");
    Ok(())
}
