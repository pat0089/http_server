use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{self, Write};
use crate::http_builder::write_http_response_header;
use crate::html_builder::Page;
use crate::http_builder::HttpStatus::RequestOk;
use crate::server::util::mime_types::MimeType::{*, self};

/// Responds 200 OK
pub fn respond_ok(stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
    if !params.is_empty() {
        return respond_ok_with_body_and_type(stream, "This should never not be empty, contact system admin", PlainText);
    }

    let mut response  = Page::new();
    let hello = "Hello, World!";
    response.add_title(hello);
    response.add_script(JavaScript, Some("hello_world.js"), None);
    response.add_style(None, Some("* { font-family: monospace; }"));

    response.add_heading(1, hello);
    response.add_break();
    response.add_paragraph(hello);

    response.add_hyperlink("WebGL Demo", "/webgl");

    respond_ok_with_body_and_type(stream, response.to_string().as_str(), Html)
}

/// Responds 200 OK 
/// 
/// helper function to write a response with a body
pub fn respond_ok_with_body_and_type(stream: &mut TcpStream, body: &str, content_type: MimeType) -> io::Result<()> {
    let header = write_http_response_header(RequestOk, Some(content_type), Some(body.len() as u64));
    let response = format!("{}{}", header, body);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

pub fn respond_ok_with_body_bytes_and_type(stream: &mut TcpStream, body: &[u8], content_type: MimeType) -> io::Result<()> {
    let header = write_http_response_header(RequestOk, Some(content_type), Some(body.len() as u64));
    let response = vec![header.as_bytes(), body].concat();
    stream.write_all(&response)?;
    stream.flush()?;
    Ok(())
}

pub fn respond_ok_memes(stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
    let _ = params;
    return respond_ok_with_body_and_type(stream, "\"memes\" [\n\t\"meme\"\n]", Json);
}

pub fn respond_ok_id(stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
    let id = params.get("id").unwrap();
    return respond_ok_with_body_and_type(stream, format!("Your id is: {}", id).as_str(), PlainText);
}

pub fn respond_ok_abxy(stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
    let b = params.get("b").unwrap();
    let y = params.get("y").unwrap();
    return respond_ok_with_body_and_type(stream, format!("{} {}", b, y).as_str(), PlainText)
}
