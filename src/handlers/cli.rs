use tokio::net::TcpStream;

use super::message::{send_message, Message};
use crate::{
    handlers::message::send_ack,
    helpers::file::{deserialize_create_file, deserialize_filename_operation},
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
}

pub async fn handle_get(message: &Message, storage: &Storage, stream: &mut TcpStream) {
    let filename = deserialize_filename_operation(&message.content);
    let res = storage.get_file(&filename).await;

    match res {
        Ok(mut content) => {
            match send_message(super::message::MessageType::Ok, &mut content, stream).await {
                Err(err) => eprintln!("Error sending get reply: {}", err),
                Ok(_) => {}
            }
        }
        Err(storage_err) => send_error_storage_data(storage_err, stream).await,
    };
}

pub async fn handle_delete(message: &Message, storage: &Storage, stream: &mut TcpStream) {
    let filename = deserialize_filename_operation(&message.content);
    let res = storage.delete_file(&filename).await;

    if let Err(storage_error) = res {
        send_error_storage_data(storage_error, stream).await;
        return;
    }

    let res = send_ack(stream).await;
    if let Err(err) = res {
        eprintln!("Error sending ack delete: {}", err.to_string());
        return;
    }
}
