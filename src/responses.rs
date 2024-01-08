use std::net::TcpStream;
use std::io::{self, Write};
use crate::http_builder::write_http_response_header;
use crate::html_builder::{ write_html, write_head, write_body, write_title, write_script, write_header, write_attribute, write_style };

//HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain
pub fn respond_bad_request(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(400, "Bad Request", "text/plain");
    let response = format!("{}Error: {}\r\n", &header, &err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//403 Forbidden
pub fn respond_forbidden(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(403, "Forbidden", "text/plain");
    let response = format!("{}Error: {}\r\n", &header, &err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//404 Not Found
pub fn respond_not_found(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(404, "Not Found", "text/plain");
    let response = format!("{}Error: {}\r\n", header, err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//500 Internal Server Error
pub fn respond_internal_server_error(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(500, "Internal Server Error", "text/plain");
    let response = format!("{}Error: {}\r\n", header, err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//200 OK
pub fn respond_ok(stream: &mut TcpStream) -> io::Result<()> {
    let title = write_title(Some("Hello, world!"));
    let script = write_script(None, Some(&write_attribute("src", "hello_world.js")));
    let style = write_style(Some("* { font-family: monospace; }"));

    let header = write_header(1, Some("Hello, world!"));
    
    let body = write_body(Some(&header));
    let head = write_head(Some(&format!("{title}\n{script}\n{style}")));

    let html = write_html(Some(&format!("{head}\n{body}")));

    let http_header = write_http_response_header(200, "OK", "text/html");

    let response = format!(
        "{}{}",
        http_header,
        html
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

pub fn respond_ok_with_body(stream: &mut TcpStream, body: &str) -> io::Result<()> {
    let header = write_http_response_header(200, "OK", "text/plain");
    let response = format!("{}{}", header, body);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}