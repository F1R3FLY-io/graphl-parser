use std::ffi::{CStr, CString};

use crate::bindings::{free_Graph, psGraph, showGraph};

/**
 * Show Graph
 */
pub fn show(
    document: impl Into<CString>,
) -> Result<std::string::String, std::ffi::IntoStringError> {
    unsafe {
        let graph = psGraph(document.into().as_ptr());
        let output_ptr = showGraph(graph);
        free_Graph(graph);

        CStr::from_ptr(output_ptr)
    }
    .to_owned()
    .into_string()
}

#[cfg(test)]
mod tests {

    use std::ffi::CString;

    use super::*;

    #[test]
    fn show_nil() {
        let document = c"{0}";
        let result = show(CString::from(document));

        assert_eq!(result, c"GNil".to_owned().into_string());
    }
}
