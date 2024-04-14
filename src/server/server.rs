use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::server::handle_client::handle_client;
use crate::server::routes::Route;
use crate::server::directories::Directory;

pub struct Server {
    routes: Arc<Vec<Route>>,
    directories: Arc<Vec<Directory>>,
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
    
        Self { routes: Arc::new(filtered_routes), directories: Arc::new(directories) }

    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let rs = self.routes.clone();
                    let ds = self.directories.clone();

                    thread::spawn(move || {
                        handle_client(stream, &rs, &ds).unwrap();
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

}