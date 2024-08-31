use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpSocket, TcpStream},
};

#[repr(u8)]
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
    origin: SocketAddr,
    message_type: MessageType,
    content: String,
}

pub fn print_message(message: Message) {
    println!(
        "Received message: {} - {} from {}",
        get_type_literal(message.message_type),
        message.content,
        message.origin.is_ipv4().to_string()
    )
}

pub fn build_message_data(message_type: MessageType, content: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(message_type as u8);
    res.append(&mut content.as_bytes().to_vec());

    res
}

pub async fn send_message(message_type: MessageType, content: String, mut stream: TcpStream) -> Result<(), std::io::Error> {
    let content = build_message_data(message_type, content);
    stream.write_all(&content).await
}

pub async fn parse_message(mut stream: TcpStream, addr: SocketAddr) -> Message {
    let mut msg = Vec::new();

    stream.read_to_end(&mut msg).await.unwrap();

    if let Some(message_type) = msg.get(0) {
        return Message {
            origin: addr,
            message_type: get_type(message_type),
            content: String::from_utf8(msg[1..msg.len()].to_vec())
                .expect("Error parsing message content"),
        };
    }

    panic!("Error parsing message");
}
