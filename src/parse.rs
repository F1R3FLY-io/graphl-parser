use std::{
    ffi::{CString, c_void},
    str::FromStr,
};

use crate::{
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
    "{context}visitIsGTensorCallback"
));

visitor_callback!(visitIsGNominate, Graph, |p, context| format!(
    "{context}visitIsGNominate"
));
visitor_callback!(visitIsGEdgeAnon, Graph, |p, context| format!(
    "{context}visitIsGEdgeAnon"
));
visitor_callback!(visitIsGEdgeNamed, Graph, |p, context| format!(
    "{context}visitIsGEdgeNamed"
));
visitor_callback!(visitIsGRuleAnonCallback, Graph, |p, context| format!(
    "{context}visitIsGRuleAnonCallback"
));
visitor_callback!(visitIsGRuleNamedCallback, Graph, |p, context| format!(
    "{context}visitIsGRuleNamedCallback"
));
visitor_callback!(visitBindingCallback, Binding, |p, context| format!(
    "{context}visitBindingCallback"
));
visitor_callback!(
    visitGraphBindingCallback,
    GraphBinding,
    |p, context| format!("{context}visitGraphBindingCallback")
);
visitor_callback!(visitVertexCallback, Vertex, |p, context| format!(
    "{context}visitVertexCallback"
));
visitor_callback!(visitIsGVarCallback, Graph, |p, context| format!(
    "{context}visitIsGVarCallback"
));
visitor_callback!(visitNameCallback, Name, |p, context| format!(
    "{context}visitNameCallback"
));
visitor_callback!(visitIsGSubgraphCallback, Graph, |p, context| format!(
    "{context}visitIsGSubgraphCallback"
));
visitor_callback!(visitUVar, *mut i8, |p, context| format!(
    "{context}visitUVar"
));
visitor_callback!(visitLVar, *mut i8, |p, context| format!(
    "{context}visitLVar"
));
visitor_callback!(visitIdent, *mut i8, |p, context| format!(
    "{context}visitIdent"
));
visitor_callback!(visitIntegerCallback, i32, |p, context| format!(
    "{context}visitIntegerCallback"
));
visitor_callback!(visitDoubleCallback, f64, |p, context| format!(
    "{context}visitDoubleCallback"
));
visitor_callback!(visitCharCallback, i8, |p, context| format!(
    "{context}visitCharCallback"
));
visitor_callback!(visitStringCallback, *mut i8, |p, context| format!(
    "{context}visitStringCallback"
));
visitor_callback!(visitIsGVertexCallback, Graph, |p, context| format!(
    "{context}visitIsGVertexCallback"
));
visitor_callback!(
    visitIsGNilCallback,
    Graph,
    |p, context: &'static mut String| context.replace("new %vertexes in {%placehodler}", "")
);
visitor_callback!(visitIsVBindCallback, Binding, |p, context| format!(
    "{context}visitIsVBindCallback"
));
visitor_callback!(visitIsGBindCallback, GraphBinding, |p, context| format!(
    "{context}visitIsGBindCallback"
));
visitor_callback!(visitIsVNameCallback, Vertex, |p, context| format!(
    "{context}visitIsVNameCallback"
));
visitor_callback!(visitNameWildcardCallback, Name, |p, context| format!(
    "{context}visitNameWildcardCallback"
));
visitor_callback!(visitNameVVarCallback, Name, |p, context| format!(
    "{context}visitNameVVarCallback"
));
visitor_callback!(visitNameGVarCallback, Name, |p, context| format!(
    "{context}visitNameGVarCallback"
));
visitor_callback!(visitIsNameQuoteGraph, Name, |p, context| format!(
    "{context}visitIsNameQuoteGraph"
));
visitor_callback!(visitIsNameQuoteVertex, Name, |p, context| format!(
    "{context}visitIsNameQuoteVertex"
));
visitor_callback!(visitListName, ListName, |p, context| format!(
    "{context}visitListName"
));
visitor_callback!(
    visitGraphCallback,
    Graph,
    |p, context: &'static mut String| {
        let placeholder = "new %vertexes in {%placehodler}";
        context.replace("%placeholder", placeholder)
    }
);

