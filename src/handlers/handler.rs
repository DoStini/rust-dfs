use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpStream, time::sleep};

use super::message::{parse_message, print_message};

pub async fn handle_message(stream: TcpStream, origin: SocketAddr) {
    let msg = parse_message(stream, origin).await;

    print_message(msg);
}
