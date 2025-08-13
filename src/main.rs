#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_nil() {
        let document = c"{0}";
        let result = show(CString::from(document)).unwrap();

        assert_eq!(result, "0 GNil".to_string());
    }

    #[test]
    fn print_nil() {
        let document = c"{0}";
        let result = print(CString::from(document)).unwrap();

        assert_eq!(result, "0 ".to_string());
    }
}