visitor_callback!(
    visitAttrCallback,
    Attr,
    |p, context: &'static mut String| { format!("{context}visitAttrCallback") }
);
visitor_callback!(
    visitAttrNameCallback,
    AttrName,
    |p, context: &'static mut String| { format!("{context}visitAttrNameCallback") }
);
visitor_callback!(
    visitAttrValCallback,
    AttrVal,
    |p, context: &'static mut String| { format!("{context}visitAttrValCallback") }
);
visitor_callback!(
    visitGBindCallback,
    GraphBinding,
    |p, context: &'static mut String| { format!("{context}visitGBindCallback") }
);
visitor_callback!(
    visitGEdgeAnonCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGEdgeAnonCallback") }
);
visitor_callback!(
    visitGEdgeNamedCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGEdgeNamedCallback") }
);
visitor_callback!(
    visitGNilCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGNilCallback") }
);
visitor_callback!(
    visitGNominateCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGNominateCallback") }
);
visitor_callback!(
    visitGRuleAnonCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGRuleAnonCallback") }
);
visitor_callback!(
    visitGRuleNamedCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGRuleNamedCallback") }
);
visitor_callback!(
    visitGSubgraphCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGSubgraphCallback") }
);
visitor_callback!(
    visitGTensorCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGTensorCallback") }
);
visitor_callback!(
    visitGVarCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGVarCallback") }
);
visitor_callback!(
    visitGVertexCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitGVertexCallback") }
);
visitor_callback!(
    visitIdentCallback,
    Ident,
    |p, context: &'static mut String| { format!("{context}visitIdentCallback") }
);
visitor_callback!(
    visitIsAttrListCallback,
    Attr,
    |p, context: &'static mut String| { format!("{context}visitIsAttrListCallback") }
);
visitor_callback!(
    visitIsAttributeNameCallback,
    LVar,
    |p, context: &'static mut String| { format!("{context}visitIsAttributeNameCallback") }
);
visitor_callback!(
    visitIsAttributePairCallback,
    AttrName,
    |p, context: &'static mut String| { format!("{context}visitIsAttributePairCallback") }
);
visitor_callback!(
    visitIsAttributeValueCallback,
    LVar,
    |p, context: &'static mut String| { format!("{context}visitIsAttributeValueCallback") }
);
visitor_callback!(
    visitIsGEdgeAnonCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitIsGEdgeAnonCallback") }
);
visitor_callback!(
    visitIsGEdgeNamedCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitIsGEdgeNamedCallback") }
);
visitor_callback!(
    visitIsGNominateCallback,
    Graph,
    |p, context: &'static mut String| { format!("{context}visitIsGNominateCallback") }
);
visitor_callback!(
    visitIsNameGVarCallback,
    Name,
    |p, context: &'static mut String| { format!("{context}visitIsNameGVarCallback") }
);
visitor_callback!(
    visitIsNameQuoteGraphCallback,
    Name,
    |p, context: &'static mut String| { format!("{context}visitIsNameQuoteGraphCallback") }
);
visitor_callback!(
    visitIsNameQuoteVertexCallback,
    Name,
    |p, context: &'static mut String| { format!("{context}visitIsNameQuoteVertexCallback") }
);
visitor_callback!(
    visitIsNameVVarCallback,
    Name,
    |p, context: &'static mut String| { format!("{context}visitIsNameVVarCallback") }
);
visitor_callback!(
    visitIsNameWildcardCallback,
    Name,
    |p, context: &'static mut String| { format!("{context}visitIsNameWildcardCallback") }
);
visitor_callback!(
    visitIsEmptyAttrListCallback,
    ListAttr,
    |p, context: &'static mut String| { format!("{context}visitIsEmptyAttrListCallback") }
);
visitor_callback!(
    visitListAttrCallback,
    ListAttr,
    |p, context: &'static mut String| { format!("{context}visitListAttrCallback") }
);
visitor_callback!(
    visitListNameCallback,
    ListName,
    |p, context: &'static mut String| { format!("{context}visitListNameCallback") }
);
visitor_callback!(
    visitLVarCallback,
    LVar,
    |p, context: &'static mut String| { format!("{context}visitLVarCallback") }
);
visitor_callback!(
    visitUVarCallback,
    UVar,
    |p, context: &'static mut String| { format!("{context}visitUVarCallback") }
);
visitor_callback!(
    visitVBindCallback,
    Binding,
    |p, context: &'static mut String| { format!("{context}visitVBindCallback") }
);

