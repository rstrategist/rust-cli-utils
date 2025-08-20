use blkrs::run_lsblk; // Import the run_lsblk function from the blkrs crate/module
use clap::{ArgAction, Parser}; // Import necessary items from the clap crate for CLI parsing
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::OpenOptions;

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

# Log output to a file
cargo run -- --log-file info sda

*/

// Define the main options struct for the CLI using clap's derive macros
#[derive(Parser)]
#[command(
    name = "lsblk",
    version = "0.0.1",
    author = "Rashid Rasul",
    about = "lsblk in Rust"
)]
struct Opts {
    // Verbosity flag, can be specified multiple times to increase verbosity
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    // Log file flag, enables logging to a file if specified
    #[clap(long, help = "Enable logging to a file")]
    log_file: bool,

    #[clap(short, long, env = "BLKRS_DEBUG")]
    debug: bool,

    // Subcommand to specify the action to perform
    #[clap(subcommand)]
    cmd: Command,
    // // Debug flag, enables improved output for debugging
    // #[clap(
    //     short,
    //     long,
    //     help = "Provides improved output for debugging",
    //     default_value_t = false
    // )]
    // debug: bool,
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

    // Initialize the logger with the specified log level and target
    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Debug);

    // If log_file is enabled, set the target to write logs to a file
    if opts.log_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("blkrs.log")
            .unwrap();
        builder.target(Target::Pipe(Box::new(file)));
    }

    builder.init();

    // Example usage of the global flags
    if opts.debug {
        log::debug!("Debug mode enabled");
    }

    log::info!("info message logging is enabled!");

    // Print verbosity level based on how many times -v/--verbose is used
    match opts.cmd {
        Command::Info(info_opts) => {
            // Example usage of the verbosity level
            match opts.verbose_level {
                0 => {
                    // Quiet mode
                }
                1 => {
                    println!("Running in verbose mode level 1");
                }
                2 => {
                    println!("Running in verbose mode level 2");
                }
                3 | _ => {
                    println!("Running in verbose mode level 3");
                }
            }

            let output = serde_json::to_string(&run_lsblk(&info_opts.device)).unwrap();
            println!("{}", output);
        }
    }
}
