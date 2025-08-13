use std::path::PathBuf;

const OBJECT_FILES_DIR: &str = "parser";
const LIB_NAME: &str = "parser";
const WRAPPER_HEADER_FILE: &str = "parser/wrapper.h";
const BINDINGS_FILE: &str = "src/bindings.rs";

fn main() {
    println!(
        "cargo:rustc-link-search={}",
        PathBuf::from(OBJECT_FILES_DIR)
            .canonicalize()
            .unwrap()
            .display()
    );
    println!("cargo:rustc-link-lib={}", LIB_NAME);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(WRAPPER_HEADER_FILE)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(BINDINGS_FILE)
        .expect("Couldn't write bindings!");
}
