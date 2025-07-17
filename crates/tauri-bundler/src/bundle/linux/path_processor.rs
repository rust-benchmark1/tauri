use std::path::Path;

pub fn normalize_path_separators(raw_path: &str) -> String {
    raw_path.replace("\\", "/")
}

pub fn resolve_relative_components(path: &str) -> String {
    if path.starts_with("./") {
        path.strip_prefix("./").unwrap_or(path).to_string()
    } else {
        path.to_string()
    }
}

pub fn extract_file_path(path: &str) -> String {
    if path.contains("file=") {
        path.split("file=")
            .nth(1)
            .unwrap_or(path)
            .split("&")
            .next()
            .unwrap_or(path)
            .to_string()
    } else {
        path.to_string()
    }
} 