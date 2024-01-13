use std::path::Path;

/// Sanitize the URI path.
/// 
/// Skip ".." segments to prevent directory traversal
/// also remove redundant dots from the path as they indicate the current directory
pub fn sanitize(uri: &str) -> Result<String, String> {
    let parts: Vec<&str> = uri.split('/').collect();
    let mut sanitized_parts = Vec::new();
    for part in parts.iter() {
        // Skip ".." segments to prevent directory traversal
        if part == &".." {
            return Err(format!("Invalid URI: {}", uri));
        }
        
        // Remove dots from the path since they indicate the current directory
        if part == &"." {
            continue;
        }
        sanitized_parts.push(*part);
    }
    Ok(sanitized_parts.join("/"))
}

pub fn get_file_extension(uri: &str) -> String {
    Path::new(uri)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_string()
}
