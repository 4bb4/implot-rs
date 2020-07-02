#![allow(dead_code)]

// This is taken pretty vanilla from
// https://github.com/Gekkio/imgui-rs/blob/master/imgui-sys/build.rs
// for now, but expected to diverge from that over time.
use std::fs;
use std::io;

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
    let mut build = cc::Build::new();
    build.cpp(true);
    // Disabled due to linking issues
    build
        .define("CIMGUI_NO_EXPORT", None)
        .define("IMGUI_DISABLE_WIN32_FUNCTIONS", None)
        .define("IMGUI_DISABLE_OSX_FUNCTIONS", None);

    // This won't seem to work yet
    let imgui_third_party =
        std::env::var_os("DEP_IMGUI_THIRD_PARTY").expect("No env var found for third_party folder");
    println!("third party is {:?}", imgui_third_party);
    build.include(imgui_third_party);

    build.flag_if_supported("-Wno-return-type-c-linkage");
    for path in &CPP_FILES {
        assert_file_exists(path)?;
        build.file(path);
    }
    build.compile("libcimplot.a");
    Ok(())
}
