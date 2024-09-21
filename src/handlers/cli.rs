use tokio::net::TcpStream;

use super::message::Message;
use crate::{
    handlers::message::send_ack,
    helpers::file::deserialize_create_file,
    storage::{config::Storage, errors::send_error_storage_data, manager::StorageManager},
};

pub async fn handle_create(message: &Message, storage: &Storage, stream: &mut TcpStream) {
    let (filename, file_content) = deserialize_create_file(&message.content);
    let res = storage.store_file(&filename, &file_content).await;

    if let Err(storage_error) = res {
        send_error_storage_data(storage_error, stream).await;
        return;
    }

    let res = send_ack(stream).await;
    if let Err(err) = res {
        eprintln!("Error sending ack create: {}", err.to_string());
        return;
    }

    println!("Created successfully")
}
