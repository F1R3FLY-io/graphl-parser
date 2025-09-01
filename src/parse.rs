use std::{
    ffi::{CString, c_void},
    str::FromStr,
};

use crate::{Visitor, context::INNER_PLACEHOLDER, free_Graph, psGraph};
use crate::{visitGraph, visitors::*};

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

    let contract = format!("{{ {} }}", INNER_PLACEHOLDER);

    let rholang_representation = Box::new(contract);
    let rholang_representation_ptr = Box::into_raw(rholang_representation);

    unsafe {
        let graph = psGraph(ptr);
        if graph.is_null() {
            return Err("psGraph returned null".to_string());
        };

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
    use std::str::FromStr;

    use super::parse;

    #[test]
    fn test_parse_empty_graph() {
        let statement = String::from_str("0").unwrap();
        let result = parse(statement).unwrap();

        assert_eq!(
            result,
            "{ visitAttrCallback { visitIsGBindCallback{ %inner } } }"
        );
    }
}
