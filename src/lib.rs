#![feature(libc)]
#![feature(core)]

extern crate url;
use url::{ Url, ParseResult, UrlParser};

extern crate libc;
use libc::{c_void, c_char, size_t};

extern crate core;
use core::mem;

use std::str;
use std::ffi::CString;

use url::urlutils::{UrlUtilsWrapper, UrlUtils};

#[allow(non_camel_case_types)]
pub type rusturl_ptr = *const ();

#[repr(C)]
pub struct url_part  {
  cstring_ptr: *const libc::c_void,
  content: *const libc::c_char,
  size: size_t,
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_new(spec: *mut libc::c_char, len: size_t) -> rusturl_ptr {
  let slice = std::slice::from_raw_parts(spec as *const libc::c_uchar, len as usize);
  let url = Box::new(Url::parse(str::from_utf8(slice).ok().unwrap()).ok().unwrap());
  mem::transmute(url)
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_free(urlptr: rusturl_ptr) {
  let url: Box<Url> = mem::transmute(urlptr);
  drop(url);
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_host(urlptr: rusturl_ptr) -> url_part {
  let url: &Url = mem::transmute(urlptr);

  let cstring_box = Box::new( CString::from_slice(url.host().unwrap().serialize().as_bytes()) );
  url_part{content: cstring_box.as_ptr(), size: 0, cstring_ptr: mem::transmute(cstring_box)}
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_host(urlptr: rusturl_ptr, host: *mut libc::c_char, len: size_t) {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(host as *const libc::c_uchar, len as usize);

  let mut _wrapper = &mut UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};

  let hostname = str::from_utf8(slice).ok().unwrap();
  _wrapper.set_host(hostname); // TODO: use result
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_free_part(part: url_part) {
  let boxy: Box<CString> = mem::transmute(part.cstring_ptr);
  drop(boxy);
}

#[test]
fn it_works() {
}
