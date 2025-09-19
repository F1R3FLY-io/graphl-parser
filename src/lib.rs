use std::ffi::{CStr, CString};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::guard::{Guard, Guarded};

#[cfg(target_arch = "wasm32")]
mod wasm;

pub mod ast;
mod bindings;
mod guard;
mod visitor;
mod walker;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = parseToAst))]
pub fn parse_to_ast(code: String) -> Result<ast::Graph, ast::Error> {
    let c_code = CString::new(code).map_err(|err| ast::Error::InvalidCString {
        position: err.nul_position(),
    })?;
    let graph = unsafe { bindings::psGraph(c_code.as_ptr()) }.guarded();

    if graph.is_null() {
        return Err(ast::Error::InvalidGraphL);
    }

    (*graph).try_into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = astToGraphl))]
pub fn ast_to_graphl(ast: ast::Graph) -> Result<String, ast::Error> {
    let ast: Guard<_> = ast.try_into()?;

    let graphl = unsafe { bindings::printGraph(*ast) };

    if graphl.is_null() {
        return Err(ast::Error::InvalidGraphL);
    }

    scopeguard::defer!({
        unsafe { bindings::bufReset() };
    });

    unsafe { CStr::from_ptr(graphl) }
        .to_str()
        .map(ToOwned::to_owned)
        .map_err(|_| ast::Error::InvalidUtf8String)
}
