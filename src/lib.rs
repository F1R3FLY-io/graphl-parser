#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
mod wasm;

// pub mod ast;
mod bindings;
mod guard;
mod parse;
mod rholang;
