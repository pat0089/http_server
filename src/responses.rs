use std::net::TcpStream;
use std::io::{self, Write};

//400 Bad Request
pub fn respond_bad_request(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nError: {}\r\n", err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//403 Forbidden
pub fn respond_forbidden(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let response = format!("HTTP/1.1 403 Forbidden\r\nContent-Type: text/plain\r\n\r\nError: {}\r\n", err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//404 Not Found
pub fn respond_not_found(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nError: {}\r\n", err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//200 OK
pub fn respond_basic_ok(stream: &mut TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//200 OK, but better
pub fn respond_ok(stream: &mut TcpStream) -> io::Result<()> {
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><head><script type=\'text/javascript\'>console.log('Hello, world!');</script><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>\r\n";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}