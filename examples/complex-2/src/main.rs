use blkrs::run_lsblk; // Import the run_lsblk function from the blkrs crate/module
use clap::{ArgAction, Parser}; // Import necessary items from the clap crate for CLI parsing

/* Example usage:

# Get info about the device 'sda'
cargo run -- info sda

# Enable debug output
cargo run -- --debug info sda

# Increase verbosity (once)
cargo run -- -v info sda

# Increase verbosity (multiple times)
cargo run -- -vv info sda

# Combine debug and verbosity flags
cargo run -- --debug -vv info sda
*/

// Define the main options struct for the CLI using clap's derive macros
#[derive(Parser)]
#[command(version = "0.0.1", author = "Rashid Rasul", about = "lsblk in Rust")]
struct Opts {
    // Debug flag, enables improved output for debugging
    #[clap(
        short,
        long,
        help = "Provides improved output for debugging",
        default_value_t = false
    )]
    debug: bool,

    // Verbosity flag, can be specified multiple times to increase verbosity
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    // Subcommand to specify the action to perform
    #[clap(subcommand)]
    cmd: Command,
}

// Enum for supported subcommands
#[derive(Parser)]
enum Command {
    // Info subcommand to get info about a device
    #[clap(name = "info", about = "Get info about a device")]
    Info(InfoOpts),
}

// Options for the "info" subcommand
#[derive(Parser)]
struct InfoOpts {
    // Device name to get info about
    #[clap(help = "Device to get info about")]
    device: String,
}

fn main() {
    // Parse command-line arguments into Opts struct
    let opts = Opts::parse();

    // Print debug flag value
    println!("{:?}", opts.debug);

    // Print verbosity level based on how many times -v/--verbose is used
    match opts.verbose_level {
        0 => println!("no verbosity"),
        1 => println!("some verbosity"),
        2 => println!("more verbosity"),
        _ => println!("too much verbosity"),
    }

    // Match on the subcommand and execute the appropriate logic
    match opts.cmd {
        Command::Info(info) => {
            let device = info.device;
            // Call run_lsblk to get device info and print the result
            let output = run_lsblk(&device);
            println!("{}", output);
        }
    }
}
