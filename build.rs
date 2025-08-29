use std::path::PathBuf;

const INCLUDE_DIR: &str = "parser";
const WRAPPER_HEADER_FILE: &str = "parser/wrapper.h";
const BINDINGS_FILE: &str = "bindings.rs";

fn main() {
    println!("cargo:rerun-if-changed=parser");

    let target = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    compile_in_parser(&target);
    generate_bindings(&target);
}

fn generate_bindings(target: &str) {
    let mut bindings = bindgen::Builder::default()
        .header(WRAPPER_HEADER_FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true);

    if target == "wasm32" {
        bindings = bindings.clang_args(["-fvisibility=default"]);
    }

    let bindings = bindings.generate().unwrap();

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(BINDINGS_FILE);
    bindings.write_to_file(out_path).unwrap()
}

fn compile_in_parser(target: &str) {
    let mut cc = cc::Build::new();

    if target == "wasm32" {
        cc.file("parser/wasm.c").flag("-DWASM=1");
    } else {
        cc.flag("-std=gnu99");
    }

    cc.files([
        "parser/Absyn.c",
        "parser/Buffer.c",
        "parser/Lexer.c",
        "parser/Parser.c",
        "parser/Printer.c",
        "parser/Skeleton.c",
    ])
    .flag("-Werror=implicit-function-declaration")
    .include(INCLUDE_DIR);

    cc.compile("parser")
}
