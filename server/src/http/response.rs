use std::io::{Result as IoResult, Write};

use super::StatusCode;

type Body = Option<String>;

pub struct Response {
  body: Body,
  status_code: StatusCode,
}

impl Response {
  pub fn new(status_code: StatusCode, body: Body) -> Self {
    Response { body, status_code }
  }

  pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
    let body = match &self.body {
      Some(b) => b,
      None => "",
    };

    write!(
      stream,
      "Http/1.1 {} {}\r\n\r\n{}",
      self.status_code,
      self.status_code.reason_phrase(),
      body
    )
  }
}
