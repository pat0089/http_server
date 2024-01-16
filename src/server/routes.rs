use std::net::TcpStream;
use std::io;

type Handler = fn(&mut TcpStream) -> io::Result<()>;

/// A route that can be matched against a request
/// 
/// The path and method are used to match against the request
pub struct Route {
    path: String,
    method: String,
    handler: Handler,
}

impl Route {
    pub fn new(path: &str, method: &str, handler: Handler) -> Self {
        //TODO: check if path and method are valid otherwise return an error (non-empty path and valid method)
        Self {
            path: path.to_string(),
            method: method.to_string(),
            handler,
        }
    }

    pub fn call(&self, stream: &mut TcpStream) -> io::Result<()> {
        (self.handler)(stream)
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn method(&self) -> String {
        self.method.clone()
    }
}