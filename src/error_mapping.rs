use url::{ ParseResult, ParseError};

pub trait ErrorCode {
  fn error_code(&self) -> i32;
}

impl<T> ErrorCode for ParseResult<T> {
  fn error_code(&self) -> i32 {
    match *self {
      Ok(_)                                              =>   0,
      Err(error)                                         =>  error.error_code(),
    }
  }
}

impl ErrorCode for ParseError {
  fn error_code(&self) -> i32 {
    match *self {
      ParseError::EmptyHost                              =>  -1,
      ParseError::InvalidScheme                          =>  -2,
      ParseError::InvalidPort                            =>  -3,
      ParseError::InvalidIpv4Address                     =>  -4,
      ParseError::InvalidIpv6Address                     =>  -5,
      ParseError::InvalidDomainCharacter                 =>  -6,
      ParseError::InvalidCharacter                       =>  -7,
      ParseError::InvalidBackslash                       =>  -8,
      ParseError::InvalidPercentEncoded                  =>  -9,
      ParseError::InvalidAtSymbolInUser                  => -10,
      ParseError::ExpectedTwoSlashes                     => -11,
      ParseError::ExpectedInitialSlash                   => -12,
      ParseError::NonUrlCodePoint                        => -13,
      ParseError::RelativeUrlWithScheme                  => -14,
      ParseError::RelativeUrlWithoutBase                 => -15,
      ParseError::RelativeUrlWithNonRelativeBase         => -16,
      ParseError::NonAsciiDomainsNotSupportedYet         => -17,
      ParseError::CannotSetJavascriptFragment            => -18,
      ParseError::CannotSetPortWithFileLikeScheme        => -19,
      ParseError::CannotSetUsernameWithNonRelativeScheme => -20,
      ParseError::CannotSetPasswordWithNonRelativeScheme => -21,
      ParseError::CannotSetHostPortWithNonRelativeScheme => -22,
      ParseError::CannotSetHostWithNonRelativeScheme     => -23,
      ParseError::CannotSetPortWithNonRelativeScheme     => -24,
      ParseError::CannotSetPathWithNonRelativeScheme     => -25,
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