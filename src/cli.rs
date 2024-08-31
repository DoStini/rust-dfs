use std::process::exit;

use clap::{Parser, ValueEnum};
use dynamo::handlers::message::{build_message_data, send_message, MessageType};
use tokio::{io::AsyncWriteExt, net::TcpStream, stream};

// cargo run --bin cli -- -h localhost -p 1000 -o new -f
#[derive(ValueEnum, Debug, Clone, Copy)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum Operation {
    New = 1,
    Get = 2,
    Delete = 3,
}

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
    #[arg(short = 'o', value_enum)]
    operation: Operation,

    /// Name of the file
    #[arg(short = 'f', long)]
    file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!(
        "Args: {} {} {} {}",
        args.hostname, args.port, args.operation as u8, args.file
    );

    let port = args.port.to_string();
    let host = format!("{}:{}", args.hostname, args.port.to_string());

    let stream_res = TcpStream::connect(host).await;

    if let Err(err) = stream_res {
        eprintln!("Error setting up connection: {err}");
        exit(1);
    }

    let message_type = match args.operation {
        Operation::Get => MessageType::CliGet,
        Operation::New => MessageType::CliPut,
        Operation::Delete => MessageType::CliDelete,
    };

    let res = send_message(message_type, args.file, stream_res.unwrap()).await;

    if let Err(err) = res {
        eprintln!("Error sending message to node {err}");
        exit(2);
    }

    println!("Sucessfully sent");
}
