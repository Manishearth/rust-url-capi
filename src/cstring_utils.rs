extern crate libc;
use libc::{c_void, c_char, size_t};

extern crate std;
use std::boxed::*;
use std::ffi::CString;
use std::mem;

#[repr(C)]
pub struct rust_cstring  {
  raw_ptr: *const libc::c_void,
  data: *const libc::c_char,
  len: size_t,
}

impl rust_cstring {
  // NULL string representation
  pub fn null() -> rust_cstring {
    rust_cstring {
      raw_ptr: 0 as *const libc::c_void,
      data: 0 as *const libc::c_char,
      len: 0,
    }
  }

  // Allocates a new box for the given string and returns a rust_cstring
  // containing the given pointer.
  // C programs can use .data field to access the string
  pub unsafe fn new(string: &String) -> rust_cstring {
    let cstring_box : Box<CString> = Box::new( CString::new(string.as_bytes()).unwrap() );
    rust_cstring{ data: cstring_box.as_ptr(), len: string.len() as u64, raw_ptr: std::boxed::into_raw(cstring_box) as *const libc::c_void}
  }
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_cstring(part: rust_cstring) {
  let boxy: Box<CString> = mem::transmute(part.raw_ptr);
  drop(boxy);
}
