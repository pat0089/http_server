use std::fs::read_to_string;
use std::net::TcpStream;
use std::io::{self, Read};
use crate::request_validation::{ validate_header, validate_request_line };
use crate::responses::{ respond_bad_request, respond_ok_with_body, respond_ok, respond_not_found, respond_forbidden, respond_internal_server_error };

pub fn handle_client(mut stream: TcpStream) -> io::Result<()>{
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

    let request_str = String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut header_lines: Vec<&str> = request_str.lines().collect();

    if header_lines.is_empty() {
        return respond_bad_request(&mut stream, "Empty request");
    }
    // Parse the request header
    let request_line = header_lines[0];
    header_lines.remove(0);

    // Validate the header
    for line in header_lines {
        if let Err(err) = validate_header(line) {
            return respond_bad_request(&mut stream, &err);
        }    
    }

    // Validate the request, then respond
    match validate_request_line(&request_line) {
        Err(err) => return respond_bad_request(&mut stream, &err),
        Ok(path) => {
            let path = path.chars().skip(1).collect::<String>();
            if path.is_empty() {
                return respond_ok(&mut stream);
            }
            let file_contents = read_to_string(&path);
            match file_contents {
                Ok(contents) => {
                    return respond_ok_with_body(&mut stream, &contents);
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
    } 
    


}
