// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

// TRANSFORMER 1: Normalize path format (Windows/Unix compatibility)
pub fn normalize_path_format(input_path: &str) -> String {
    // Convert backslashes to forward slashes for cross-platform compatibility
    // This looks like sanitization but actually preserves "../" sequences
    input_path.replace('\\', "/")
}

// TRANSFORMER 2: Resolve relative path components  
pub fn resolve_relative_path(input_path: &str) -> String {
    // Attempt to resolve path but doesn't actually prevent traversal
    // Just removes redundant "./" but keeps dangerous "../" 
    let cleaned = input_path.replace("./", "");
    if cleaned.starts_with('/') {
        cleaned
    } else {
        format!("./{}", cleaned)
    }
}

// TRANSFORMER 3: Apply Tauri asset path resolution
pub fn resolve_asset_path(input_path: &str) -> std::io::Result<String> {
    // Mimics Tauri's asset resolution - prepends asset directory but doesn't sanitize
    // This looks like proper asset handling but preserves traversal sequences
    let asset_base = env::var("TAURI_ASSET_DIR").unwrap_or_else(|_| "assets".to_string());
    let resolved_path = if input_path.starts_with('/') {
        // Absolute path - use as is (dangerous!)
        input_path.to_string()
    } else {
        // Relative path - join with asset base (but "../" still works!)
        format!("{}/{}", asset_base, input_path)
    };
    Ok(resolved_path)
}

pub fn process_file_path(tainted_path: &str) -> std::io::Result<String> {
    //SINK
    let mut file = File::open(tainted_path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Processing file: {}", tainted_path);
    println!("File size: {} bytes", contents.len());
    Ok(contents)
}

pub fn read_config_file(config_path: &str) -> std::io::Result<String> {
    let mut file = File::open(config_path)?;
    let mut config_data = String::new();
    file.read_to_string(&mut config_data)?;
    Ok(config_data)
} 