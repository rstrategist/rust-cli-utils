use log::{error, info};
use std::process::Command;

/// Runs a shell command and returns its standard output as a String.
/// If the command fails, prints an error and returns an empty String.
fn run_command(command: &str) -> String {
    // Split the command string into the program and its arguments
    let args: Vec<&str> = command.split(" ").collect();
    // Create a new Command with the first argument as the program, and the rest as arguments
    info!("Running command: {}", command);
    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(output) => {
            // Convert the command's stdout from bytes to String
            let stdout = String::from_utf8_lossy(&output.stdout);
            info!("Command '{}' executed successfully", command);
            stdout.to_string()
        }
        Err(error) => {
            // Print error messages if the command fails
            error!("Command failed: {command}");
            error!("error: {}", error);
            "".to_string()
        }
    }
}

/// Runs the `lsblk` command to get block device information in JSON format,
/// then searches for a device by name and returns its JSON object.
/// If the device is not found or the command fails, returns an empty JSON object.
pub fn run_lsblk(device: &str) -> serde_json::Value {
    // Prepare the lsblk command to output JSON with specific columns
    let command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
    info!("Querying block devices using lsblk for device: {}", device);
    let output = run_command(command);
    if output.is_empty() {
        // Return empty JSON if the command failed
        error!("lsblk command failed or returned empty output");
        return serde_json::json!({});
    }
    // Parse the command output as JSON
    //let devices: serde_json::Value = serde_json::from_str(&output).unwrap();
    let devices: serde_json::Value = match serde_json::from_str(&output) {
        Ok(devices) => devices,
        // If parsing fails, return empty JSON
        Err(error) => {
            error!("Failed to parse JSON from lsblk output: {error}");
            error!("Full Output: {output}");
            return serde_json::json!({});
        }
    };

    let devices = match devices["blockdevices"].as_array() {
        Some(arr) => arr,
        None => {
            error!("No 'blockdevices' array found in lsblk output");
            return serde_json::json!({});
        }
    };

    // Search for the device in the top-level block devices
    for parent in devices {
        if parent["name"] == device {
            info!("Found device '{}' at top level", device);
            return parent.clone();
        }
        // If not found, search in the children of the device
        if let Some(children) = parent["children"].as_array() {
            for child in children {
                if child["name"] == device {
                    info!("Found device '{}' as a child device", device);
                    return child.clone();
                }
            }
        }
    }
    // Return empty JSON if the device was not found
    error!("Device '{}' not found in lsblk output", device);
    serde_json::json!({})
}
