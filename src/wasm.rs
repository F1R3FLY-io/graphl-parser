use std::alloc::{Layout, LayoutError};
use std::ffi::{CStr, c_void};
use std::{mem, ptr};

use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Copy)]
struct Header {
    allocation_layout: Layout,
}

impl Header {
    fn new(size: usize) -> Result<Self, LayoutError> {
        Layout::from_size_align(size, mem::align_of::<usize>())
            .map(|allocation_layout| Self { allocation_layout })
    }

    unsafe fn new_from_ptr(ptr: *mut u8) -> Self {
        let header_layout = Layout::new::<Self>();
        let header_ptr = unsafe { ptr.sub(header_layout.size()) } as *mut Header;
        unsafe { *header_ptr }
    }

    fn combined_layout(&self) -> Result<(Layout, usize), LayoutError> {
        let header_layout = Layout::new::<Self>();
        header_layout.extend(self.allocation_layout)
    }

    unsafe fn allocate(self, zeroed: bool) -> Result<*mut u8, LayoutError> {
        let (combined_layout, offset) = self.combined_layout()?;

        let ptr = if zeroed {
            unsafe { std::alloc::alloc_zeroed(combined_layout) }
        } else {
            unsafe { std::alloc::alloc(combined_layout) }
        };
        if ptr.is_null() {
            return Ok(ptr::null_mut());
        }

        let header_ptr = ptr as *mut Header;
        unsafe {
            header_ptr.write(self);
            Ok(ptr.add(offset))
        }
    }

    unsafe fn deallocate(ptr: *mut u8) {
        let header_layout = Layout::new::<Self>();
        let header = unsafe { Self::new_from_ptr(ptr) };
        let combined = header.combined_layout().unwrap().0;

        unsafe { std::alloc::dealloc(ptr.sub(header_layout.size()), combined) }
    }

    unsafe fn reallocate(ptr: *mut u8, new_size: usize) -> Result<*mut u8, LayoutError> {
        let header_layout = Layout::new::<Self>();
        let old_header = unsafe { Self::new_from_ptr(ptr) };
        let old_combined_layout = old_header.combined_layout().unwrap().0;

        let new_header = Self::new(new_size)?;
        let (new_combined_layout, new_offset) = new_header.combined_layout()?;

        let new_ptr = unsafe {
            std::alloc::realloc(
                ptr.sub(header_layout.size()),
                old_combined_layout,
                new_combined_layout.size(),
            )
        };

        if new_ptr.is_null() {
            return Ok(ptr::null_mut());
        }

        let header_ptr = new_ptr as *mut Header;
        unsafe {
            header_ptr.write(new_header);
            Ok(new_ptr.add(new_offset))
        }
    }
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    if size == 0 {
        return ptr::null_mut();
    }

    Header::new(size)
        .and_then(|header| unsafe { header.allocate(false) })
        .unwrap_or_default() as _
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    unsafe { Header::deallocate(ptr as _) }
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() {
        return unsafe { self::malloc(new_size) };
    }

    if new_size == 0 {
        unsafe { self::free(ptr) };
        return ptr::null_mut();
    }

    unsafe { Header::reallocate(ptr as _, new_size) }.unwrap_or_default() as _
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn calloc(num: usize, size: usize) -> *mut c_void {
    let total_size = match num.checked_mul(size) {
        Some(total_size) => total_size,
        None => return ptr::null_mut(),
    };

    if total_size == 0 {
        return ptr::null_mut();
    }

    Header::new(total_size)
        .and_then(|header| unsafe { header.allocate(true) })
        .unwrap_or_default() as _
}

#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn rust_panic(prefix: *const i8, s: *const i8) {
    let prefix = unsafe { CStr::from_ptr(prefix) };
    let s = unsafe { CStr::from_ptr(s) };
    panic!("{}{}", prefix.to_str().unwrap(), s.to_str().unwrap())
}

#[wasm_bindgen(start)]
fn init_panic_hook() {
    console_error_panic_hook::set_once()
}
