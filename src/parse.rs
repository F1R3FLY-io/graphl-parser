use std::{
    ffi::{CString, c_void},
    str::FromStr,
};

use crate::{
    Binding, Graph, GraphBinding, ListName, Name, Vertex, Visitor, free_Graph, psGraph, visitGraph,
};

macro_rules! visitor_callback {
    ($visitor_fn_name:ident, $graph_element_type:ty, $callback:expr) => {
        unsafe extern "C" fn $visitor_fn_name(p: $graph_element_type, context: *mut c_void) {
            if let Some(ctx) = get_context(context as *mut String) {
                let result: String = $callback(p, ctx);
                save_context(context as *mut String, result);
            }
        }
    };
}

fn get_context(context: *mut String) -> Option<&'static mut String> {
    if context.is_null() || context.is_null() {
        None
    } else {
        unsafe { Some(&mut *context) }
    }
}

fn save_context(context: *mut String, content: String) {
    unsafe {
        if let Some(ctx) = context.as_mut() {
            *ctx = content;
        }
    };
}

visitor_callback!(visitIsGTensorCallback, Graph, |_p, context| format!(
    "{context}Gnil Called"
));

visitor_callback!(visitIsGNominate, Graph, |_p, context| format!(
    "{context}visitIsGNominate"
));
visitor_callback!(visitIsGEdgeAnon, Graph, |_p, context| format!(
    "{context}visitIsGEdgeAnon"
));
visitor_callback!(visitIsGEdgeNamed, Graph, |_p, context| format!(
    "{context}visitIsGEdgeNamed"
));
visitor_callback!(visitIsGRuleAnonCallback, Graph, |_p, context| format!(
    "{context}visitIsGRuleAnonCallback"
));
visitor_callback!(visitIsGRuleNamedCallback, Graph, |_p, context| format!(
    "{context}visitIsGRuleNamedCallback"
));
visitor_callback!(visitBindingCallback, Binding, |_p, context| format!(
    "{context}visitBindingCallback"
));
visitor_callback!(
    visitGraphBindingCallback,
    GraphBinding,
    |_p, context| format!("{context}visitGraphBindingCallback")
);
visitor_callback!(visitVertexCallback, Vertex, |_p, context| format!(
    "{context}visitVertexCallback"
));
visitor_callback!(visitIsGVarCallback, Graph, |_p, context| format!(
    "{context}visitIsGVarCallback"
));
visitor_callback!(visitNameCallback, Name, |_p, context| format!(
    "{context}visitNameCallback"
));
visitor_callback!(visitIsGSubgraphCallback, Graph, |_p, context| format!(
    "{context}visitIsGSubgraphCallback"
));
visitor_callback!(visitUVar, *mut i8, |_p, context| format!(
    "{context}visitUVar"
));
visitor_callback!(visitLVar, *mut i8, |_p, context| format!(
    "{context}visitLVar"
));
visitor_callback!(visitIdent, *mut i8, |_p, context| format!(
    "{context}visitIdent"
));
visitor_callback!(visitIntegerCallback, i32, |_p, context| format!(
    "{context}visitIntegerCallback"
));
visitor_callback!(visitDoubleCallback, f64, |_p, context| format!(
    "{context}visitDoubleCallback"
));
visitor_callback!(visitCharCallback, i8, |_p, context| format!(
    "{context}visitCharCallback"
));
visitor_callback!(visitStringCallback, *mut i8, |_p, context| format!(
    "{context}visitStringCallback"
));
visitor_callback!(visitIsGVertexCallback, Graph, |_p, context| format!(
    "{context}visitIsGVertexCallback"
));
visitor_callback!(visitIsGNilCallback, Graph, |_p, context| format!(
    "{context}visitIsGNilCallback"
));
visitor_callback!(visitIsVBindCallback, Binding, |_p, context| format!(
    "{context}visitIsVBindCallback"
));
visitor_callback!(visitIsGBindCallback, GraphBinding, |_p, context| format!(
    "{context}visitIsGBindCallback"
));
visitor_callback!(visitIsVNameCallback, Vertex, |_p, context| format!(
    "{context}visitIsVNameCallback"
));
visitor_callback!(visitNameWildcardCallback, Name, |_p, context| format!(
    "{context}visitNameWildcardCallback"
));
visitor_callback!(visitNameVVarCallback, Name, |_p, context| format!(
    "{context}visitNameVVarCallback"
));
visitor_callback!(visitNameGVarCallback, Name, |_p, context| format!(
    "{context}visitNameGVarCallback"
));
visitor_callback!(visitIsNameQuoteGraph, Name, |_p, context| format!(
    "{context}visitIsNameQuoteGraph"
));
visitor_callback!(visitIsNameQuoteVertex, Name, |_p, context| format!(
    "{context}visitIsNameQuoteVertex"
));
visitor_callback!(visitListName, ListName, |_p, context| format!(
    "{context}visitListName"
));