pub fn parse(document: String) -> std::result::Result<String, String> {
    let mut visitor = Visitor {
        visitIsGTensorCallback: Some(visitIsGTensorCallback),
        visitIsGRuleAnonCallback: Some(visitIsGRuleAnonCallback),
        visitIsGRuleNamedCallback: Some(visitIsGRuleNamedCallback),
        visitBindingCallback: Some(visitBindingCallback),
        visitVertexCallback: Some(visitVertexCallback),
        visitIsGVarCallback: Some(visitIsGVarCallback),
        visitNameCallback: Some(visitNameCallback),
        visitIsGSubgraphCallback: Some(visitIsGSubgraphCallback),
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
        visitGraphCallback: Some(visitGraphCallback),
        visitAttrCallback: Some(visitAttrCallback),
        visitAttrNameCallback: Some(visitAttrNameCallback),
        visitAttrValCallback: Some(visitAttrValCallback),
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
        visitIsAttrListCallback: Some(visitIsAttrListCallback),
        visitIsAttributeNameCallback: Some(visitIsAttributeNameCallback),
        visitIsAttributePairCallback: Some(visitIsAttributePairCallback),
        visitIsAttributeValueCallback: Some(visitIsAttributeValueCallback),
        visitIsGEdgeAnonCallback: Some(visitIsGEdgeAnonCallback),
        visitIsGEdgeNamedCallback: Some(visitIsGEdgeNamedCallback),
        visitIsGNominateCallback: Some(visitIsGNominateCallback),
        visitIsNameGVarCallback: Some(visitIsNameGVarCallback),
        visitIsNameQuoteGraphCallback: Some(visitIsNameQuoteGraphCallback),
        visitIsNameQuoteVertexCallback: Some(visitIsNameQuoteVertexCallback),
        visitIsNameVVarCallback: Some(visitIsNameVVarCallback),
        visitIsNameWildcardCallback: Some(visitIsNameWildcardCallback),
        visitIsEmptyAttrListCallback: Some(visitIsEmptyAttrListCallback),
        visitListAttrCallback: Some(visitListAttrCallback),
        visitListNameCallback: Some(visitListNameCallback),
        visitLVarCallback: Some(visitLVarCallback),
        visitUVarCallback: Some(visitUVarCallback),
        visitVBindCallback: Some(visitVBindCallback),
    };

    let document = CString::from_str(&document).map_err(|e| e.to_string())?;
    let ptr = document.as_ptr();

    let contract = r#"contract %contract_name(%contract_arguments){%placeholder}"#;

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
    use std::{
        ffi::{CString, c_void},
        str::FromStr,
    };

    use crate::{
        Graph, Visitor,
        parse::{get_context, save_context},
    };

    use super::parse;

    #[test]
    fn test_parse_empty_graph() {
        let statement = String::from_str("0").unwrap();
        let result = parse(statement).unwrap();

        assert_eq!(
            result,
            "contract %contract_name(%contract_arguments){%placeholder}visitAttrCallbackvisitIsAttributeNameCallback"
        );
    }

    #[test]
    fn test_visit_callback_declaration_graph() {
        let context = String::from_str("Hello, ").unwrap();

        visitor_callback!(visitIsGNilCallback, Graph, |p, context| format!(
            "{context}Gnil Called"
        ));

        visitor_callback!(visigGraphCallback, Graph, |p, context| format!(
            "{context}Graph Called"
        ));

        let mut visitor = Visitor {
            visitIsGNilCallback: Some(visitIsGNilCallback),
            visitGraphCallback: Some(visigGraphCallback),
            ..Default::default()
        };
        let context = Box::into_raw(Box::new(context));
        let context_ptr = context as *mut c_void;

        let statement = CString::new("{0}").unwrap();
        let graph = unsafe { crate::psGraph(statement.as_ptr()) };
        unsafe { crate::visitGraph(graph, &mut visitor, context_ptr) };

        let c = unsafe { Box::from_raw(context_ptr as *mut String) };
        let context = c.as_str();

        assert_eq!(context, "Hello, Graph CalledGnil Called");
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
