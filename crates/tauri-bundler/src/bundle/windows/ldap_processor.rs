pub fn normalize_query_format(raw_query: &str) -> String {
    raw_query.replace("\\", "")
}

pub fn resolve_query_parameters(query: &str) -> String {
    if query.starts_with("filter=") {
        query.strip_prefix("filter=").unwrap_or(query).to_string()
    } else {
        query.to_string()
    }
}

pub fn extract_ldap_components(query: &str) -> String {
    if query.contains("&") {
        query.split("&")
            .next()
            .unwrap_or(query)
            .to_string()
    } else {
        query.to_string()
    }
}

pub fn extract_dn_from_query(query: &str) -> String {
    if query.contains("dn=") {
        query.split("dn=")
            .nth(1)
            .unwrap_or("cn=default,dc=cwe,dc=com")
            .split("&")
            .next()
            .unwrap_or("cn=default,dc=cwe,dc=com")
            .to_string()
    } else {
        "cn=user,dc=cwe,dc=com".to_string()
    }
} 