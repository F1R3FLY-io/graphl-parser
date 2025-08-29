use std::{
    alloc::{Layout, alloc, dealloc},
    ffi::CStr,
};

#[unsafe(no_mangle)]
pub fn rust_alloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 16).unwrap();
    unsafe { alloc(layout) }
}

#[unsafe(no_mangle)]
pub fn rust_free(ptr: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, 16).unwrap();
    unsafe { dealloc(ptr, layout) };
}

#[unsafe(no_mangle)]
pub fn rust_panic(s: *const i8) {
    let str = unsafe { CStr::from_ptr(s) };
    panic!("{}", str.to_str().unwrap());
}
