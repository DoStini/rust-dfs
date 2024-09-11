use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MessageType {
    CliPut = 1,
    CliGet = 2,
    CliDelete = 3,
}

fn get_type(t: &u8) -> MessageType {
    match t {
        1 => MessageType::CliPut,
        2 => MessageType::CliGet,
        3 => MessageType::CliDelete,
        _ => panic!("Invalid value: {t}"),
    }
}

fn get_type_literal(t: MessageType) -> String {
    match t {
        MessageType::CliPut => String::from("CLI_PUT"),
        MessageType::CliGet => String::from("CLI_GET"),
        MessageType::CliDelete => String::from("CLI_DELETE"),
    }
}

pub struct Message {
    pub origin: SocketAddr,
    pub message_type: MessageType,
    pub content: Vec<u8>,
}

pub fn print_message(message: &Message) {
    println!(
        "Received message: {} - {} from {}",
        get_type_literal(message.message_type),
        String::from_utf8(message.content.clone()).expect("Error parsing message content"),
        message.origin.ip().to_string()
    )
}

pub fn build_message_data(message_type: MessageType, content: &mut Vec<u8>) -> Vec<u8> {
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
    stream.write_all(&content).await
}

pub async fn parse_message(stream: &mut TcpStream, addr: SocketAddr) -> Message {
    let mut msg = Vec::new();

    stream.read_to_end(&mut msg).await.unwrap();

    if let Some(message_type) = msg.get(0) {
        return Message {
            origin: addr,
            message_type: get_type(message_type),
            content: msg[1..msg.len()].to_vec(),
        };
    }

    panic!("Error parsing message");
}
