#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub mod ast;
mod bindings;
mod parse;
mod print;
mod show;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_to_ast(code: String) -> Result<ast::Graph, ast::Error> {
    let c_code = std::ffi::CString::new(code).map_err(|err| ast::Error::InvalidCString {
        position: err.nul_position(),
    })?;
    let graph = unsafe { bindings::psGraph(c_code.as_ptr()) };

    if graph.is_null() {
        return Err(ast::Error::InvalidGraphL);
    }

    scopeguard::defer!({
        unsafe { bindings::free_Graph(graph) };
    });

    graph.try_into()
}

pub mod utils {
    pub use crate::print::*;
    pub use crate::show::*;
}
