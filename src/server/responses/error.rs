use std::{io::{self, Write}, net::TcpStream};

use crate::http_builder::{write_http_response_header, HttpStatus, HttpStatus::{BadRequest, Forbidden, NotFound, InternalServerError}};
use crate::server::util::mime_types::MimeType::PlainText;


/// Responds with an error determined by status
/// 
/// helper function to write an error response with a body
pub fn respond_error_with_body_and_status(stream: &mut TcpStream, body: &str, status: HttpStatus) -> io::Result<()> {
    let header = write_http_response_header(status, Some(PlainText), None);
    let response = format!("{}Error - {}\r\n", header, body);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

/// 400 Bad Request
pub fn respond_bad_request(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    respond_error_with_body_and_status(stream, err, BadRequest)
}

/// 403 Forbidden
pub fn respond_forbidden(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    respond_error_with_body_and_status(stream, err, Forbidden)
}

/// responds 404 Not Found
pub fn respond_not_found(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    respond_error_with_body_and_status(stream, err, NotFound)
}

/// Responds 500 Internal Server Error
pub fn respond_internal_server_error(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    respond_error_with_body_and_status(stream, err, InternalServerError)
}