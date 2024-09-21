use clap::ValueEnum;
use std::process::exit;
use tokio::net::TcpStream;

use crate::{
    handlers::message::{get_type_literal, send_message, MessageType},
    helpers::file::{read_to_buf, serialize_create_file, serialize_filename},
};

#[derive(ValueEnum, Debug, Clone, Copy)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum Operation {
    New = 1,
    Get = 2,
    Delete = 3,
}

async fn create_command(filename: &String, stream: &mut TcpStream) {
    let mut file_buffer = Vec::new();
    read_to_buf(filename, &mut file_buffer).await;

    let mut send_buffer = Vec::new();
    serialize_create_file(filename, &mut file_buffer, &mut send_buffer);
    let res = send_message(MessageType::CliPut, &mut send_buffer, stream).await;
    if let Err(err) = res {
        eprintln!(
            "Error sending  {} to node {}",
            get_type_literal(MessageType::CliPut),
            err
        );
        exit(2);
    }
}

async fn filename_command(command: MessageType, filename: &String, stream: &mut TcpStream) {
    let mut send_buffer = Vec::new();
    serialize_filename(filename, &mut send_buffer);

    let res = send_message(command, &mut send_buffer, stream).await;
    if let Err(err) = res {
        eprintln!(
            "Error sending {} to node {}",
            get_type_literal(command),
            err
        );
        exit(2);
    }
}

async fn get_command(filename: &String, stream: &mut TcpStream) {
    filename_command(MessageType::CliGet, filename, stream).await
}

async fn delete_command(filename: &String, stream: &mut TcpStream) {
    filename_command(MessageType::CliDelete, filename, stream).await
}

pub async fn handle_command(command: Operation, filename: &String, stream: &mut TcpStream) {
    match command {
        Operation::New => create_command(filename, stream).await,
        Operation::Get => get_command(filename, stream).await,
        Operation::Delete => delete_command(filename, stream).await,
    }
}
