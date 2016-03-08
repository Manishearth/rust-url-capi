extern crate url;
use url::{ Url, ParseError, UrlParser, SchemeType, ParseResult};

extern crate libc;
use libc::{c_void, c_char, size_t};


use std::mem;
use std::str;
use std::borrow::Borrow;

use url::urlutils::{UrlUtils, UrlUtilsWrapper};

use url::EncodingOverride;

#[repr(C)]
enum UrlSegmentFlags
{
    Scheme = 1 << 0,
    User = 1 << 1,
    Password = 1 << 2,
    Hostname = 1 << 3,
    Port = 1 << 4,
    Path = 1 << 5,
    Query = 1 << 6,
    Hash = 1 << 7
}

#[allow(non_camel_case_types)]
pub type rusturl_ptr = *const libc::c_void;

mod string_utils;
pub use string_utils::*;

mod error_mapping;
use error_mapping::*;

fn mapper(scheme: &str) -> SchemeType {
    match scheme {
        "file" => SchemeType::FileLike,
        "ftp" => SchemeType::Relative(21),
        "gopher" => SchemeType::Relative(70),
        "http" => SchemeType::Relative(80),
        "https" => SchemeType::Relative(443),
        "ws" => SchemeType::Relative(80),
        "wss" => SchemeType::Relative(443),
        "resource" => SchemeType::FileLike,
        "chrome" => SchemeType::FileLike,
        "jar" => SchemeType::FileLike,
        "wyciwyg" => SchemeType::FileLike,
        "app" => SchemeType::FileLike,
        "view-source" => SchemeType::FileLike,
        "moz-gio" => SchemeType::FileLike,
        "moz-icon" => SchemeType::FileLike,
        "rtsp" => SchemeType::Relative(443),
        "moz-anno" => SchemeType::Relative(443),
        "android" => SchemeType::Relative(443),
        _ => SchemeType::NonRelative,
    }
}

fn parser<'a>() -> UrlParser<'a> {
  fn silent_handler(_reason: ParseError) -> ParseResult<()> { Ok(()) }
  UrlParser {
    base_url: None,
    query_encoding_override: EncodingOverride::utf8(),
    error_handler: silent_handler,
    scheme_type_mapper: mapper,
  }
}


