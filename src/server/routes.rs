use std::collections::HashMap;
use std::net::TcpStream;
use std::io;

use crate::http_builder::HttpMethod;

type Handler = fn(&mut TcpStream, HashMap<String, String>) -> io::Result<()>;

/// A route that can be matched against a request
/// 
/// The path and method are used to match against the request
#[derive(Debug)]
pub struct Route {
    path: String,
    method: HttpMethod,
    handler: Handler,
}

impl Route {
    pub fn new(path: &str, method: HttpMethod, handler: Handler) -> Self {
        //TODO: check if path and method are valid otherwise return an error (non-empty path and valid method)
        if path.is_empty() {
            panic!("Invalid route: empty path");
        }
        Self {
            path: path.to_string(),
            method,
            handler,
        }
    }

    pub fn call(&self, stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
        (self.handler)(stream, params)
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn method(&self) -> HttpMethod {
        self.method.clone()
    }
}