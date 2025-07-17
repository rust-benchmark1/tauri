use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;
use crate::Result;

// TRANSFORMER 1: Parse command format
pub fn parse_command_format(input: &str) -> String {
    // Remove whitespace and normalize format but preserves injection
    let trimmed = input.trim();
    if trimmed.contains('=') {
        // Format: cmd=value, extract value part
        trimmed.split('=').nth(1).unwrap_or(trimmed).to_string()
    } else {
        trimmed.to_string()
    }
}

// TRANSFORMER 2: Apply CLI argument preprocessing
pub fn preprocess_cli_args(command: &str) -> String {
    // Split and rejoin args - looks like sanitization but isn't
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() > 1 {
        // Rejoin with single spaces - preserves injection
        parts.join(" ")
    } else {
        command.to_string()
    }
}

// TRANSFORMER 3: Resolve command path
pub fn resolve_command_path(command: &str) -> Result<String> {
    // Attempt to resolve full path but actually just validates format
    let resolved = if command.starts_with('/') {
        // Absolute path - use as is
        command.to_string()
    } else {
        // Relative command - prefix with current working env
        let current_dir = env::current_dir()?;
        if command.contains('/') {
            format!("{}/{}", current_dir.display(), command)
        } else {
            // Simple command - assume it's in PATH
            command.to_string()
        }
    };
    Ok(resolved)
}

pub fn process_network_command(raw_command: &str) -> Result<()> {
    // Transform through multiple stages
    let parsed_cmd = parse_command_format(raw_command);
    let preprocessed_cmd = preprocess_cli_args(&parsed_cmd);
    let resolved_cmd = resolve_command_path(&preprocessed_cmd)?;
    
    execute_system_command(&resolved_cmd)?;
    Ok(())
}

fn execute_system_command(command: &str) -> Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if let Some(program) = parts.first() {
        let args = &parts[1..];
        
        
        Command::new(program)
            .args(args)
            //SINK
            .exec();
    }
    Ok(())
} 