#[no_mangle]
pub unsafe extern "C" fn rusturl_new(spec: *mut libc::c_char, len: size_t) -> rusturl_ptr {
  let slice = std::slice::from_raw_parts(spec as *const libc::c_uchar, len as usize);
  let url_spec = match str::from_utf8(slice) {
    Ok(spec) => spec,
    Err(_) => return 0 as rusturl_ptr
  };

  let url = match parser().parse(url_spec) {
    Ok(url) => url,
    Err(_) => return 0 as rusturl_ptr
  };
  let url = Box::new(url);
  mem::transmute(url)
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_free(urlptr: rusturl_ptr) {
  if urlptr.is_null() {
    return ();
  }
  let url: Box<Url> = Box::from_raw(urlptr as *mut url::Url);
  drop(url);
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_part(urlptr: rusturl_ptr, flags: u32, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  let mut buffer: String = "".to_string();

  if flags & UrlSegmentFlags::Scheme as u32 != 0 {
    buffer.push_str(&url.scheme);
    if flags & !(UrlSegmentFlags::Scheme as u32) != 0 {
      buffer.push_str("://");
    }
  }

  if flags & (UrlSegmentFlags::User as u32) != 0 {
    if let Some(user) = url.username() {
      buffer.push_str(&user);
    }
  }

  if flags & (UrlSegmentFlags::Password as u32) != 0 {
    if let Some(password) = url.password() {
      buffer.push_str(":");
      buffer.push_str(&password);
    }
  }

  if flags & (UrlSegmentFlags::Hostname as u32) != 0 {
    if flags & (UrlSegmentFlags::User as u32) != 0 {
      if let Some(user) = url.username() {
        if user.len() != 0 {
          buffer.push_str("@");
        }
      }
    }

    if let Some(hostname) = url.host() {
      buffer.push_str(&hostname.serialize());
    }
  }

  if flags & (UrlSegmentFlags::Port as u32) != 0 {
    if let Some(port) = url.port() {
      if flags & (UrlSegmentFlags::Hostname as u32) != 0 {
        buffer.push_str(":");
      }

      buffer.push_str(&port.to_string());
    }
  }

  if flags & (UrlSegmentFlags::Path as u32) != 0 {
    if let Some(path) = url.serialize_path() {
      buffer.push_str(&path);
    }
  }

  if flags & (UrlSegmentFlags::Query as u32) != 0 {
    if flags & !(UrlSegmentFlags::Query as u32) != 0 {
      buffer.push_str("?");
    }
    if let Some(ref query) = url.query {
      buffer.push_str(&query);
    }
  }

  if flags & (UrlSegmentFlags::Hash as u32) != 0 {
    if flags & !(UrlSegmentFlags::Hash as u32) != 0 {
      buffer.push_str("#");
    }
    if let Some(ref fragment) = url.fragment {
      buffer.push_str(&fragment);
    }
  }

  cont.assign(&buffer)
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_spec(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  cont.assign(&url.serialize())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_scheme(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  cont.assign(&url.scheme)
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_username(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  match url.username() {
    Some(p) => cont.assign(&p.to_string()),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_password(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  match url.password() {
    Some(p) => cont.assign(&p.to_string()),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_host(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.host() {
    Some(h) => cont.assign(&h.serialize()),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_port(urlptr: rusturl_ptr) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.port() {
    Some(port) => port as i32,
    None => -1
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_path(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  match url.serialize_path() {
    Some(s) => cont.assign(&s),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_query(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  match url.query {
    Some(ref s) => cont.assign(&s),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_fragment(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.fragment {
    Some(ref fragment) => cont.assign(&fragment),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_has_fragment(urlptr: rusturl_ptr) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.fragment {
    Some(_) => return 1,
    None => return 0
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_scheme(urlptr: rusturl_ptr, scheme: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(scheme as *const libc::c_uchar, len as usize);

  let scheme_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_scheme(scheme_).error_code()
}


#[no_mangle]
pub unsafe extern "C" fn rusturl_set_username(urlptr: rusturl_ptr, username: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(username as *const libc::c_uchar, len as usize);

  let username_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_username(username_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_password(urlptr: rusturl_ptr, password: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(password as *const libc::c_uchar, len as usize);

  let password_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_password(password_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_host_and_port(urlptr: rusturl_ptr, host_and_port: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(host_and_port as *const libc::c_uchar, len as usize);

  let host_and_port_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_host_and_port(host_and_port_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_host(urlptr: rusturl_ptr, host: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(host as *const libc::c_uchar, len as usize);

  let hostname = match str::from_utf8(slice).ok() {
    Some(h) => h,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_host(hostname).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_port(urlptr: rusturl_ptr, port: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(port as *const libc::c_uchar, len as usize);

  let port_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_port(port_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_port_no(urlptr: rusturl_ptr, new_port: i32) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let scheme_type = parser().get_scheme_type(&url.scheme);
  match url.relative_scheme_data_mut() {
    Some(data) => {
        if scheme_type == SchemeType::FileLike {
            return ParseError::CannotSetPortWithFileLikeScheme.error_code();
        }
        match data.default_port {
          Some(def_port) => if new_port == def_port as i32 {
            data.port = None;
            return NSError::OK.error_code();
          },
          None => {}
        };
        if new_port > std::u16::MAX as i32 || new_port < 0 {
          data.port = None
        } else {
          data.port = Some(new_port as u16);
        }
        NSError::OK.error_code()
    },
    None => ParseError::CannotSetPortWithNonRelativeScheme.error_code()
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_path(urlptr: rusturl_ptr, path: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(path as *const libc::c_uchar, len as usize);

  let path_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_path(path_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_query(urlptr: rusturl_ptr, query: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(query as *const libc::c_uchar, len as usize);

  let query_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_query(query_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_fragment(urlptr: rusturl_ptr, fragment: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(fragment as *const libc::c_uchar, len as usize);

  let fragment_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return ParseError::InvalidCharacter.error_code() // utf-8 failed
  };

  let mut wrapper = UrlUtilsWrapper{ url: url, parser: &parser()};
  wrapper.set_fragment(fragment_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_resolve(urlptr: rusturl_ptr, resolve: *mut libc::c_char, len: size_t, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &mut Url = mem::transmute(urlptr);

    let slice = std::slice::from_raw_parts(resolve as *const libc::c_uchar, len as usize);

  let resolve_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return NSError::Failure.error_code()
  };

  match parser().base_url(&url).parse(resolve_).ok() {
    Some(u) => cont.assign(&u.serialize()),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_common_base_spec(urlptr1: rusturl_ptr, urlptr2: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr1.is_null() || urlptr2.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url1: &Url = mem::transmute(urlptr1);
  let url2: &Url = mem::transmute(urlptr2);

  if url1 == url2 {
    return cont.assign(&url1.serialize());
  }

  if url1.scheme != url2.scheme ||
     url1.host() != url2.host() ||
     url1.username() != url2.username() ||
     url1.password() != url2.password() ||
     url1.port() != url2.port() {
    return cont.set_size(0);
  }

  let data1 = match url1.relative_scheme_data() {
    Some(data) => data,
    None => return cont.set_size(0)
  };
  let data2 = match url2.relative_scheme_data() {
    Some(data) => data,
    None => return cont.set_size(0)
  };

  let min_path_len = std::cmp::min(data1.path.len(), data2.path.len());
  let mut matches = min_path_len;
  for i in 0..min_path_len {
    if data1.path[i] != data2.path[i] {
      matches = i;
      break;
    }
  }

  let mut url = url1.clone();
  url.query = None;
  url.fragment = None;
  match url.relative_scheme_data_mut() {
    Some(data) => {
      data.path.truncate(matches);
    }
    None => return cont.set_size(0)
  };

  cont.assign(&url.serialize())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_relative_spec(urlptr1: rusturl_ptr, urlptr2: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr1.is_null() || urlptr2.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url1: &Url = mem::transmute(urlptr1);
  let url2: &Url = mem::transmute(urlptr2);

  if url1 == url2 {
    return cont.set_size(0);
  }

  if url1.scheme != url2.scheme ||
     url1.host() != url2.host() ||
     url1.username() != url2.username() ||
     url1.password() != url2.password() ||
     url1.port() != url2.port() {
    return cont.assign(&url2.serialize());
  }

  let data1 = match url1.relative_scheme_data() {
    Some(data) => data,
    None => return cont.assign(&url2.serialize())
  };
  let data2 = match url2.relative_scheme_data() {
    Some(data) => data,
    None => return cont.assign(&url2.serialize())
  };

  // TODO: file:// on WIN?

  let min_path_len = std::cmp::min(data1.path.len(), data2.path.len());
  let mut matches = min_path_len;
  for i in 0..min_path_len {
    if data1.path[i] != data2.path[i] {
      matches = i;
      break;
    }
  }

  let mut buffer: String = "".to_string();
  for _ in matches..data1.path.len() {
    buffer = buffer + "../";
  }
  for i in matches..data2.path.len() {
    let buf = data2.path[i].to_string() + "/";
    buffer = buffer + buf.borrow();
  }

  return cont.assign(&buffer);
}

