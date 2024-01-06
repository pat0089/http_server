use std::fs::File;
use std::net::TcpStream;
use std::io::{self, Read};
use crate::request_validation::{ validate_header, validate_request_line };
use crate::responses::{ respond_bad_request, respond_ok, respond_not_found };

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

    // Validate the request
    match validate_request_line(&request_line) {
        Err(err) => return respond_bad_request(&mut stream, &err),
        Ok(path) => {
            //if the path points to nothing, return 404
            if File::open(path).is_err() {
                return respond_not_found(&mut stream, "File not found");
            }
            //add logic here later for 403 Forbidden
        }
    } 
    
    for line in header_lines {
        if let Err(err) = validate_header(line) {
            return respond_bad_request(&mut stream, &err);
        }    
    }

    // Write a response
    respond_ok(&mut stream)

}
