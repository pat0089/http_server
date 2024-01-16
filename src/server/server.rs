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
        //if a route is named a file in an allowed directory, attempt to serve the file instead and ignore route function/functionality
        let mut filtered_routes = Vec::new();

        'route_loop: for route in routes {
            if let Some(last_segment) = route.path().split('/').last() {
                if last_segment.contains('.') && !last_segment.starts_with('.') {
                    // Check if the route is in an allowed directory
                    for dir in &directories {
                        if route.path().starts_with(&dir.path()) {
                            // This is a valid file path in an allowed directory, skip this route
                            continue 'route_loop;
                        }
                    }
                }
            }
            // If it's not a file path in an allowed directory, keep the route
            filtered_routes.push(route);
        }
    
        Self { routes: filtered_routes, directories }

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