#![allow(dead_code)]

// This is taken pretty vanilla from
// https://github.com/Gekkio/imgui-rs/blob/master/imgui-sys/build.rs
// for now, but expected to diverge from that over time.
use std::{env, fs, io, path::PathBuf};

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
    // Disabled due to linking issues
    build
        .define("CIMGUI_NO_EXPORT", None)
        .define("IMGUI_DISABLE_WIN32_FUNCTIONS", None)
        .define("IMGUI_DISABLE_OSX_FUNCTIONS", None);

    let imgui_third_party =
        env::var_os("DEP_IMGUI_THIRD_PARTY").expect("No envvar found for third_party");
    println!("third party is {:?}", imgui_third_party);
    build.include(imgui_third_party.clone().into_string().unwrap() + "/imgui/");

    build.flag_if_supported("-Wno-return-type-c-linkage");
    for path in &CPP_FILES {
        assert_file_exists(path)?;
        build.file(path);
    }
    build.compile("libcimplot.a");

    // --- Create bindgen bindings
    // The actual generate() errors out right now with parsing errors,
    // will probably need to whiltelist things, fix preprocessor definitions,
    // bindgen settings or some combination thereof.
    let _bindings = bindgen::Builder::default()
        .header(imgui_third_party.into_string().unwrap() + "/cimgui.h")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    //.generate()
    //.expect("Unable to generate bindings");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    //bindings
    //.write_to_file(out_path.join("bindings.rs"))
    //.expect("Couldn't write bindings!");

    Ok(())
}
