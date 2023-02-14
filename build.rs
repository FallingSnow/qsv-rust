use std::{path::PathBuf, env};

fn main() {

    println!("cargo:rustc-link-lib=dylib=mfx");
    // println!("cargo:rustc-link-lib=dylib=va");
    println!("cargo:rustc-link-lib=dylib=va-drm");

    let libmfx = pkg_config::probe_library("mfx").unwrap();
    let libvadrm = pkg_config::probe_library("libva-drm").unwrap();
    // https://github.com/Intel-Media-SDK/MediaSDK/blob/master/api/include/mfxvideo.h
    // https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let libmfx_include_path = libmfx.include_paths[0].join("mfx");
    let libvadrm_include_path = libvadrm.include_paths[0].join("va");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(libmfx_include_path.join("mfxvideo.h").to_string_lossy())
        .header(libvadrm_include_path.join("va_drm.h").to_string_lossy())
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
