use url::ParseError;

pub trait ErrorCode {
  fn error_code(&self) -> i32;
}

impl ErrorCode for ParseError {
  #[allow(overflowing_literals)]
  fn error_code(&self) -> i32 {
    0x804b000a // NS_ERROR_MALFORMED_URI
  }
}


impl ErrorCode for Result<(), ParseError> {
  fn error_code(&self) -> i32 {
    match *self {
      Ok(()) => 0,
      Err(err) => err.error_code(),
    }
  }
}


impl ErrorCode for Result<(), ()> {
  fn error_code(&self) -> i32 {
    match *self {
      Ok(()) => 0,
      Err(_) => -1,
    }
  }
}

pub enum NSError {
  OK,
  InvalidArg,
  Failure,
}

impl ErrorCode for NSError {
  #[allow(overflowing_literals)]
  fn error_code(&self) -> i32 {
    match *self {
      NSError::OK => 0,
      NSError::InvalidArg => 0x80070057,
      NSError::Failure => 0x80004005
    }
  }
}