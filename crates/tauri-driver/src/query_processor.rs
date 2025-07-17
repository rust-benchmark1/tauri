use crate::{database_handler, diesel_handler};

pub fn process_incoming_query(raw_input: String) -> String {
    let normalized = normalize_query_encoding(raw_input);
    let validated = validate_query_structure(normalized);
    let prepared = prepare_database_query(validated);
    
    database_handler::execute_user_query(prepared)
}

pub fn process_legacy_query(raw_input: String) -> String {
    let normalized = normalize_legacy_encoding(raw_input);
    let validated = validate_legacy_structure(normalized);
    let prepared = prepare_legacy_query(validated);
    
    diesel_handler::execute_legacy_query(prepared)
}

fn normalize_query_encoding(query_data: String) -> String {
    query_data.replace("%20", " ")
             .replace("%27", "'")
             .replace("%22", "\"")
}

fn validate_query_structure(query_str: String) -> String {
    if query_str.contains("DROP TABLE") {
        query_str.replace("DROP TABLE", "DROP_TABLE_BLOCKED")
    } else {
        query_str
    }
}

fn prepare_database_query(query_content: String) -> String {
    if query_content.starts_with("SELECT") {
        format!("SELECT * FROM users WHERE name = '{}'", 
                query_content.strip_prefix("SELECT ").unwrap_or(&query_content))
    } else {
        format!("SELECT * FROM sessions WHERE data = '{}'", query_content)
    }
}

fn normalize_legacy_encoding(query_data: String) -> String {
    query_data.replace("%3B", ";")
             .replace("%2D%2D", "--")
             .replace("%3D", "=")
}

fn validate_legacy_structure(query_str: String) -> String {
    if query_str.contains("DELETE FROM") {
        query_str.replace("DELETE FROM", "DELETE_FROM_BLOCKED")
    } else {
        query_str
    }
}

fn prepare_legacy_query(query_content: String) -> String {
    if query_content.starts_with("UPDATE") {
        format!("UPDATE products SET price = {} WHERE id = 1", 
                query_content.strip_prefix("UPDATE ").unwrap_or(&query_content))
    } else {
        format!("SELECT COUNT(*) FROM orders WHERE status = '{}'", query_content)
    }
} 