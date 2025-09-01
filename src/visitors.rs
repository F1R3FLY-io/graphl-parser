use crate::{
    Attr, AttrName, AttrVal, Binding, Graph, GraphBinding, Ident, LVar, ListAttr, ListName, Name,
    UVar, Vertex,
};
use std::ffi::c_void;

use crate::context::{get_context, save_context};

macro_rules! visitor_callback {
    ($visitor_fn_name:ident, $graph_element_type:ty, $callback:expr) => {
        pub(crate) unsafe extern "C" fn $visitor_fn_name(
            p: $graph_element_type,
            context: *mut c_void,
        ) {
            if let Some(ctx) = get_context(context as *mut String) {
                let result: String = $callback(p, ctx);
                save_context(context as *mut String, result);
            }
        }
    };
}

pub(crate) use visitor_callback;

visitor_callback!(visitIsGTensorCallback, Graph, |p, context| format!(
    "{context}visitIsGTensorCallback"
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
