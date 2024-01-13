use std::path::Path;

pub fn sanitize(uri: &str) -> Result<String, String> {
    let path = Path::new(uri);

    // Check for '..' to prevent directory traversal attacks
    if path.components().any(|component| component.as_os_str() == "..") {
        return Err("Invalid URI: contains '..'".to_string());
    }

    // Normalize the path
    Ok(path
        .canonicalize()
        .map_err(|e| e.to_string())?
        .to_str()
        .ok_or("Failed to convert path to string")?
        .to_string())
}

pub fn get_file_extension(uri: &str) -> String {
    Path::new(uri)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_string()
}
