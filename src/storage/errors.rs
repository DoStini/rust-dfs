use tokio::net::TcpStream;

use crate::handlers::message::{build_error_data, send_message, ErrorType, MessageType};

#[derive(Debug)]
pub enum StorageErrors {
    AlreadyExists = 1,
    NotFound = 2,
    UnknownError = 3,
}

pub fn get_storage_error_type(t: &u8) -> StorageErrors {
    match t {
        1 => StorageErrors::AlreadyExists,
        2 => StorageErrors::NotFound,
        3 => StorageErrors::UnknownError,
        _ => panic!("Invalid value: {t}"),
    }
}

pub async fn send_error_storage_data(error_type: StorageErrors, stream: &mut TcpStream) {
    let mut content = vec![error_type as u8];
    let mut data = build_error_data(ErrorType::StorageError, &mut content);

    let res = send_message(MessageType::Error, &mut data, stream).await;
    if let Err(err) = res {
        eprintln!("Error sending error message: {}", err.to_string());
    }
}
