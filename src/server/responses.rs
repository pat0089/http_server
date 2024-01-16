use std::net::TcpStream;
use std::io::{self, Write};
use crate::http_builder::write_http_response_header;
use crate::html_builder::{ write_html, write_head, write_body, write_title, write_script, write_header, write_attribute, write_style };
use crate::http_builder::HttpStatus::{RequestOk, BadRequest, Forbidden, NotFound, InternalServerError};
use crate::server::util::mime_types::MimeType::{*, self};

//400 Bad Request
pub fn respond_bad_request(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(BadRequest, Some(PlainText), None);
    let response = format!("{}Error: {}\r\n", &header, &err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//403 Forbidden
pub fn respond_forbidden(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(Forbidden, Some(PlainText), None);
    let response = format!("{}Error: {}\r\n", &header, &err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//404 Not Found
pub fn respond_not_found(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(NotFound, Some(PlainText), None);
    let response = format!("{}Error: {}\r\n", header, err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

//500 Internal Server Error
pub fn respond_internal_server_error(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let header = write_http_response_header(InternalServerError, Some(PlainText), None);
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

    let response_body = write_html(Some(&format!("{head}\n{body}")));

    respond_ok_with_body_and_type(stream, &response_body, Html)
}

pub fn respond_ok_with_body_and_type(stream: &mut TcpStream, body: &str, content_type: MimeType) -> io::Result<()> {
    let header = write_http_response_header(RequestOk, Some(content_type), Some(body.len() as u64));
    let response = format!("{}{}", header, body);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}