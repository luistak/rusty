#![allow(dead_code)]

use std::env;
use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    /*
        let string = String::from("127.0.0.1:8080");
        // Never slice strings based on index, because in rust it focus on bite length
        let string_slice = &string[string.len() - 4..];
        let string_borrow: &str = &string;
        let string_literal = "1234";

        dbg!(&string, string_slice, string_borrow, string_literal);
    */
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());

    server.run(WebsiteHandler::new(public_path));
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/