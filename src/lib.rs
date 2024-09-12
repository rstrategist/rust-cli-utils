//! This is a library that provides utilities for command-line tools.
//! Current functions and examples are shown below.
//! # Examples:
//! ```
//! use rust_cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line".

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
pub mod colors;
pub mod config;

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use rust_cli_utils::read_stdin;
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// //use cli_utils::read_stdin;
/// //let input = read_stdin();
/// ```
fn run_command(command: &str) -> String {
    let args: Vec<&str> = command.split(" ").collect();
    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        }
        Err(error) => {
            println!("Command failed: {command}");
            eprintln!("error: {}", error);
            "".to_string()
        }
    }
}

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// //use cli_utils::read_stdin;
/// //let input = read_stdin();
/// ```
pub fn run_lsblk(device: &str) -> serde_json::Value {
    let command = "llsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
    let output = run_command(command);
    if output.is_empty() {
        return serde_json::json!({});
    }
    let devices: serde_json::Value = serde_json::from_str(&output).unwrap();
    let devices = devices["blockdevices"].as_array().unwrap();
    for parent in devices {
        if parent["name"] == device {
            return parent.clone();
        }
        if let Some(children) = parent["children"].as_array() {
            for child in children {
                if child["name"] == device {
                    return child.clone();
                }
            }
        }
    }
    serde_json::json!({})
}

#[cfg(test)]
mod tests {
    use super::_read_stdin;
    use std::io::Cursor;

    #[test]
    fn test_read_input() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_input_empty() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }
}
