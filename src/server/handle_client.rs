use std::collections::HashMap;
use std::fs::read_to_string;
use std::net::TcpStream;
use std::io::{self, Read};
use std::str::FromStr;
use crate::server::util::mime_types::from_file_extension;
use crate::server::util::request_validation::{ validate_header, validate_request_line };
use crate::server::responses::{ error::{ respond_bad_request, respond_not_found, respond_forbidden, respond_internal_server_error}, ok::respond_ok_with_body_and_type };
use crate::http_builder::{HttpRequest, HttpRequestLine, HttpMethod, HttpHeader};
use crate::server::util::uri::get_file_extension;
use crate::server::routes::Route;
use crate::server::directories::{ Directory, directory_is_first_level };

use super::responses::experimental::respond_redirect;
//use crate::server::util::externals::ExternalRequest;

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

    // TODO: fix sending external requests


    // TODO: make a rich type mapping for finding routes, 
    // e.g. RouteMatch enum with 
        // Match(HashMap<String, String>)
        // Redirect(String)
        // NoMatch
    for route in routes {
        if request.method() == route.method() {
            match match_request_to_route(&request.path(), &route.path()) {
                RouteMatch::Match(params) => {
                    return route.call(&mut stream, params);
                }
                RouteMatch::Redirect(correct_path) => {
                    return respond_redirect(&mut stream, &correct_path);
                }
                RouteMatch::Malformed(error) => {
                    //get the correct path
                    return respond_bad_request(&mut stream, &error);
                }
                RouteMatch::NoMatch => {}
            }
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

    respond_not_found(&mut stream, "Not Found")
    
}

/// Figure out the malformation of a request to give a more specific error
/// first, match the path of the request to the path of the route to find the differences
fn malformatted_request_path(request_parts: &Vec<&str>, route_parts: &Vec<&str>) -> RouteMatch {
    for i in 0..route_parts.len() {
        
        if route_parts[i].starts_with(':') {
            //parameter
            if request_parts.get(i).is_none() {
                return RouteMatch::Malformed("Missing parameter".to_string());
            }
        }
        if request_parts.get(i).is_none() {
            //deviated from specified path
            return RouteMatch::Malformed("Malformed path".to_string());
        }
    }
    RouteMatch::Malformed("Unknown Malformation".to_string())
}

#[derive(Debug, PartialEq)]
enum RouteMatch {
    Match(HashMap<String, String>),
    Redirect(String),
    Malformed(String),
    NoMatch
}

/// Match the path of the request to the path of the route
/// unofficial validator for the path, can handle parameters starting with ':'
/// returns None if the paths do not match or are otherwise malformed
fn match_request_to_route(request_path: &str, route_path: &str) -> RouteMatch {
    let request_parts = request_path.trim_matches('/').split('/').collect::<Vec<&str>>();
    let route_parts = route_path.trim_matches('/').split('/').collect::<Vec<&str>>();

    if request_parts.len() > route_parts.len() {
        return RouteMatch::NoMatch;
    }

    let mut params = HashMap::new();
    for (request_part, path_part) in request_parts.iter().zip(route_parts.iter()) {
        if path_part.starts_with(':') {
            params.insert(path_part.trim_matches(':').to_string(), request_part.to_string());
        } else if request_part != path_part {
            return RouteMatch::NoMatch;
        }
    }

    if request_parts.len() < route_parts.len() {
        //TODO: check if this difference in length is because of missing parameters or malformed via incomplete path
        return malformatted_request_path(&request_parts, &route_parts);
    }

    RouteMatch::Match(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match_request_to_route() {
        assert_eq!(match_request_to_route("/home", "/home"), RouteMatch::Match(HashMap::new()));
        assert_eq!(match_request_to_route("/home", "/home/"), RouteMatch::Match(HashMap::new()));
        assert_eq!(match_request_to_route("/home", "/home/abc"), RouteMatch::Malformed("Malformed path".to_string()));
        assert_eq!(match_request_to_route("/home/abc", "/home/abc"), RouteMatch::Match(HashMap::new()));
        assert_eq!(match_request_to_route("/home/abc", "/home/abc/"), RouteMatch::Match(HashMap::new()));
        assert_eq!(match_request_to_route("/home/abc", "/home/abc/def"), RouteMatch::Malformed("Malformed path".to_string()));

        //NoMatch for this one instead of Malformed because the request path is longer than the route path
        assert_eq!(match_request_to_route("/home/abc", "/home/"), RouteMatch::NoMatch);
    }

    #[test]
    fn test_parameters_match_request_to_route() {
        assert_eq!(match_request_to_route("/home/abc", "/home/:id"), RouteMatch::Match(HashMap::from([("id".to_string(), "abc".to_string())])));
        assert_eq!(match_request_to_route("/home/abc", "/home/:id/"), RouteMatch::Match(HashMap::from([("id".to_string(), "abc".to_string())])));

        assert_eq!(match_request_to_route("/home/", "/home/:id"), RouteMatch::Malformed("Missing parameter".to_string()));
        assert_eq!(match_request_to_route("/home/abc", "/home/:id/def"), RouteMatch::Malformed("Malformed path".to_string()));
        assert_eq!(match_request_to_route("/home/abc/def", "/home/:id/def/ghi"), RouteMatch::Malformed("Malformed path".to_string()));
        assert_eq!(match_request_to_route("/home/abc/def/ghi", "/home/:id/def/:name"), RouteMatch::Match(HashMap::from([("id".to_string(), "abc".to_string()), ("name".to_string(), "ghi".to_string())])));

        //NoMatch for this one instead of Malformed because the request path is longer than the route path
        assert_eq!(match_request_to_route("/home/abc/def/ghi", "/home/:id/def/"), RouteMatch::NoMatch);
        assert_eq!(match_request_to_route("/home/abc/def", "/home/:id/def/:name/"), RouteMatch::Malformed("Missing parameter".to_string()));
    }

    #[test]
    fn test_edge_cases_match_request_to_route() {
        //difference between these are that the malformation of the request path is in the middle versus at the end
        assert_eq!(match_request_to_route("/a//", "/a/b/"), RouteMatch::Malformed("Malformed path".to_string()));
        assert_eq!(match_request_to_route("/a//c", "/a/b/c"), RouteMatch::NoMatch);

        //the difference here is that the missing parameter is at the end instead of in the middle, 
        // where it would technically be optional and/or be an empty value...
        // and if it shouldn't be, ...TODO
        assert_eq!(match_request_to_route("/your//", "/your/:id"), RouteMatch::Malformed("Missing parameter".to_string()));
        assert_eq!(match_request_to_route("/your//and/name", "/your/:id/and/:name"), RouteMatch::Match(HashMap::from([("id".to_string(), "".to_string()), ("name".to_string(), "name".to_string())])));
    }
}