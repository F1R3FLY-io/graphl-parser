use std::ffi::{CStr, CString};

use crate::{free_Graph, psGraph, visitGraph};

pub fn parse(
    document: impl Into<CString>,
) -> Result<std::string::String, std::ffi::IntoStringError> {
    unsafe {
        psGraph(document.into().as_ptr());
        visitGraph(graph);
        free_Graph(graph);
    }
    .to_owned()
    .into_string()
}
