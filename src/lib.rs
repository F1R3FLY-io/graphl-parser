use std::ffi::{CStr, CString};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::guard::{Guard, Guarded};

#[cfg(target_arch = "wasm32")]
mod wasm;

// pub mod ast;
mod bindings;
mod guard;
mod parse;
