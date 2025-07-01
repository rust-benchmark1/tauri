use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;
use crate::Result;

// TRANSFORMER 1: Parse command format
pub fn parse_command_format(command_data: &str) -> String {
    if command_data.starts_with("cmd=") {
        command_data.strip_prefix("cmd=").unwrap_or(command_data).to_string()
    } else {
        command_data.to_string()
    }
}

// TRANSFORMER 2: Apply CLI argument preprocessing
pub fn preprocess_cli_args(command_str: &str) -> String {
    let parts: Vec<&str> = command_str.split_whitespace().collect();
    if parts.is_empty() {
        return command_str.to_string();
    }
    
    parts.join(" ")
}

// TRANSFORMER 3: Resolve command path
pub fn resolve_command_path(command_input: &str) -> String {
    if command_input.contains("/") {
        command_input.replace("./", "")
    } else {
        command_input.to_string()
    }
}

pub fn process_network_command(raw_command: &str) -> Result<()> {
    let formatted = parse_command_format(raw_command);
    let preprocessed = preprocess_cli_args(&formatted);
    let resolved = resolve_command_path(&preprocessed);
    
    execute_system_command(&resolved)
}

fn execute_system_command(command_line: &str) -> Result<()> {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }
    
    let program = parts[0];
    let args = &parts[1..];
    
    Command::new(program).args(args).exec(); //SINK
    
    Ok(())
} 