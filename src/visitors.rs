use crate::{
    Attr, AttrName, AttrVal, Binding, Graph, GraphBinding, Ident, LVar, ListAttr, ListName, Name,
    UVar, Vertex, context::INNER_PLACEHOLDER,
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

visitor_callback!(visitIsGTensorCallback, Graph, |_p, _context| format!(
    "visitIsGTensorCallback"
));

visitor_callback!(visitIsGRuleAnonCallback, Graph, |_p, _context| format!(
    "visitIsGRuleAnonCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIsGRuleNamedCallback, Graph, |_p, _context| format!(
    "visitIsGRuleNamedCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitBindingCallback, Binding, |_p, _context| format!(
    "visitBindingCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitVertexCallback, Vertex, |_p, _context| format!(
    "visitVertexCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIsGVarCallback, Graph, |_p, _context| format!(
    "visitIsGVarCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitNameCallback, Name, |_p, _context| format!(
    "visitNameCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIsGSubgraphCallback, Graph, |_p, _context| format!(
    "visitIsGSubgraphCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIntegerCallback, i32, |_p, _context| format!(
    "visitIntegerCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitDoubleCallback, f64, |_p, _context| format!(
    "visitDoubleCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitCharCallback, i8, |_p, _context| format!(
    "visitCharCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitStringCallback, *mut i8, |_p, _context| format!(
    "visitStringCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIsGVertexCallback, Graph, |_p, _context| format!(
    "visitIsGVertexCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(
    visitIsGNilCallback,
    Graph,
    |_p, context: &'static mut String| context.replace("new %vertexes in {%placehodler}", "")
);
visitor_callback!(visitIsVBindCallback, Binding, |_p, _context| format!(
    "visitIsVBindCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIsGBindCallback, GraphBinding, |_p, _context| format!(
    "visitIsGBindCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitIsVNameCallback, Vertex, |_p, _context| format!(
    "visitIsVNameCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitNameWildcardCallback, Name, |_p, _context| format!(
    "visitNameWildcardCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitNameVVarCallback, Name, |_p, _context| format!(
    "visitNameVVarCallback{{ {} }}",
    INNER_PLACEHOLDER
));
visitor_callback!(visitNameGVarCallback, Name, |_p, _context| format!(
    "visitNameGVarCallback{{ {} }}",
    INNER_PLACEHOLDER
));

visitor_callback!(
    visitAttrCallback,
    Attr,
    |_p, _context: &'static mut String| {
        format!("visitAttrCallback {{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitAttrNameCallback,
    AttrName,
    |_p, _context: &'static mut String| {
        format!("visitAttrNameCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitAttrValCallback,
    AttrVal,
    |_p, _context: &'static mut String| {
        format!("visitAttrValCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGBindCallback,
    GraphBinding,
    |_p, _context: &'static mut String| {
        format!("visitGBindCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGEdgeAnonCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGEdgeAnonCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGEdgeNamedCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGEdgeNamedCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGNilCallback,
    Graph,
    |_p, _context: &'static mut String| { format!("visitGNilCallback{{ {} }}", INNER_PLACEHOLDER) }
);
visitor_callback!(
    visitGNominateCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGNominateCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGRuleAnonCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGRuleAnonCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGRuleNamedCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGRuleNamedCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGSubgraphCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGSubgraphCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGTensorCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGTensorCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitGVarCallback,
    Graph,
    |_p, _context: &'static mut String| { format!("visitGVarCallback{{ {} }}", INNER_PLACEHOLDER) }
);
visitor_callback!(
    visitGVertexCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitGVertexCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIdentCallback,
    Ident,
    |_p, _context: &'static mut String| {
        format!("visitIdentCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsAttrListCallback,
    Attr,
    |_p, _context: &'static mut String| {
        format!("visitIsAttrListCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsAttributeNameCallback,
    LVar,
    |_p, _context: &'static mut String| {
        format!("visitIsAttributeNameCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsAttributePairCallback,
    AttrName,
    |_p, _context: &'static mut String| {
        format!(
            "visitIsAttributePairCallback{{ {} }}{{ {} }}",
            INNER_PLACEHOLDER, INNER_PLACEHOLDER
        )
    }
);
visitor_callback!(
    visitIsAttributeValueCallback,
    LVar,
    |_p, _context: &'static mut String| {
        format!("visitIsAttributeValueCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsGEdgeAnonCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitIsGEdgeAnonCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsGEdgeNamedCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitIsGEdgeNamedCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsGNominateCallback,
    Graph,
    |_p, _context: &'static mut String| {
        format!("visitIsGNominateCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsNameGVarCallback,
    Name,
    |_p, _context: &'static mut String| {
        format!("visitIsNameGVarCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsNameQuoteGraphCallback,
    Name,
    |_p, _context: &'static mut String| {
        format!("visitIsNameQuoteGraphCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsNameQuoteVertexCallback,
    Name,
    |_p, _context: &'static mut String| {
        format!("visitIsNameQuoteVertexCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsNameVVarCallback,
    Name,
    |_p, _context: &'static mut String| {
        format!("visitIsNameVVarCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsNameWildcardCallback,
    Name,
    |_p, _context: &'static mut String| {
        format!("visitIsNameWildcardCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitIsEmptyAttrListCallback,
    ListAttr,
    |_p, _context: &'static mut String| {
        format!("visitIsEmptyAttrListCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitListAttrCallback,
    ListAttr,
    |_p, _context: &'static mut String| {
        format!("visitListAttrCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitListNameCallback,
    ListName,
    |_p, _context: &'static mut String| {
        format!("visitListNameCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
visitor_callback!(
    visitLVarCallback,
    LVar,
    |_p, _context: &'static mut String| { format!("visitLVarCallback{{ {} }}", INNER_PLACEHOLDER) }
);
visitor_callback!(
    visitUVarCallback,
    UVar,
    |_p, _context: &'static mut String| { format!("visitUVarCallback{{ {} }}", INNER_PLACEHOLDER) }
);
visitor_callback!(
    visitVBindCallback,
    Binding,
    |_p, _context: &'static mut String| {
        format!("visitVBindCallback{{ {} }}", INNER_PLACEHOLDER)
    }
);
