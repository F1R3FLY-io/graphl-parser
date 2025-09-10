#![allow(dead_code, non_snake_case, unused_variables)]

use std::ffi::{CString, c_void};
use std::str::FromStr;

use crate::bindings::{
    Attr, AttrName, AttrVal, Binding, Graph, GraphBinding, Ident, LVar, ListAttr, ListName, Name,
    UVar, Vertex, Visitor, free_Graph, psGraph, visitGraph,
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

visitor_callback!(visitIsGTensorCallback, Graph, |p, context| format!(
    "{context} visitIsGTensorCallback"
));

visitor_callback!(visitIsGNominate, Graph, |p, context| format!(
    "{context} visitIsGNominate"
));
visitor_callback!(visitIsGEdgeAnon, Graph, |p, context| format!(
    "{context} visitIsGEdgeAnon"
));
visitor_callback!(visitIsGEdgeNamed, Graph, |p, context| format!(
    "{context} visitIsGEdgeNamed"
));
visitor_callback!(visitIsGRuleAnonCallback, Graph, |p, context| format!(
    "{context} visitIsGRuleAnonCallback"
));
visitor_callback!(visitIsGRuleNamedCallback, Graph, |p, context| format!(
    "{context} visitIsGRuleNamedCallback"
));
visitor_callback!(visitBindingCallback, Binding, |p, context| format!(
    "{context} visitBindingCallback"
));
visitor_callback!(visitGBindCallback, GraphBinding, |p, context| format!(
    "{context} visitGraphBindingCallback"
));
visitor_callback!(visitVertexCallback, Vertex, |p, context| format!(
    "{context} visitVertexCallback"
));
visitor_callback!(visitIsGVarCallback, Graph, |p, context| format!(
    "{context} visitIsGVarCallback"
));
visitor_callback!(visitNameCallback, Name, |p, context| format!(
    "{context} visitNameCallback"
));
visitor_callback!(visitIsGSubgraphCallback, Graph, |p, context| format!(
    "{context} visitIsGSubgraphCallback"
));
visitor_callback!(visitUVar, *mut i8, |p, context| format!(
    "{context} visitUVar"
));
visitor_callback!(visitLVar, *mut i8, |p, context| format!(
    "{context} visitLVar"
));
visitor_callback!(visitIdent, *mut i8, |p, context| format!(
    "{context} visitIdent"
));
visitor_callback!(visitIntegerCallback, i32, |p, context| format!(
    "{context} visitIntegerCallback"
));
visitor_callback!(visitDoubleCallback, f64, |p, context| format!(
    "{context} visitDoubleCallback"
));
visitor_callback!(visitCharCallback, i8, |p, context| format!(
    "{context} visitCharCallback"
));
visitor_callback!(visitStringCallback, *mut i8, |p, context| format!(
    "{context} visitStringCallback"
));
visitor_callback!(visitIsGVertexCallback, Graph, |p, context| format!(
    "{context} visitIsGVertexCallback"
));
visitor_callback!(visitIsGNilCallback, Graph, |p, context| format!(
    "{context}"
));
visitor_callback!(visitIsVBindCallback, Binding, |p, context| format!(
    "{context} visitIsVBindCallback"
));
visitor_callback!(visitIsGBindCallback, GraphBinding, |p, context| format!(
    "{context} visitIsGBindCallback"
));
visitor_callback!(visitIsVNameCallback, Vertex, |p, context| format!(
    "{context} visitIsVNameCallback"
));
visitor_callback!(visitNameWildcardCallback, Name, |p, context| format!(
    "{context} visitNameWildcardCallback"
));
visitor_callback!(visitNameVVarCallback, Name, |p, context| format!(
    "{context} visitNameVVarCallback"
));
visitor_callback!(visitNameGVarCallback, Name, |p, context| format!(
    "{context} visitNameGVarCallback"
));
visitor_callback!(visitIsNameQuoteGraph, Name, |p, context| format!(
    "{context} visitIsNameQuoteGraph"
));
visitor_callback!(visitIsNameQuoteVertex, Name, |p, context| format!(
    "{context} visitIsNameQuoteVertex"
));
visitor_callback!(visitListName, ListName, |p, context| format!(
    "{context} visitListName"
));
visitor_callback!(visitListNameCallback, ListName, |p, context| format!(
    "{context} visitListNameCallback"
));
visitor_callback!(visitIdentCallback, Ident, |p, context| {
    format!("{context} visitIdentCallback")
});
visitor_callback!(visitUVarCallback, UVar, |p, context| format!(
    "{context} visitUVarCallback"
));

visitor_callback!(visitIsGEdgeNamedCallback, Graph, |p, context| format!(
    "{context} visitIsGEdgeNamedCallback"
));

visitor_callback!(visitIsGEdgeAnonCallback, Graph, |p, context| {
    format!("{context} visitIsGEdgeAnonCallback")
});

visitor_callback!(visitIsGNominateCallback, Graph, |p, context| format!(
    "{context} visitIsGNominateCallback"
));

visitor_callback!(visitLVarCallback, LVar, |p, context| {
    format!("{context} visitLVarCallback")
});

visitor_callback!(visitAttrCallback, Attr, |p, context| {
    format!("{context} visitAttrCallback")
});

visitor_callback!(visitAttrNameCallback, AttrName, |p, context| {
    format!("{context} visitAttrNameCallback")
});

visitor_callback!(visitAttrValCallback, AttrVal, |p, context| {
    format!("{context} visitAttrValCallback")
});

visitor_callback!(visitGEdgeNamedCallback, Graph, |p, context| format!(
    "{context} visitGEdgeNamedCallback"
));
visitor_callback!(visitGNilCallback, Graph, |p, context| {
    format!("{context} Nil")
});
visitor_callback!(visitGNominateCallback, Graph, |p, context| format!(
    "{context} visitGNominateCallback"
));
visitor_callback!(visitGRuleAnonCallback, Graph, |p, context| format!(
    "{context} visitGRuleAnonCallback"
));
visitor_callback!(visitGRuleNamedCallback, Graph, |p, context| format!(
    "{context} visitGRuleNamedCallback"
));
visitor_callback!(visitGSubgraphCallback, Graph, |p, context| format!(
    "{context} visitGSubgraphCallback"
));
visitor_callback!(visitGTensorCallback, Graph, |p, context| format!(
    "{context} visitGTensorCallback"
));
visitor_callback!(visitGVarCallback, Graph, |p, context| format!(
    "{context} visitGVarCallback"
));
visitor_callback!(visitGVertexCallback, Graph, |p, context| format!(
    "{context} visitGVertexCallback"
));
visitor_callback!(visitIsAttrListCallback, ListAttr, |p, context| format!(
    "{context} visitIsAttrListCallback"
));
visitor_callback!(
    visitIsAttributeNameCallback,
    AttrName,
    |p, context| format!("{context} visitIsAttributeNameCallback")
);
visitor_callback!(visitIsAttributePairCallback, Attr, |p, context| format!(
    "{context} visitIsAttributePairCallback"
));
visitor_callback!(
    visitIsAttributeValueCallback,
    AttrVal,
    |p, context| format!("{context} visitIsAttributeValueCallback")
);
visitor_callback!(visitIsNameGVarCallback, Name, |p, context| format!(
    "{context} visitIsNameGVarCallback"
));
visitor_callback!(visitIsNameQuoteGraphCallback, Name, |p, context| format!(
    "{context} visitIsNameQuoteGraphCallback"
));
visitor_callback!(visitIsNameQuoteVertexCallback, Name, |p, context| format!(
    "{context} visitIsNameQuoteVertexCallback"
));
visitor_callback!(visitIsNameVVarCallback, Name, |p, context| format!(
    "{context} visitIsNameVVarCallback"
));
visitor_callback!(visitIsNameWildcardCallback, Name, |p, context| format!(
    "{context} visitIsNameWildcardCallback"
));
visitor_callback!(
    visitIsEmptyAttrListCallback,
    ListAttr,
    |p, context| format!("{context} visitIsEmptyAttrListCallback")
);
visitor_callback!(visitListAttrCallback, ListAttr, |p, context| format!(
    "{context} visitListAttrCallback"
));
visitor_callback!(visitVBindCallback, Binding, |p, context| format!(
    "{context} visitVBindCallback"
));

visitor_callback!(visitGEdgeAnonCallback, Graph, |p, context| format!(
    "{context} visitGEdgeAnonCallback"
));

pub fn parse(document: String) -> Result<String, String> {
    let mut visitor = Visitor {
        visitAttrCallback: Some(visitAttrCallback),
        visitAttrNameCallback: Some(visitAttrNameCallback),
        visitAttrValCallback: Some(visitAttrValCallback),
        visitBindingCallback: Some(visitBindingCallback),
        visitGBindCallback: Some(visitGBindCallback),
        visitGEdgeAnonCallback: Some(visitGEdgeAnonCallback),
        visitGEdgeNamedCallback: Some(visitGEdgeNamedCallback),
        visitGNilCallback: Some(visitGNilCallback),
        visitGNominateCallback: Some(visitGNominateCallback),
        visitGRuleAnonCallback: Some(visitGRuleAnonCallback),
        visitGRuleNamedCallback: Some(visitGRuleNamedCallback),
        visitGSubgraphCallback: Some(visitGSubgraphCallback),
        visitGTensorCallback: Some(visitGTensorCallback),
        visitGVarCallback: Some(visitGVarCallback),
        visitGVertexCallback: Some(visitGVertexCallback),
        visitIdentCallback: Some(visitIdentCallback),
        visitIntegerCallback: Some(visitIntegerCallback),
        visitIsAttrListCallback: Some(visitIsAttrListCallback),
        visitIsAttributeNameCallback: Some(visitIsAttributeNameCallback),
        visitIsAttributePairCallback: Some(visitIsAttributePairCallback),
        visitIsAttributeValueCallback: Some(visitIsAttributeValueCallback),
        visitIsGBindCallback: Some(visitIsGBindCallback),
        visitIsGEdgeAnonCallback: Some(visitIsGEdgeAnonCallback),
        visitIsGEdgeNamedCallback: Some(visitIsGEdgeNamedCallback),
        visitIsGNilCallback: Some(visitIsGNilCallback),
        visitIsGNominateCallback: Some(visitIsGNominateCallback),
        visitIsGRuleAnonCallback: Some(visitIsGRuleAnonCallback),
        visitIsGRuleNamedCallback: Some(visitIsGRuleNamedCallback),
        visitIsGSubgraphCallback: Some(visitIsGSubgraphCallback),
        visitIsGTensorCallback: Some(visitIsGTensorCallback),
        visitIsGVarCallback: Some(visitIsGVarCallback),
        visitIsGVertexCallback: Some(visitIsGVertexCallback),
        visitIsNameGVarCallback: Some(visitIsNameGVarCallback),
        visitIsNameQuoteGraphCallback: Some(visitIsNameQuoteGraphCallback),
        visitIsNameQuoteVertexCallback: Some(visitIsNameQuoteVertexCallback),
        visitIsNameVVarCallback: Some(visitIsNameVVarCallback),
        visitIsNameWildcardCallback: Some(visitIsNameWildcardCallback),
        visitIsVBindCallback: Some(visitIsVBindCallback),
        visitIsVNameCallback: Some(visitIsVNameCallback),
        visitIsEmptyAttrListCallback: Some(visitIsEmptyAttrListCallback),
        visitListAttrCallback: Some(visitListAttrCallback),
        visitListNameCallback: Some(visitListNameCallback),
        visitLVarCallback: Some(visitLVarCallback),
        visitNameCallback: Some(visitNameCallback),
        visitNameGVarCallback: Some(visitNameGVarCallback),
        visitNameVVarCallback: Some(visitNameVVarCallback),
        visitNameWildcardCallback: Some(visitNameWildcardCallback),
        visitStringCallback: Some(visitStringCallback),
        visitUVarCallback: Some(visitUVarCallback),
        visitVBindCallback: Some(visitVBindCallback),
        visitVertexCallback: Some(visitVertexCallback),
        visitDoubleCallback: Some(visitDoubleCallback),
        visitCharCallback: Some(visitCharCallback),
    };

    let document = CString::from_str(&document).map_err(|e| e.to_string())?;
    let ptr = document.as_ptr();

    let contract = r#""#;

    let rholang_representation = String::from_str(contract).map(Box::new).unwrap();
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

    Ok(*rholang_representation)
}

#[cfg(test)]
mod tests {
    use std::ffi::{CString, c_void};
    use std::str::FromStr;

    use crate::bindings::{Graph, Visitor};
    use crate::parse::{get_context, parse, save_context};

    #[test]
    fn test_parse_empty_graph() {
        let statement = String::from_str("{0}").unwrap();
        let result = parse(statement).unwrap();

        assert_eq!(result, "");
    }

    #[test]
    fn test_vertex() {
        let statement = String::from_str("<a> | 0").unwrap();
        let result = parse(statement).unwrap();

        assert_eq!(
            result,
            r#"contract "my_workflow" (contract_return) = {
          new a in {
            a!(*contract_return)
          }
        }"#
        );
    }

    #[test]
    fn test_graph() {
        let statement = String::from_str(
            r#"{(
                  let xa = <a> in <a> | 0,
                  let xb = <b> in <b> | 0
                )}"#,
        )
        .unwrap();
        let result = parse(statement).unwrap();

        assert_eq!(
            result,
            r#"contract "test"(arg_1, arg_2, contract_return) = {
              new a, b a_ret in {
                a!(*arg_1, *arg_2, *a_ret) |
                for (res <- a_ret) {
                  b!(*res, *contract_return)
                }
              }
          }"#
        )
    }

    #[test]
    fn test_visit_callback_declaration_graph() {
        let context = String::from_str("Hello, ").unwrap();

        visitor_callback!(visitIsGNilCallback, Graph, |p, context| format!(
            "{context}Gnil Called"
        ));

        let mut visitor = Visitor {
            visitIsGNilCallback: Some(visitIsGNilCallback),
            ..Default::default()
        };
        let context = Box::into_raw(Box::new(context));
        let context_ptr = context as *mut c_void;

        let statement = CString::new("{0}").unwrap();
        let graph = unsafe { crate::bindings::psGraph(statement.as_ptr()) };
        unsafe { crate::bindings::visitGraph(graph, &mut visitor, context_ptr) };

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
