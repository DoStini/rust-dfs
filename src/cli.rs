use std::process::exit;

use clap::Parser;
use dynamo::commands::commands::{handle_command, Operation};
use tokio::net::TcpStream;

// cargo run --bin cli -- -h localhost -p 1000 -o new -f

/// Cli that interacts with the system
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hostname of the bootstrap node
    #[arg(short = 'H', long)]
    hostname: String,

    /// Port to listen requests
    #[arg(short = 'p', long)]
    port: u16,

    /// Operation to perform
    #[command(subcommand)]
    operation: Operation,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let host = format!("{}:{}", args.hostname, args.port.to_string());

    let stream = TcpStream::connect(host).await;

    if let Err(err) = stream {
        eprintln!("Error setting up connection: {err}");
        exit(1);
    }

    let mut stream = stream.unwrap();

    handle_command(&args.operation, &mut stream).await;
}
