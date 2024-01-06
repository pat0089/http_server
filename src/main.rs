use std::net::{TcpStream, TcpListener};
use std::io::{self, Read, Write};
mod request_validation;
use request_validation::{ validate_header, validate_request_line };

fn respond_bad_request(stream: &mut TcpStream, err: &str) -> io::Result<()> {
    let response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nError: {}\r\n", err);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    let mut buffer = Vec::new();
    let mut local_buf = [0; 1024];

    loop {
        let size = stream.read(&mut local_buf)?;
        if size == 0 {
            break;
        }
        buffer.extend_from_slice(&local_buf[..size]);

        // Check if we have received double CRLF
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    let request_str = String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut request_lines: Vec<&str> = request_str.lines().collect();

    // Parse the request header
    let request_line = request_lines[0];
    request_lines.remove(0);

    // Validate the request
    if let Err(err) = validate_request_line(request_line) {
        return respond_bad_request(&mut stream, &err);
    }

    for line in request_lines {
        if let Err(err) = validate_header(line) {
            return respond_bad_request(&mut stream, &err);
        }    
    }

    // Write a response
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
