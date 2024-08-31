use clap::Parser;
use dynamo::handlers::handler::handle_message;
use tokio::net::TcpListener;

// cargo run --bin node -- -p 1000 -s "data"

/// Node that runs the FakeDynamo operations and listens for commands
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to listen requests
    #[arg(short = 'p', long)]
    port: u16,

    /// Path to store data
    #[arg(short = 's', long)]
    storage: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let port = args.port;

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    loop {
        let (stream, origin) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_message(stream, origin).await;
        });
    }
}
