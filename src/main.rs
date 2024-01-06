use std::net::TcpListener;
mod http_builder;
mod uri_sanitizer;
mod request_validation;
mod responses;
mod client;

use client::handle_client;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
