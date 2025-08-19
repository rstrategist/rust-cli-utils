use clap::{Parser, Subcommand};
use rust_cli_utils::run_lsblk;

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Rashid Rasul"
    about = "lsblk in Rust"
)]

struct Opts {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "info", about = "get info about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(short, long, about = "device to get info about")]
    device: String,
}

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Info(info) => {
            let device = info.device;
            let output = run_lsblk(&device);
            pprintln!("{}", output);
        }
    }
}

// cargo run -- --help
// cargo run info vda1
