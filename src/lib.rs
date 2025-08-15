#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

impl ToString for Visitor {
    fn to_string(&self) -> std::string::String {
        return "Hello, World!".to_string();
    }
}

mod parse;
mod print;
mod show;

pub use parse::*;

pub mod utils {
    pub use crate::print::*;
    pub use crate::show::*;
}
