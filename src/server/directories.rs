use super::util::uri::sanitize;

/// <h1>Directory</h1> 
/// Structure to represent a directory.
/// Contains the path of the directory, to be used for accessing files in that directory.
pub struct Directory {
    path: String,
    pub allow_subdirectories: bool
}

impl Directory {
    pub fn new(path: &str, allow_subdirectories: bool) -> Self {
        Self {
            path: path.to_string(),
            allow_subdirectories
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

pub fn directory_is_first_level(requested_path: &str, allowed_path: &str) -> bool {
    //sanitize the path before comparing
    let sanitized_requested_path = sanitize(requested_path).unwrap_or("".to_string());
    let requested_path = sanitized_requested_path.split('/').collect::<Vec<&str>>();
    let allowed_path = allowed_path.split('/').collect::<Vec<&str>>();

    if requested_path.len() > allowed_path.len() {
        return false;
    }
    true
}
