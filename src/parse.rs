use std::ffi::{CString, c_void};

use crate::{Visitor, free_Graph, psGraph, visitGraph};

pub fn parse(document: impl Into<CString>) -> std::result::Result<String, String> {
    let rholang_representation = CString::default();

    let mut visitor = Visitor {
        visitIsGTensorCallback: todo!(),
        visitIsGNominate: todo!(),
        visitIsGEdgeAnon: todo!(),
        visitIsGEdgeNamed: todo!(),
        visitIsGRuleAnonCallback: todo!(),
        visitIsGRuleNamedCallback: todo!(),
        visitBindingCallback: todo!(),
        visitGraphBindingCallback: todo!(),
        visitVertexCallback: todo!(),
        visitIsGVarCallback: todo!(),
        visitNameCallback: todo!(),
        visitIsGSubgraphCallback: todo!(),
        visitUVar: todo!(),
        visitLVar: todo!(),
        visitIdent: todo!(),
        visitIntegerCallback: todo!(),
        visitDoubleCallback: todo!(),
        visitCharCallback: todo!(),
        visitStringCallback: todo!(),
        visitIsGVertexCallback: todo!(),
        visitIsGNilCallback: todo!(),
        visitIsVBindCallback: todo!(),
        visitIsGBindCallback: todo!(),
        visitIsVNameCallback: todo!(),
        visitNameWildcardCallback: todo!(),
        visitNameVVarCallback: todo!(),
        visitNameGVarCallback: todo!(),
        visitIsNameQuoteGraph: todo!(),
        visitIsNameQuoteVertex: todo!(),
        visitListName: todo!(),
    };
    let document = document.into();

    unsafe {
        let graph = psGraph(document.as_ptr());
        if graph.is_null() {
            return Err("psGraph returned null".to_string());
        }
        visitGraph(
            graph,
            &mut visitor,
            rholang_representation.as_ptr() as *mut c_void,
        );
        free_Graph(graph);
    };

    rholang_representation
        .into_string()
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString, c_void},
        mem::forget,
        os::raw::c_char,
        ptr::{copy_nonoverlapping, write},
        str::FromStr,
    };

    use crate::Visitor;

    #[test]
    fn test_visit_graph() {
        let statement = CString::new("{0}").unwrap();
        let graph = unsafe { crate::psGraph(statement.as_ptr()) };
        let context = CString::new("Hello, ").unwrap();

        unsafe extern "C" fn visitIsGNilCallback(_p: crate::Graph, context: *mut c_void) {
            if context.is_null() || context.is_null() {
                return;
            }

            let value = unsafe { CStr::from_ptr(context as *mut c_char).to_str().unwrap() };

            let updated_context = format!("{}Gnil Called", value);
            let updated_context_bytes = updated_context.as_bytes();

            unsafe {
                copy_nonoverlapping(
                    updated_context_bytes.as_ptr(),
                    context as *mut u8,
                    updated_context_bytes.len(),
                );
            }
        }

        let mut visitor = Visitor {
            visitIsGNilCallback: Some(visitIsGNilCallback),
            ..Default::default()
        };
        let context = context.into_raw() as *mut c_void;

        unsafe { crate::visitGraph(graph, &mut visitor, context) };

        let result = unsafe { CStr::from_ptr(context as *mut c_char) };

        assert_eq!(result.to_str().unwrap(), "Hello, Gnil Called");
    }
}
