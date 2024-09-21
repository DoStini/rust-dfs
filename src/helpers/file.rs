use std::process::exit;

use tokio::{fs::File, io::AsyncReadExt};

pub async fn read_to_buf(filename: &String, buf: &mut Vec<u8>) {
    let file = File::options()
        .read(true)
        .create_new(false)
        .open(filename)
        .await;

    if let Err(err) = file {
        eprint!("Error while opening file: {err}");
        exit(3);
    }

    let res = file.unwrap().read_to_end(buf).await;
    if let Err(err) = res {
        eprint!("Error while reading file: {err}");
        exit(3);
    }
}

pub fn serialize_create_file<'a>(
    filename: &'a String,
    content: &'a mut Vec<u8>,
    output: &'a mut Vec<u8>,
) {
    serialize_filename(filename, output);
    output.push(content.len() as u8);
    output.append(content);
}

pub fn serialize_filename<'a>(filename: &'a String, output: &'a mut Vec<u8>) {
    let mut mut_filename = filename.clone();

    output.push(filename.len() as u8);
    unsafe {
        output.append(mut_filename.as_mut_vec());
    }
}

pub fn deserialize_create_file(data: &Vec<u8>) -> (String, Vec<u8>) {
    let error = "Error deserializing received message content";

    let filename_len = data.get(0).expect(&error);
    let filename = String::from_utf8(data[1..*filename_len as usize + 1].to_vec()).expect(&error);

    let data_size = data.len();
    // let content_len = data.get(*filename_len as usize + 1).expect(&error);
    let content = data[*filename_len as usize + 2..data_size].to_vec();

    (filename, content)
}

pub fn deserialize_filename_operation(data: &Vec<u8>) -> String {
    let filename = String::from_utf8(data[1..data.len()].to_vec())
        .expect("Error deserializing filename operation");
    filename
}
