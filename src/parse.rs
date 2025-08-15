use std::ffi::CString;

use crate::{Visitor, free_Graph, psGraph, visitGraph};

pub fn parse(document: impl Into<CString>) -> Result<std::string::String, String> {
    let mut visitor = Visitor::default();
    let document = document.into();
    unsafe {
        let graph = psGraph(document.as_ptr());
        if graph.is_null() {
            return Err("psGraph returned null".into());
        }
        visitGraph(graph, &mut visitor);
        free_Graph(graph);
    };

    Ok(visitor.to_string())
}

#[test]
fn test_visit_graph() {
    let graph = unsafe { crate::make_GNil() };
    let mut result = String::new();

    let mut visitor = Visitor {
        visitIsGNilCallback: Some(unsafe {
            std::mem::transmute(|_: &mut crate::Graph| {
                result.push_str("visitIsGNilCallback is called");
            })
        }),
        ..Default::default()
    };

    unsafe { visitGraph(graph, &mut visitor) };
    assert_eq!(result, "Hello");
}
