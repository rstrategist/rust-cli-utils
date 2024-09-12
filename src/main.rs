use clap::ColorChoice;
use clap::{Arg, Command};
use rust_cli_utils::run_lsblk;

// fn main() {
//     let matches = Command::new("lsblk")
//         .version("0.0.1")
//         .author("Rashid Rasul")
//         .about("lsblk in Rust")
//         .color(ColorChoice::Always)
//         .arg(
//             Arg::new("device")
//                 .help("Device to query")
//                 .required(true)
//                 .index(1),
//         )
//         .get_matches();

//     if let Some(device) = matches.get_one::<String>("device") {
//         let output = serde_json::to_string(&run_lsblk(&device)).unwrap();
//         println!("{}", output);
//     } else {
//         println!("No device provided");
//     }
// }

fn main() {
    use rust_cli_utils::colors::*;
    println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
}
