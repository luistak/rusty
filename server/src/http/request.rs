use std::str;
use std::str::Utf8Error;
use std::error::Error;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Result as FmtResult, Formatter};

use super::method::{Method, MethodError};
use super::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'buffer_lifetime> {
  path: &'buffer_lifetime str,
  query_string: Option<QueryString<'buffer_lifetime>>,
  method: Method,
}

impl<'buffer_lifetime> Request<'buffer_lifetime> {
  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn method(&self) -> &Method {
    &self.method
  }

  pub fn query_string(&self) -> Option<&QueryString> {
    self.query_string.as_ref()
  }
}

impl<'buffer_lifetime> TryFrom<&'buffer_lifetime [u8]> for Request<'buffer_lifetime> {
  type Error = ParseError;

  // GET /search?name=abc&sort=1 HTTP/1.1\r\n
  fn try_from(buf: &'buffer_lifetime [u8]) -> Result<Self, Self::Error> {
    let request = str::from_utf8(buf)?;

    let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;

    let mut query_string = None;
    if let Some(i) = path.find('?') {
      query_string = Some(QueryString::from(&path[i + 1..]));
      path = &path[..i];
    }

    Ok(Self {
      path,
      query_string,
      method,
    })
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  for (index, char) in request.chars().enumerate() {
    if char == ' ' || char == '\r' {
      return Some((&request[..index], &request[index + 1..]))
    }
  }

  None
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "InvalidRequest",
      Self::InvalidEncoding => "InvalidEncoding",
      Self::InvalidProtocol => "InvalidProtocol",
      Self::InvalidMethod => "InvalidMethod",
    }
  }
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}
