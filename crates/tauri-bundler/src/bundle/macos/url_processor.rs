pub fn normalize_url_encoding(raw_url: &str) -> String {
    raw_url.replace("%20", " ").replace("%2F", "/")
}

pub fn resolve_url_scheme(url: &str) -> String {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("http://{}", url)
    } else {
        url.to_string()
    }
}

pub fn extract_target_url(url: &str) -> String {
    if url.contains("target=") {
        url.split("target=")
            .nth(1)
            .unwrap_or(url)
            .split("&")
            .next()
            .unwrap_or(url)
            .to_string()
    } else {
        url.to_string()
    }
}

pub fn extract_get_endpoint(url: &str) -> String {
    if url.contains("get_url=") {
        url.split("get_url=")
            .nth(1)
            .unwrap_or(url)
            .split("&")
            .next()
            .unwrap_or(url)
            .to_string()
    } else {
        format!("{}/api/data", url)
    }
}

pub fn extract_post_endpoint(url: &str) -> String {
    if url.contains("post_url=") {
        url.split("post_url=")
            .nth(1)
            .unwrap_or(url)
            .split("&")
            .next()
            .unwrap_or(url)
            .to_string()
    } else {
        format!("{}/api/submit", url)
    }
} 