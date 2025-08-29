use std::ffi::{CStr, CString};

use crate::bindings::{free_Graph, printGraph, psGraph};

pub fn print(
    document: impl Into<CString>,
) -> Result<std::string::String, std::ffi::IntoStringError> {
    unsafe {
        let graph = psGraph(document.into().as_ptr());
        let output_ptr = printGraph(graph);
        free_Graph(graph);

        CStr::from_ptr(output_ptr)
    }
    .to_owned()
    .into_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_nil() {
        let document = c"{0}";
        let result = print(CString::from(document));

        assert_eq!(result, c"0 ".to_owned().into_string());
    }
}
