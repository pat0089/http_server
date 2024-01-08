use std::str::FromStr;
use std::fmt::{Formatter, self};

// Define an enum for the status code and message
#[derive(Debug, Clone)]
pub enum HttpStatus {
    Ok,  // 200
    Created,  // 201
    Accepted,  // 202
    NoContent,  // 204
    BadRequest,  // 400
    Forbidden,  // 403
    NotFound,  // 404
    InternalServerError,  // 500
    // Add more status codes as needed
}
impl FromStr for HttpStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "200 OK" => Ok(HttpStatus::Ok),
            "201 Created" => Ok(HttpStatus::Created),
            "202 Accepted" => Ok(HttpStatus::Accepted),
            "204 No Content" => Ok(HttpStatus::NoContent),
            "400 Bad Request" => Ok(HttpStatus::BadRequest),
            "403 Forbidden" => Ok(HttpStatus::Forbidden),
            "404 Not Found" => Ok(HttpStatus::NotFound),
            "500 Internal Server Error" => Ok(HttpStatus::InternalServerError),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HttpStatus {
    // Returns the status code and message as a tuple
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::Created => write!(f, "201 Created"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
            HttpStatus::Forbidden => write!(f, "403 Forbidden"),
            HttpStatus::BadRequest => write!(f, "400 Bad Request"),
            HttpStatus::NoContent => write!(f, "204 No Content"),
            HttpStatus::InternalServerError => write!(f, "500 Internal Server Error"),
            _ => write!(f, "Not Implemented!"),
        }
    }
}

// Define an enum for different content types
#[derive(Debug, Clone, PartialEq, Eq)]

pub enum ContentType {
    Html,
    Json,
    PlainText,
    Javascript,
    Css,
    // Add more as needed
}

impl fmt::Display for ContentType {
    // Returns the MIME type for the content type
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ContentType::Html => write!(f, "text/html"),
            ContentType::Json => write!(f, "application/json"),
            ContentType::PlainText => write!(f, "text/plain"),
            ContentType::Javascript => write!(f, "text/javascript"),
            ContentType::Css => write!(f, "text/css"),
        }
    }
}

impl FromStr for ContentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text/html" => Ok(ContentType::Html),
            "application/json" => Ok(ContentType::Json),
            "text/plain" => Ok(ContentType::PlainText),
            "text/javascript" => Ok(ContentType::Javascript),
            "text/css" => Ok(ContentType::Css),
            _ => Err(()),
        }
    }
}

/// Enum to represent the HTTP method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
    // Extend with other methods as needed.
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "PATCH" => Ok(HttpMethod::PATCH),
            "TRACE" => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::TRACE => write!(f, "TRACE"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
        }
    }
}

/// Enum to represent specific header types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpHeader {
    ContentType(ContentType),
    ContentLength(u64),
    Custom(String, String), // For headers not explicitly listed here.
}

impl fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeader::ContentType(content_type) => write!(f, "Content-Type: {}\r\n", content_type),
            HttpHeader::ContentLength(content_length) => write!(f, "Content-Length: {}\r\n", content_length),
            HttpHeader::Custom(name, value) => write!(f, "{}: {}\r\n", name, value),
        }
    }
}

/// Structure to represent the first line of an HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequestLine {
    method: HttpMethod,
    uri: String,
    http_version: String,
}

impl HttpRequestLine {
    pub fn new(method: HttpMethod, uri: &str, http_version: &str) -> Self {
        HttpRequestLine {
            method,
            uri: uri.to_string(),
            http_version: http_version.to_string(),
        }
    }
}

impl fmt::Display for HttpRequestLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\r\n", self.http_version, self.method, self.uri)
    }
}

struct HttpResponseLine {
    status: HttpStatus
}

impl HttpResponseLine {
    fn new(status: HttpStatus) -> Self {
        HttpResponseLine {
            status,
        }
    }
}

impl fmt::Display for HttpResponseLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {}\r\n", self.status)
    }
}

pub struct HttpResponse {
    response_line: HttpResponseLine,
    headers: Vec<HttpHeader>,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> Self {
        HttpResponse {
            response_line: HttpResponseLine::new(status),
            headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, header: HttpHeader) {
        self.headers.push(header);
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut headers_string = String::new();
        for header in &self.headers {
            headers_string.push_str(&format!("{}", header));
        }
        write!(f, "{}{}\r\n\r\n", self.response_line, headers_string)
    }
}


/// Structure to represent a complete HTTP request header.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    request_line: HttpRequestLine,
    headers: Vec<HttpHeader>,
}

impl HttpRequest {
    pub fn new(request_line: HttpRequestLine) -> Self {
        HttpRequest {
            request_line,
            headers: Vec::new(),
        }
    }

    /// Adds a header to the collection.
    pub fn add_header(&mut self, header: HttpHeader) {
        self.headers.push(header);
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut headers_string = String::new();
        for header in &self.headers {
            headers_string.push_str(&format!("{}", header));
        }
        write!(f, "{}{}\r\n\r\n", self.request_line, headers_string)
    }
}

pub fn write_http_response_header(code :u32, message: &str, content_type: &str) -> String {
    let mut response = HttpResponse::new(
        HttpStatus::from_str(
            &format!("{} {}", code, message))
            .unwrap_or(HttpStatus::InternalServerError)
    );

    response.add_header(HttpHeader::ContentType(ContentType::from_str(content_type).unwrap_or(ContentType::PlainText)));

    response.to_string()
    //format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\n\r\n", code.to_string(), message, content_type)
}