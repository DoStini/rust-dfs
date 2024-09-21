use std::{net::SocketAddr, process::exit, time::Duration};

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::sleep,
};

#[repr(u8)]
#[derive(Clone, Copy)]

pub enum ErrorType {
    StorageError = 1,
}

pub fn get_error_type(t: &u8) -> ErrorType {
    match t {
        1 => ErrorType::StorageError,
        _ => panic!("Invalid value: {t}"),
    }
}

#[derive(Copy, Clone)]
pub enum MessageType {
    CliPut = 1,
    CliGet = 2,
    CliDelete = 3,
    Error = 4,
    Ok = 5,
}

fn get_type(t: &u8) -> MessageType {
    match t {
        1 => MessageType::CliPut,
        2 => MessageType::CliGet,
        3 => MessageType::CliDelete,
        4 => MessageType::Error,
        5 => MessageType::Ok,
        _ => panic!("Invalid value: {t}"),
    }
}

pub fn get_type_literal(t: MessageType) -> String {
    match t {
        MessageType::CliPut => String::from("CLI_PUT"),
        MessageType::CliGet => String::from("CLI_GET"),
        MessageType::CliDelete => String::from("CLI_DELETE"),
        MessageType::Ok => String::from("OK"),
        MessageType::Error => String::from("ERROR"),
    }
}

pub struct Message {
    pub origin: SocketAddr,
    pub message_type: MessageType,
    pub content: Vec<u8>,
}

pub fn print_message(message: &Message) {
    println!(
        "Received message: {} from {}",
        get_type_literal(message.message_type),
        message.origin.ip().to_string()
    )
}

pub fn build_error_data(error_type: ErrorType, content: &mut Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(error_type as u8);
    res.append(content);

    res
}

fn build_message_data(message_type: MessageType, content: &mut Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(message_type as u8);
    res.append(content);

    res
}

pub async fn send_message(
    message_type: MessageType,
    content: &mut Vec<u8>,
    stream: &mut TcpStream,
) -> Result<(), std::io::Error> {
    let content = build_message_data(message_type, content);
    let res = stream.write_all(&content).await;

    match res {
        Err(err) => return Err(err),
        Ok(_) => stream.flush().await,
    }
}

pub async fn send_ack(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    send_message(MessageType::Ok, &mut vec![], stream).await
}

pub async fn read_message(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = vec![0; 2048];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => {
                buf.truncate(n);
                return Ok(buf);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    panic!("Reading message failed");
}

pub async fn parse_message(stream: &mut TcpStream, addr: SocketAddr) -> Message {
    let res = read_message(stream).await;

    if let Err(err) = res {
        eprintln!("Error receving message: {}", err.to_string());
        exit(2);
    }

    let msg = res.unwrap();

    if let Some(message_type) = msg.get(0) {
        return Message {
            origin: addr,
            message_type: get_type(message_type),
            content: msg[1..msg.len()].to_vec(),
        };
    }

    panic!("Error parsing message");
}
