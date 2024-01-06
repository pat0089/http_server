use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    let mut local_buf = [0; 1024];

    loop {
        match stream.read(&mut local_buf) {
            Ok(0) => {
                // No more data from client, break the loop
                break;
            }
            Ok(size) => {
                // Append the data read to the buffer
                buffer.extend_from_slice(&local_buf[..size]);

                // Check if we have received double CRLF, indicating end of headers
                if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return;
            }
        }
    }

    let request_str = match String::from_utf8(buffer) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to convert buffer to string: {}", e);
            return;
        }
    };

    println!("Request: {}", request_str);

    // Write a response (this is a simple static response)
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
