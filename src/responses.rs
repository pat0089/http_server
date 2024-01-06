use std::net::TcpStream;
use std::io::{self, Write};
use crate::http_builder::{ write_html, write_head, write_body, write_title, write_script, write_header, write_paragraph, write_style };

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
    let title = write_title(Some("Hello, world!"));
    let script = write_script(Some("console.log(\'Hello, world!\');"));
    
    let header = write_header(1, Some("Hello, world!"));
    
    let body = write_body(Some(&header));
    let head = write_head(Some(&format!("{title}\n{script}")));

    let html = write_html(Some(&format!("{head}\n{body}")));

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", 
        html
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}