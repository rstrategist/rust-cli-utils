use chrono::prelude::*;
use clap::Parser;

// This example demonstrates how to create a CLI application
// that generates a timestamp with an optional offset in seconds.
// The timestamp can be formatted in either RFC 3339 or a default format (YYYY-MM-DD HH:MM:SS).
// To run this example, you can use the following command:
// cargo run -- --help
// cargo run -- 60
// cargo run -- 30 --format rfc3339

/// Generate a timestamp with an optional offset
#[derive(Parser)]
#[command(
    name = "timestamp-cli",
    version = "0.1.0",
    author = "Rashid Rasul",
    about = "Generate a timestamp with an optional offset"
)]
struct Cli {
    /// Number of seconds to add to the current time
    #[arg(default_value_t = 0)]
    seconds: i64,

    /// Output format: "rfc3339" or "default" (YYYY-MM-DD HH:MM:SS)
    #[arg(long, default_value = "default")]
    format: String,
}

fn main() {
    let cli = Cli::parse();

    let now = Local::now();
    let target = now + chrono::Duration::seconds(cli.seconds);

    let output = match cli.format.to_lowercase().as_str() {
        "rfc3339" => target.to_rfc3339_opts(SecondsFormat::Secs, true),
        "default" => target.format("%Y-%m-%d %H:%M:%S").to_string(),
        other => {
            eprintln!("Error: Unsupported format '{}'", other);
            std::process::exit(1);
        }
    };

    println!("{}", output);
}
