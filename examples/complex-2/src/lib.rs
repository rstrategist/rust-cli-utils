use std::process::Command;

/// Runs a shell command and returns its standard output as a String.
/// If the command fails, prints an error and returns an empty String.
fn run_command(command: &str) -> String {
    // Split the command string into the program and its arguments
    let args: Vec<&str> = command.split(" ").collect();
    // Create a new Command with the first argument as the program, and the rest as arguments
    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(output) => {
            // Convert the command's stdout from bytes to String
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        }
        Err(error) => {
            // Print error messages if the command fails
            println!("Command failed: {command}");
            eprintln!("error: {}", error);
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
    let output = run_command(command);
    if output.is_empty() {
        // Return empty JSON if the command failed
        return serde_json::json!({});
    }
    // Parse the command output as JSON
    let devices: serde_json::Value = serde_json::from_str(&output).unwrap();
    let devices = devices["blockdevices"].as_array().unwrap();
    // Search for the device in the top-level block devices
    for parent in devices {
        if parent["name"] == device {
            return parent.clone();
        }
        // If not found, search in the children of the device
        if let Some(children) = parent["children"].as_array() {
            for child in children {
                if child["name"] == device {
                    return child.clone();
                }
            }
        }
    }
    // Return empty JSON if the device was not
    serde_json::json!({})
}
