use std::net::SocketAddr;
use tokio::net::TcpStream;

use crate::storage::config::Storage;

use crate::handlers::{
    cli::handle_create,
    message::{parse_message, print_message, Message},
};

use super::cli::handle_delete;

async fn route_message(
    message: &Message,
    _origin: &SocketAddr,
    stream: &mut TcpStream,
    storage: &Storage,
) {
    match message.message_type {
        super::message::MessageType::CliPut => handle_create(message, storage, stream).await,
        super::message::MessageType::CliGet => todo!(),
        super::message::MessageType::CliDelete => handle_delete(message, storage, stream).await,
        super::message::MessageType::Error => todo!(),
        super::message::MessageType::Ok => todo!(),
    }
}

pub async fn handle_message(mut stream: TcpStream, origin: SocketAddr, storage: Storage) {
    let msg = parse_message(&mut stream, origin).await;

    print_message(&msg);

    route_message(&msg, &origin, &mut stream, &storage).await;
}
