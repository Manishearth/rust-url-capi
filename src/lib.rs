extern crate url;
use url::{Url, Position};

extern crate libc;
use libc::size_t;


use std::mem;
use std::str;
use std::str::FromStr;
use std::convert::From;

#[allow(non_camel_case_types)]
pub type rusturl_ptr = *const libc::c_void;

mod string_utils;
pub use string_utils::*;

mod error_mapping;
use error_mapping::*;

#[no_mangle]
pub unsafe extern "C" fn rusturl_new(spec: *mut libc::c_char, len: size_t) -> rusturl_ptr {
  let slice = std::slice::from_raw_parts(spec as *const libc::c_uchar, len as usize);
  let url_spec = match str::from_utf8(slice) {
    Ok(spec) => spec,
    Err(_) => return 0 as rusturl_ptr
  };

  let url = match Url::parse(url_spec) {
    Ok(url) => url,
    Err(e) => {
      //println!("error: {:?} {:?}", e, url_spec);
      return 0 as rusturl_ptr;
    }
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
pub unsafe extern "C" fn rusturl_get_substring(urlptr: rusturl_ptr, from: u32, to: u32, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  cont.assign(&url[Position::from(from)..Position::from(to)])
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_spec(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  cont.assign(&url.as_str())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_scheme(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  cont.assign(&url.scheme())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_username(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  cont.assign(&url.username())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_password(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  match url.password() {
    Some(p) => cont.assign(p),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_host(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.host_str() {
    Some(host) => cont.assign(host),
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
  cont.assign(url.path())
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_query(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);
  match url.query() {
    Some(query) => cont.assign(query),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_get_fragment(urlptr: rusturl_ptr, cont: *mut libc::c_void) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.fragment() {
    Some(fragment) => cont.assign(fragment),
    None => cont.set_size(0)
  }
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_has_fragment(urlptr: rusturl_ptr) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let url: &Url = mem::transmute(urlptr);

  match url.fragment() {
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
    None => return Err(()).error_code() // utf-8 failed
  };

  url.set_scheme(scheme_).error_code()
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
    None => return Err(()).error_code() // utf-8 failed
  };

  url.set_username(username_).error_code()
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
    None => return Err(()).error_code() // utf-8 failed
  };

  if password_.len() == 0 {
    url.set_password(None).error_code()
  } else {
    url.set_password(Some(password_)).error_code()
  }
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
    None => return Err(()).error_code() // utf-8 failed
  };

  url::quirks::set_host(url, host_and_port_).error_code()
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
    None => return Err(()).error_code() // utf-8 failed
  };

  url.set_host(Some(hostname)).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_port(urlptr: rusturl_ptr, port: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(port as *const libc::c_uchar, len as usize);

  let port_ = match str::from_utf8(slice).ok() {
    Some(p) => if p.len() == 0 {
                  None
               } else {
                   if let Ok(newport) = u16::from_str(p) {
                       Some(newport)
                   } else {
                       return Err(()).error_code();
                   }
               },
    None => return Err(()).error_code() // utf-8 failed
  };

  url.set_port(port_).error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_port_no(urlptr: rusturl_ptr, new_port: i32) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  if new_port >= 0 && new_port <= 0xFFFF {
    url.set_port(Some(new_port as u16)).error_code()
  } else {
    url.set_port(None).error_code()
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
    None => return Err(()).error_code() // utf-8 failed
  };

  url.set_path(path_);
  (Ok(()) as Result<(),()>) .error_code()
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
    None => return Err(()).error_code() // utf-8 failed
  };

  if query_.len() > 0 {
    url.set_query(Some(query_));
  } else {
    url.set_query(None);
  }

  (Ok(()) as Result<(),()>) .error_code()
}

#[no_mangle]
pub unsafe extern "C" fn rusturl_set_fragment(urlptr: rusturl_ptr, fragment: *mut libc::c_char, len: size_t) -> i32 {
  if urlptr.is_null() {
    return NSError::InvalidArg.error_code();
  }
  let mut url: &mut Url = mem::transmute(urlptr);
  let slice = std::slice::from_raw_parts(fragment as *const libc::c_uchar, len as usize);

  if len == 0 {
    url.set_fragment(None);
    return (Ok(()) as Result<(),()>) .error_code();
  }

  let fragment_ = match str::from_utf8(slice).ok() {
    Some(p) => p,
    None => return Err(()).error_code() // utf-8 failed
  };

  url.set_fragment(Some(fragment_));

  (Ok(()) as Result<(),()>) .error_code()
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
    None => return Err(()).error_code()
  };

  match url.join(resolve_) {
    Ok(u) => { cont.assign(&u.as_str()); return (Ok(()) as Result<(),()>) .error_code(); },
    Err(e) => { cont.set_size(0); return e.error_code(); }
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
    return cont.assign(url1.as_str());
  }

  if url1.scheme() != url2.scheme() ||
     url1.host() != url2.host() ||
     url1.username() != url2.username() ||
     url1.password() != url2.password() ||
     url1.port() != url2.port() {
    return cont.set_size(0);
  }

  let mut common = url1[Position::BeforeScheme..Position::AfterPort].to_string();

  match (url1.path_segments(), url2.path_segments()) {
      (Some(mut seg1), Some(mut seg2)) => {
        loop {
          let part1 = seg1.next();
          let part2 = seg2.next();
          if part1 != part2 {
            break;
          }
          if part1 == None {
            break;
          }
          common.push('/');
          if let Some(part) = part1 {
            common.push_str(part);
          }
        }
      },
      _ => { common.push('/'); },
  }
  cont.assign(&common)
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

  if url1.scheme() != url2.scheme() ||
     url1.host() != url2.host() ||
     url1.username() != url2.username() ||
     url1.password() != url2.password() ||
     url1.port() != url2.port() {
    return cont.assign(url2.as_str());
  }

  let mut common = "".to_string();

  // TODO: windows skip leading /

  match (url1.path_segments(), url2.path_segments()) {
    (Some(mut seg1), Some(mut seg2)) => {
      loop {
        let part1 = seg1.next();
        let part2 = seg2.next();
        if part1 == part2 {
          continue;
        }

        while let Some(part) = seg1.next() {
          if part.len() > 0 {
            common.push_str("../");
          }
        }

        if let Some(part) = part2 {
          common.push_str(part);
        }
        while let Some(part) = seg2.next() {
          common.push('/');
          common.push_str(part);
        }

        common.push_str(&url2[Position::AfterPath..Position::AfterFragment]);
      }
    },
    _ => { return cont.assign(url2.as_str()); },
  }

  cont.assign(&common)
}

