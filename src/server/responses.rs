use std::net::TcpStream;
use std::io::{self, Write};
use crate::http_builder::write_http_response_header;
use crate::html_builder::Page;
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

    let mut response  = Page::new();
    let hello = "Hello, World!";
    response.add_title(hello);
    response.add_script(Javascript, Some("hello_world.js"), None);
    response.add_style(None, Some("* { font-family: monospace; }"));

    response.add_heading(1, hello);
    response.add_break();
    response.add_paragraph(hello);

    respond_ok_with_body_and_type(stream, response.to_string().as_str(), Html)
}

pub fn respond_ok_with_body_and_type(stream: &mut TcpStream, body: &str, content_type: MimeType) -> io::Result<()> {
    let header = write_http_response_header(RequestOk, Some(content_type), Some(body.len() as u64));
    let response = format!("{}{}", header, body);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}