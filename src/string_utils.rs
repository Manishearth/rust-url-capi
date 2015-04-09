extern crate libc;
use libc::{c_void, c_char, size_t};

extern crate std;
use std::ptr;

use error_mapping::*;


#[repr(C)]
pub struct string_container  {
  pub container: *mut libc::c_void,
  pub fn_set_size: Option<extern "C" fn(user: *mut libc::c_void, size: size_t)>,
  pub fn_get_buffer: Option<extern "C" fn(user: *mut c_void) -> *mut libc::c_char>,
}

pub trait StringContainer {
  fn set_size(&self, size_t) -> i32;
  fn get_buffer(&self) -> *mut libc::c_char;
  fn copy_String_to(&self, content: &String) -> i32;
}

impl StringContainer for *mut string_container {
  fn set_size(&self, size: size_t) -> i32 {
    if (*self).is_null() {
      return NSError::InvalidArg.error_code();
    }
    unsafe {
      match (**self).fn_set_size {
        Some(f) => f((**self).container, size),
        None => return NSError::InvalidArg.error_code()
      }
    return NSError::OK.error_code();
    }
  }

  fn get_buffer(&self) -> *mut libc::c_char {
    unsafe {
      match (**self).fn_get_buffer {
        Some(f) => return f((**self).container),
        None => return 0 as *mut libc::c_char
      }
    }
  }

  fn copy_String_to(&self, content: &String) -> i32 {
    if (*self).is_null() {
      return NSError::InvalidArg.error_code();
    }

    unsafe {
      let slice = content.as_bytes();

      match (**self).fn_set_size {
        Some(f) => f((**self).container, slice.len() as u64),
        None => return NSError::InvalidArg.error_code()
      }

      let buf = match (**self).fn_get_buffer {
        Some(f) => f((**self).container),
        None => 0 as *mut libc::c_char
      };

      if buf.is_null() {
        return NSError::Failure.error_code();
      }

      ptr::copy(slice.as_ptr(), buf as *mut u8, content.len());
    }

    NSError::OK.error_code()
  }
}
