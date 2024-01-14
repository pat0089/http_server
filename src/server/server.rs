use std::net::TcpListener;

use crate::server::handle_client::handle_client;
use crate::server::routes::Route;

pub struct Server {
    routes: Vec<Route>,
}

impl Server {
    pub fn new(routes: Vec<Route>) -> Self {
        Self { routes }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream, &self.routes).expect("Failed to handle client");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

}