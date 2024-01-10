use std::net::TcpListener;

use crate::server::handle_client::handle_client;

pub fn run() {
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