use crate::server::util::uri::sanitize;

// REST methods: GET, POST, PUT, DELETE
const REQUEST_METHODS: [&str; 4] = ["GET", "POST", "PUT", "DELETE"];

pub fn validate_request_line(request_line: &str) -> Result<String, String> {
    let parts: Vec<&str> = request_line.splitn(3, ' ').collect();
    let method = parts[0];
    let path = parts[1];
    let protocol = parts[2];
    if parts.len() != 3 {
        return Err("Invalid request line: missing method, path, or protocol".to_string());
    }
    if !REQUEST_METHODS.contains(&method) {
        return Err("Invalid request line: invalid method".to_string());
    }
    // Sanitize the path
    let path = sanitize(path)?;

    // Validate the path
    if path.is_empty() {
        return Err("Invalid request line: empty path".to_string());
    }

    println!("Method: {}, Path: {}, Protocol: {}", method, path, protocol);
    Ok(path)
}

pub fn validate_header(header: &str) -> Result<(), String> {
    //skip empty headers
    if header.is_empty() {
        return Ok(());
    }
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("{}", header.len()));
    }

    let header_name = parts[0].trim();
    let header_value = parts[1].trim();

    // Validate header name and value
    match header_name.to_lowercase().as_str() {
        "host" => validate_host_header(header_value),
        "content-length" => validate_content_length(header_value),
        // Add more header validations as needed
        _ => Ok(()), // For now, other headers are not validated
    }
}

fn validate_host_header(value: &str) -> Result<(), String> {
    if value.is_empty() {
        Err("Host header value is empty".to_string())
    } else {
        // Further validation can be added here
        Ok(())
    }
}

fn validate_content_length(value: &str) -> Result<(), String> {
    value.parse::<u64>()
        .map_err(|_| "Invalid Content-Length value".to_string())?;
    Ok(())
}
