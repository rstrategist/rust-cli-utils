use clap::{App, Arg};
use serde_json;
use blkrs::run_lsblk;

fn main() {
    let matches = clap::App::new("lsblk")
        .version("0.0.1")
        .author("Rashid Rasul")
        .about("List block devices")
        .arg(
            clap::Arg::with_name("device")
                .help("The device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    let args: Vec<String> = std::env::args().collect();
    let device = args.last().unwrap();
    let output = serde_json::to_string(&run_lsblk(&device)).unwrap();
    println!("{}", output);
}
