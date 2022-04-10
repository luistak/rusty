use std::io::Read;
use std::net::TcpListener;
use crate::http::{Response, StatusCode, Request, ParseError};

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    print!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  address: String,
}

impl Server {
  // Like a static function
  pub fn new(address: String) -> Self {
    return Server { address };
  }

  // Methods accepts a first parameters
  pub fn run(self, mut handler: impl Handler) {
    println!("Listening on {}", self.address);

    let listener = TcpListener::bind(&self.address).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];

          match stream.read(&mut buffer) {
            Ok(_) => {
              // println!("Received a request: {}", String::from_utf8_lossy(&buffer));

              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(error) => handler.handle_bad_request(&error),
              };

              if let Err(e) = response.send(&mut stream) {
                print!("Failed to send request: {}", e);
              }
            },
            Err(error) => print!("Failed to read from connection: {}", error)
          }
        },
        Err(error) => print!("Failed to establish a connection: {}", error)
      }
    }
  }
}
