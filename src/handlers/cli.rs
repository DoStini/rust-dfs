use super::message::Message;
use crate::{
    helpers::file::deserialize_create_file,
    storage::{config::Storage, manager::StorageManager},
};

pub async fn handle_create(message: &Message, storage: &Storage) {
    let (filename, file_content) = deserialize_create_file(&message.content);
    let res = storage.store_file(&filename, &file_content).await;

    match res {
        true => println!("Sucessfully stored!"),
        false => println!("Not storedd!"),
    }
}
