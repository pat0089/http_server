use std::io::{Read, Write};
use std::{net::TcpStream, sync::mpsc};
use std::thread;
use crate::http_builder::{HttpRequest, HttpResponse, HttpStatus};

pub struct ExternalRequest {
    request: HttpRequest,
}

pub struct ExternalResponse {
    pub response: HttpResponse,
    pub raw_response: String,
}

impl ExternalRequest {
    pub fn new(request: HttpRequest) -> Self {
        ExternalRequest {
            request
        }
    }

    pub fn send(self) -> Result<ExternalResponse, std::io::Error> {
        let (sender, receiver) = mpsc::channel();

        let handle = thread::spawn(move || -> Result<(), std::io::Error> {
            let mut stream = TcpStream::connect(format!("{}:80", self.request.get_host()))?;

            println!("request: {}", &self.request);
            stream.write_all(self.request.to_string().as_bytes())?;
    
            stream.flush()?;

            let mut response = String::new();

            stream.read_to_string(&mut response)?;
            match sender.send(response) {
                Ok(_) => (),
                Err(e) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e))
                }
            };

            Ok(())
        });

        let response = match receiver.recv() {
            Ok(response) => response,
            Err(_) => {
                return Ok(ExternalResponse {
                    response: HttpResponse::new(HttpStatus::InternalServerError),
                    raw_response: "Error receiving response".to_string(),
                })
            }
        };

        match handle.join() {
            Ok(_) => (),
            Err(_) => {
                return Ok(ExternalResponse {
                    response: HttpResponse::new(HttpStatus::InternalServerError),
                    raw_response: "Error joining thread".to_string(),
                })
            }
        };

        Ok(ExternalResponse {
            response: HttpResponse::new(HttpStatus::RequestOk),
            raw_response: response,
        })

    }
}