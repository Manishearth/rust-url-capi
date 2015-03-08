#![feature(libc)]
#![feature(core)]

// warning: use of unstable library feature 'alloc': may be renamed
#![feature(alloc)]

extern crate url;
use url::{ Url, ParseError, UrlParser};

extern crate libc;
use libc::{c_void, c_char, size_t};

extern crate core;
use core::mem;

use std::str;

use url::urlutils::{UrlUtils, UrlUtilsWrapper};

#[allow(non_camel_case_types)]
pub type rusturl_ptr = *const libc::c_void;

mod cstring_utils;
pub use cstring_utils::*;

mod error_mapping;
use error_mapping::{ErrorCode};

#[no_mangle]
pub unsafe extern "C" fn rusturl_new(spec: *mut libc::c_char, len: size_t) -> rusturl_ptr {
  let slice = std::slice::from_raw_parts(spec as *const libc::c_uchar, len as usize);
  let url = Box::new(Url::parse(str::from_utf8(slice).ok().unwrap()).ok().unwrap());
  std::boxed::into_raw(url) as rusturl_ptr
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_free(urlptr: rusturl_ptr) {
  let url: Box<Url> = Box::from_raw(urlptr as *mut url::Url);
  drop(url);
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_spec(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);

  rust_cstring::new(&url.serialize())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_scheme(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);

  rust_cstring::new(&url.scheme)
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_username(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);
  match url.username() {
    Some(p) => rust_cstring::new(&p.to_string()),
    None => rust_cstring::null()
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_password(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);
  match url.password() {
    Some(p) => rust_cstring::new(&p.to_string()),
    None => rust_cstring::null()
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_host(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);

  let host = match url.host() {
    Some(h) => h,
    None => return rust_cstring::null()
  };

  rust_cstring::new(&host.serialize())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_port(urlptr: rusturl_ptr) -> i32 {
  let url: &Url = mem::transmute(urlptr);

  match url.port() {
    Some(port) => port as i32,
    None => -1
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_path(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);
  match url.serialize_path() {
    Some(s) => rust_cstring::new(&s),
    None => rust_cstring::null()
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_query(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);
  match url.lossy_percent_decode_query() {
    Some(s) => rust_cstring::new(&s),
    None => rust_cstring::null()
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_fragment(urlptr: rusturl_ptr) -> rust_cstring {
  let url: &Url = mem::transmute(urlptr);
  // TODO: fragment shouldn't be encoded
  match url.lossy_percent_decode_fragment() {
    Some(s) => rust_cstring::new(&s),
    None => rust_cstring::null()
  }
}


#[no_mangle]
pub unsafe extern "C" fn rusturl_set_scheme(urlptr: rusturl_ptr, scheme: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(scheme as *const libc::c_uchar, len as usize);

  let scheme_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_scheme(scheme_).error_code()
}


#[no_mangle]
pub unsafe extern "C" fn rusturl_set_username(urlptr: rusturl_ptr, username: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(username as *const libc::c_uchar, len as usize);

  let username_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_username(username_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_password(urlptr: rusturl_ptr, password: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(password as *const libc::c_uchar, len as usize);

  let password_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_password(password_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_host_and_port(urlptr: rusturl_ptr, host_and_port: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(host_and_port as *const libc::c_uchar, len as usize);

  let host_and_port_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_host_and_port(host_and_port_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_host(urlptr: rusturl_ptr, host: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(host as *const libc::c_uchar, len as usize);

  let hostname = match str::from_utf8(slice).ok() {
    Some(h) => h,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_host(hostname).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_port(urlptr: rusturl_ptr, port: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(port as *const libc::c_uchar, len as usize);

  let port_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_port(port_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_path(urlptr: rusturl_ptr, path: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(path as *const libc::c_uchar, len as usize);

  let path_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_path(path_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_query(urlptr: rusturl_ptr, query: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(query as *const libc::c_uchar, len as usize);

  let query_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_query(query_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_fragment(urlptr: rusturl_ptr, fragment: *mut libc::c_char, len: size_t) -> i32 {
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(fragment as *const libc::c_uchar, len as usize);

  let fragment_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &UrlParser::new()};
  wrapper.set_fragment(fragment_).error_code()
}
