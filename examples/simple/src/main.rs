use blkrs::run_lsblk;
use clap::{App, Arg};
use serde_json;

fn main() {
    // Define the command-line interface using clap
    let matches = App::new("lsblk")
        .version("0.0.1")
        .author("Rashid Rasul")
        .about("List block devices")
        // Define the command-line arguments
        .arg(
            Arg::with_name("device")
                .help("The device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Extract the device argument and run the lsblk command
    let device = matches.value_of("device").unwrap();
    // Call the run_lsblk function from the blkrs crate
    // and serialize the output to JSON
    let output = serde_json::to_string(&run_lsblk(&device)).unwrap();
    println!("{}", output);
}
