use std::env;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // We just forward the DEP_IMGUI_THIRD_PARTY variable here because the
    // main function outside the build script does not actually see it
    let cimgui_include_path =
        env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined");
    println!(
        "cargo:rustc-env=DEP_IMGUI_THIRD_PARTY={}",
        cimgui_include_path
            .to_str()
            .expect("Could not turn cimgui include path to string")
    );
}
