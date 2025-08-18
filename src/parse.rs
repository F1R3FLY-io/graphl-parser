use std::{ffi::CString, sync::Mutex};

use crate::{Visitor, free_Graph, psGraph, visitGraph};

pub fn parse(document: impl Into<CString>) -> std::result::Result<String, String> {
    let STORAGE = Mutex::new(String::new());

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
        visitGraph(graph, &mut visitor);
        free_Graph(graph);
    };

    STORAGE.into_inner().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use std::{ffi::CString, str::FromStr, sync::Mutex};

    use crate::Visitor;

    use super::parse;

    #[test]
    fn test_parse() {
        let cstring = CString::from_str("{0}").unwrap();

        let rho_lang = parse(cstring).unwrap();

        assert_eq!(rho_lang, "");
    }

    #[test]
    fn test_visit_graph() {
        let cstring = CString::from_str("{0}").unwrap();
        let graph = unsafe { crate::psGraph(cstring.as_ptr()) };
        static RESULT: Mutex<String> = Mutex::new(String::new());

        unsafe extern "C" fn visitorCallback(_p: crate::Graph) {
            let prev_value = RESULT.lock().unwrap().clone();
            *RESULT.lock().unwrap() = format!("{prev_value}Gnil Called");
        }

        let mut visitor = Visitor {
            visitIsGNilCallback: Some(visitorCallback),
            ..Default::default()
        };

        unsafe { crate::visitGraph(graph, &mut visitor) };
        unsafe { crate::visitGraph(graph, &mut visitor) };

        assert_eq!(
            RESULT.lock().unwrap().as_str(),
            "Gnil CalledGnil Called".to_string()
        );
    }
}
