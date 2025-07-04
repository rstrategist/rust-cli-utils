use blkrs::run_lsblk;
use clap::{ArgAction, Parser};

// This example demonstrates how to use environment variables
// to set global flags for a command-line application.
// To see all available devices in codespaces, run:
// cat /proc/partitions
// To run this example, you can use the following command:
// cargo run -- --debug info sda

#[derive(Parser)]
#[command(
    name = "lsblk",
    version = "0.0.1",
    author = "Rashid Rasul",
    about = "lsblk in Rust"
)]
struct Opts {
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(short, long, env = "BLKRS_DEBUG")]
    debug: bool,

    #[clap(long, env = "BLKRS_ENABLE_LOGGING")]
    enable_logging: bool,

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "info", about = "Get information about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "Device to query")]
    device: String,
}

fn main() {
    /* Parse command line arguments
    Environment variables can be used to set the debug flag
    and the verbosity level.
    For example, you can set BLKRS_DEBUG=1 to enable debug mode.
    The verbosity level can be set using the --verbose or -v flag.
    You can also use the environment variable BLKRS_VERBOSE to set the verbosity level.
    The verbosity level can be set to 0, 1, 2, or 3.
    The default verbosity level is 0 (quiet mode).
    If the verbosity level is set to 1, 2, or 3,
    the program will print additional information to the console.
    The debug flag can be set to enable debug mode.
    In debug mode, the program will print additional debug information to the console.
    */

    // Retrieve the debug flag from the environment variable
    // or use the default value if not set.
    // The environment variable can be set to "true" or "false".
    // If the environment variable is not set, it defaults to "false".
    use std::env;
    let debug = env::var("BLKRS_DEBUG").unwrap_or_else(|_| "false".to_string());

    let opts = Opts::parse();

    // Example usage of the global flags
    if opts.debug {
        println!("Debug mode enabled");
    }

    if opts.enable_logging {
        println!("Logging is enabled (via --enable-logging or BLKRS_ENABLE_LOGGING)");
    }

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
