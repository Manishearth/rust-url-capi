extern crate libc;
use libc::{c_void, c_char, size_t};

extern crate std;
use std::ptr;

use error_mapping::*;

extern "C" {
  fn c_fn_set_size(user: *mut libc::c_void, size: size_t) -> i32;
  fn c_fn_get_buffer(user: *mut c_void) -> *mut libc::c_char;
}

pub trait StringContainer {
  fn set_size(&self, size_t) -> i32;
  fn get_buffer(&self) -> *mut libc::c_char;
  fn assign(&self, content: &String) -> i32;
}

impl StringContainer for *mut libc::c_void {
  fn set_size(&self, size: size_t) -> i32 {
    if (*self).is_null() {
      return NSError::InvalidArg.error_code();
    }
    unsafe {
      c_fn_set_size(*self, size as u64);
    }

    return NSError::OK.error_code();
  }
  fn get_buffer(&self) -> *mut libc::c_char {
    if (*self).is_null() {
      return 0 as *mut libc::c_char;
    }
    unsafe {
      c_fn_get_buffer(*self)
    }
  }
  fn assign(&self, content: &String) -> i32 {
    if (*self).is_null() {
      return NSError::InvalidArg.error_code();
    }

    unsafe {
      let slice = content.as_bytes();
      c_fn_set_size(*self, slice.len() as u64);
      let buf = c_fn_get_buffer(*self);
      if buf.is_null() {
        return NSError::Failure.error_code();
      }

      ptr::copy(slice.as_ptr(), buf as *mut u8, content.len());
    }

    NSError::OK.error_code()
  }
}
