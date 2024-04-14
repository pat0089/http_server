use std::fs::read_to_string;
use std::net::TcpStream;
use std::io::{self, Read};
use std::str::FromStr;
use crate::server::util::mime_types::from_file_extension;
use crate::server::util::request_validation::{ validate_header, validate_request_line };
use crate::server::responses::{ respond_bad_request, respond_not_found, respond_forbidden, respond_internal_server_error, respond_ok_with_body_and_type };
use crate::http_builder::{HttpRequest, HttpRequestLine, HttpMethod, HttpHeader};
use crate::server::util::uri::get_file_extension;
use crate::server::routes::Route;
use crate::server::directories::{ Directory, directory_is_first_level };
use crate::server::util::externals::ExternalRequest;

pub fn read_in_request(stream: &mut TcpStream) -> io::Result<String> {

    let mut buffer = Vec::new();
    let mut local_buf = [0; 1024];

    // Read the request
    loop {
        let size = stream.read(&mut local_buf)?;
        if size == 0 {
            break;
        }
        buffer.extend_from_slice(&local_buf[..size]);

        // Check if we have received double CRLF, indicating the end of the header of the request
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn handle_file_case(mut stream: &mut TcpStream, path: &str) -> io::Result<()> {
    let path = path.chars().skip(1).collect::<String>();
    let file_contents = read_to_string(&path);
    match file_contents {
                Ok(contents) => {
                    return respond_ok_with_body_and_type(&mut stream, &contents, 
                        from_file_extension(
                            &get_file_extension(&path)
                        )
                    );
                }
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                    // File not found, send 404 response.
                    return respond_not_found(&mut stream, "File Not Found")
                }
                Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                    // Permission denied, send 403 response.
                    return respond_forbidden(&mut stream, "Forbidden, Access Denied")
                }
                Err(_) => {
                    //if the path points to nothing, return 404
                    return respond_internal_server_error(&mut stream, "Something went wrong! Contact the server administrator.")
                    //add logic here later for 403 Forbidden
                }
            }
}

pub fn handle_client(mut stream: TcpStream, routes: &[Route], directories: &[Directory]) -> io::Result<()> {

    let request_str = read_in_request(&mut stream).expect("Failed to read request");

    let mut header_lines: Vec<&str> = request_str.lines().collect();

    if header_lines.is_empty() {
        return respond_bad_request(&mut stream, "Empty request");
    }
    // Parse the request header
    let request_line = header_lines[0];
    header_lines.remove(0);

    let mut headers = Vec::new();
    for header in &header_lines {
        if !header.is_empty() {
            headers.push(HttpHeader::from_str(header).expect("Invalid header format"));            
        }
    }


    let mut request_method_and_path : Vec<&str> = request_line.splitn(3, ' ').collect();
    request_method_and_path.pop();
    let mut request = HttpRequest::new(
        HttpRequestLine::new(
            HttpMethod::from_str(request_method_and_path[0]).unwrap_or(HttpMethod::GET),
            request_method_and_path[1]
        )
    );

    for header in headers {
        request.add_header(header);
    }

    // Validate the request line
    if let Err(err) = validate_request_line(request_line) {
        return respond_bad_request(&mut stream, &err);
    }

    // Validate the header
    for line in header_lines {
        if let Err(err) = validate_header(line) {
            return respond_bad_request(&mut stream, &err);
        }
    }

    //google.com External Request
    let mut request = HttpRequest::new(
        HttpRequestLine::new(HttpMethod::GET, "/"),
    );
    request.add_header(HttpHeader::Host("google.com".to_string()));
    request.add_header(HttpHeader::Accept(vec![crate::server::util::mime_types::MimeType::Html]));
    request.add_header(HttpHeader::AcceptLanguage("en-US".to_string()));
    request.add_header(HttpHeader::Connection(true));
    let external_request = ExternalRequest::new(
        request.clone()
    );
    
    println!("sending basic GET request to {}", request.path());
    let external_response = external_request.send();
    if let Ok(external_response) = external_response {
        return respond_ok_with_body_and_type(&mut stream, &external_response.raw_response, crate::server::util::mime_types::MimeType::Html);
    }

    for route in routes {
        if request_line.starts_with(&route.method()) && request.path() == route.path() {
            return route.call(&mut stream);
        }
    }

    for directory in directories {
        if request.path().starts_with(&directory.path()) {
            if directory.allow_subdirectories || directory_is_first_level(&request.path(), &directory.path()){
                return handle_file_case(&mut stream, &request.path());
            } else if !directory_is_first_level(&request.path(), &directory.path()) {
                return respond_forbidden(&mut stream, "Forbidden, Access Denied");
            }
        }
    }

    handle_file_case(&mut stream, &request.path())?;

    respond_not_found(&mut stream, "Not Found")
}