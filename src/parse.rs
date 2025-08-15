use std::ffi::CString;

use crate::{Visitor, free_Graph, psGraph, visitGraph};

pub fn parse(
    document: impl Into<CString>,
) -> Result<std::string::String, std::ffi::IntoStringError> {
    let mut visitor = Visitor::default();
    unsafe {
        let graph = psGraph(document.into().as_ptr());
        visitGraph(graph, &mut visitor);
        free_Graph(graph);
    };

    Ok("Ok".to_string())
}
