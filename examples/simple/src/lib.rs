use std::process::Command;

fn run_command(command: &str) -> String {
    let args: Vec<&str> = command.split(" ").collect();
    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.to_string()
}

/// Function to run the lsblk command and return the JSON output for a specific device
/// # Arguments
/// * `device` - The name of the device to query (e.g., "sda", "nvme0n1")
/// # Returns
/// A serde_json::Value containing the details of the specified device
pub fn run_lsblk(device: &str) -> serde_json::Value {
    let command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
    let output = run_command(command);
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
    panic!("Device not found");
}
