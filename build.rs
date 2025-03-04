use std::{path::PathBuf, env};

fn main() {
    let libmfx = pkg_config::probe_library("mfx").unwrap();
    // https://github.com/Intel-Media-SDK/MediaSDK/blob/master/api/include/mfxvideo.h
    // https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let libmfx_include_path = libmfx.include_paths[0].display();
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{libmfx_include_path}/mfx/mfxdefs.h"))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
