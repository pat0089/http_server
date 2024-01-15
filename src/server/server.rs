use std::net::TcpListener;

use crate::server::handle_client::handle_client;
use crate::server::routes::Route;
use crate::server::directories::Directory;

pub struct Server {
    routes: Vec<Route>,
    directories: Vec<Directory>,
}

impl Server {
    pub fn new(routes: Vec<Route>, directories: Vec<Directory>) -> Self {
        Self { routes, directories }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream, &self.routes, &self.directories).expect("Failed to handle client");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

}