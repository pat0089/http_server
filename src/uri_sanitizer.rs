pub fn sanitize_uri(uri: &str) -> Result<String, String> {
    let parts: Vec<&str> = uri.split('/').collect();
    let mut sanitized_parts = Vec::new();
    for part in parts.iter() {
        if part == &".." {
            return Err(format!("Invalid URI segment: '{}'", part));
        }
        sanitized_parts.push(*part);
    }
    Ok(sanitized_parts.join("/"))
}