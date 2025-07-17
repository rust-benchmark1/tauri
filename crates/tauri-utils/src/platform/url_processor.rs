use std::collections::HashMap;

pub fn normalize_url_format(raw_url: &str) -> String {
    raw_url.replace("\\", "/")
}

pub fn resolve_url_scheme(url: &str) -> String {
    if url.starts_with("//") {
        format!("http:{}", url)
    } else {
        url.to_string()
    }
}

pub fn parse_redirect_params(url: &str) -> String {
    let params: HashMap<&str, &str> = url
        .split('&')
        .filter_map(|param| {
            let mut parts = param.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key, value)),
                _ => None,
            }
        })
        .collect();
    
    params.get("redirect_url")
        .or_else(|| params.get("url"))
        .or_else(|| params.get("target"))
        .unwrap_or(&url)
        .to_string()
} 