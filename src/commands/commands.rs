use clap::{arg, Subcommand};
use std::process::exit;
use tokio::{fs::File, io::AsyncWriteExt, net::TcpStream};

use crate::{
    handlers::message::{
        get_error_type, get_type_literal, parse_message, send_message, ErrorType, Message,
        MessageType,
    },
    helpers::file::{read_to_buf, serialize_create_file, serialize_filename},
    storage::errors::{get_storage_error_type, StorageErrors},
};

#[derive(Subcommand, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
#[repr(u8)]
pub enum Operation {
    New {
        #[arg(short = 'f', long)]
        filename: String,
    } = 1,
    Get {
        #[arg(short = 'f', long)]
        filename: String,
        #[arg(short = 'o', long)]
        output: String,
    } = 2,
    Delete {
        #[arg(short = 'f', long)]
        filename: String,
    } = 3,
}

fn validate_ack(message: &Message) -> bool {
    match message.message_type {
        MessageType::Ok => {
            println!("Operation completed successfully");
            true
        }
        MessageType::Error => {
            match get_error_type(&message.content[0]) {
                ErrorType::StorageError => match get_storage_error_type(&message.content[1]) {
                    StorageErrors::AlreadyExists => eprintln!("File already exists in the system."),
                    StorageErrors::NotFound => eprintln!("File not found in the system."),
                    StorageErrors::UnknownError => {
                        eprintln!("Something unexpected happened while processing your request.")
                    }
                },
            }
            false
        }
        _ => {
            eprintln!("Error parsing reply.");
            false
        }
    }
}

async fn wait_ack(stream: &mut TcpStream) {
    let msg = parse_message(stream).await;
    validate_ack(&msg);
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

    wait_ack(stream).await;
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

async fn get_command(filename: &String, output: &String, stream: &mut TcpStream) {
    filename_command(MessageType::CliGet, filename, stream).await;

    let get_reply = parse_message(stream).await;
    if !validate_ack(&get_reply) {
        return;
    }

    let file = File::create_new(&output).await;

    if let Err(err) = file {
        eprintln!("Error creating file: {}", err);
        exit(2);
    }

    let res = file.unwrap().write_all(&get_reply.content).await;
    if let Err(err) = res {
        eprintln!("Error writing file {}", err);
        exit(2);
    }
}

async fn delete_command(filename: &String, stream: &mut TcpStream) {
    filename_command(MessageType::CliDelete, filename, stream).await;
    wait_ack(stream).await;
}

pub async fn handle_command(command: &Operation, stream: &mut TcpStream) {
    match command {
        Operation::New { filename } => create_command(&filename, stream).await,
        Operation::Get { filename, output } => get_command(&filename, &output, stream).await,
        Operation::Delete { filename } => delete_command(&filename, stream).await,
    }
}