pub fn parse(document: String) -> std::result::Result<String, String> {
    let mut visitor = Visitor {
        visitIsGTensorCallback: Some(visitIsGTensorCallback),
        visitIsGNominate: Some(visitIsGNominate),
        visitIsGEdgeAnon: Some(visitIsGEdgeAnon),
        visitIsGEdgeNamed: Some(visitIsGEdgeNamed),
        visitIsGRuleAnonCallback: Some(visitIsGRuleAnonCallback),
        visitIsGRuleNamedCallback: Some(visitIsGRuleNamedCallback),
        visitBindingCallback: Some(visitBindingCallback),
        visitGraphBindingCallback: Some(visitGraphBindingCallback),
        visitVertexCallback: Some(visitVertexCallback),
        visitIsGVarCallback: Some(visitIsGVarCallback),
        visitNameCallback: Some(visitNameCallback),
        visitIsGSubgraphCallback: Some(visitIsGSubgraphCallback),
        visitUVar: Some(visitUVar),
        visitLVar: Some(visitLVar),
        visitIdent: Some(visitIdent),
        visitIntegerCallback: Some(visitIntegerCallback),
        visitDoubleCallback: Some(visitDoubleCallback),
        visitCharCallback: Some(visitCharCallback),
        visitStringCallback: Some(visitStringCallback),
        visitIsGVertexCallback: Some(visitIsGVertexCallback),
        visitIsGNilCallback: Some(visitIsGNilCallback),
        visitIsVBindCallback: Some(visitIsVBindCallback),
        visitIsGBindCallback: Some(visitIsGBindCallback),
        visitIsVNameCallback: Some(visitIsVNameCallback),
        visitNameWildcardCallback: Some(visitNameWildcardCallback),
        visitNameVVarCallback: Some(visitNameVVarCallback),
        visitNameGVarCallback: Some(visitNameGVarCallback),
        visitIsNameQuoteGraph: Some(visitIsNameQuoteGraph),
        visitIsNameQuoteVertex: Some(visitIsNameQuoteVertex),
        visitListName: Some(visitListName),
    };

    let document = CString::from_str(&document).map_err(|e| e.to_string())?;
    let ptr = document.as_ptr();

    let rholang_representation = String::from_str("contract %contract_name(%arguments){%body}")
        .map(Box::new)
        .unwrap();
    let rholang_representation_ptr = Box::into_raw(rholang_representation);

    unsafe {
        let graph = psGraph(ptr);
        if graph.is_null() {
            return Err("psGraph returned null".to_string());
        }
        visitGraph(
            graph,
            &mut visitor,
            rholang_representation_ptr as *mut c_void,
        );
        free_Graph(graph);
    };

    let rholang_representation = unsafe { Box::from_raw(rholang_representation_ptr) };

    let result = Ok(*rholang_representation);

    result
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString, c_void},
        os::raw::c_char,
        str::FromStr,
    };

    use crate::{
        Graph, Visitor,
        parse::{get_context, save_context},
    };

    use super::parse;

    #[test]
    fn test_parse_empty_graph() {
        let statement = String::from_str("{0}").unwrap();
        let result = parse(statement).unwrap();

        assert_eq!(
            result,
            "contract %contract_name(%arguments){%body}visitIsGNilCallback"
        );
    }

    #[test]
    fn test_visit_callback_declaration_graph() {
        let context = String::from_str("Hello, ").unwrap();

        visitor_callback!(visitIsGNilCallback, Graph, |_p, context| format!(
            "{context}Gnil Called"
        ));

        let mut visitor = Visitor {
            visitIsGNilCallback: Some(visitIsGNilCallback),
            ..Default::default()
        };
        let context = Box::into_raw(Box::new(context));
        let context_ptr = context as *mut c_void;

        let statement = CString::new("{0}").unwrap();
        let graph = unsafe { crate::psGraph(statement.as_ptr()) };
        unsafe { crate::visitGraph(graph, &mut visitor, context_ptr) };

        let c = unsafe { Box::from_raw(context_ptr as *mut String) };
        let context = c.as_str();

        assert_eq!(context, "Hello, Gnil Called");
    }

    #[test]
    fn test_get_context() {
        let context = Box::new(String::from_str("Hello, world").unwrap());
        let ptr = Box::into_raw(context);

        let result = get_context(ptr).unwrap().clone();

        assert_eq!(result, String::from_str("Hello, world").unwrap());
    }

    #[test]
    fn test_save_context() {
        let context = Box::new(String::from_str("Hello, world").unwrap());
        let ptr = Box::into_raw(context);

        save_context(ptr, "Good bey, world!".into());

        // reconstruct context from pointer
        let context = unsafe { Box::from_raw(ptr) };

        assert_eq!(*context, String::from_str("Good bey, world!").unwrap());
    }
}
