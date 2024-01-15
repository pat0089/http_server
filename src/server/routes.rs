use std::net::TcpStream;
use std::io;

type Handler = fn(&mut TcpStream) -> io::Result<()>;

/// A route that can be matched against a request
/// 
/// The path and method are used to match against the request
pub struct Route {
    pub path: String,
    pub method: String,
    handler: Handler,
}

impl Route {
    pub fn new(path: &str, method: &str, handler: Handler) -> Self {
        Self {
            path: path.to_string(),
            method: method.to_string(),
            handler,
        }
    }

    pub fn call(&self, stream: &mut TcpStream) -> io::Result<()> {
        (self.handler)(stream)
    }
}