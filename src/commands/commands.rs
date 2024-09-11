use clap::ValueEnum;
use std::process::exit;
use tokio::net::TcpStream;

use crate::{
    handlers::message::{send_message, MessageType},
    helpers::file::{read_to_buf, serialize_create_file},
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
        eprintln!("Error sending message to node {err}");
        exit(2);
    }

    println!("Sucessfully sent!");
}

pub async fn handle_command(command: Operation, filename: &String, stream: &mut TcpStream) {
    match command {
        Operation::New => create_command(filename, stream),
        Operation::Get => todo!(),
        Operation::Delete => todo!(),
    }
    .await
}
