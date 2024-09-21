use std::{future::Future, path::Path, process::exit};

use tokio::{
    fs::{create_dir_all, remove_file, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use super::{config::Storage, errors};

pub trait StorageManager {
    fn make_path(&self, filename: &String) -> String;
    fn start_storage(max_file_size: u32, path: String) -> impl Future<Output = Storage> + Send;
    fn store_file(
        &self,
        filename: &String,
        content: &Vec<u8>,
    ) -> impl Future<Output = Result<(), errors::StorageErrors>> + Send;

    fn delete_file(
        &self,
        filename: &String,
    ) -> impl Future<Output = Result<(), errors::StorageErrors>> + Send;

    fn get_file(
        &self,
        filename: &String,
    ) -> impl Future<Output = Result<Vec<u8>, errors::StorageErrors>> + Send;
}

impl StorageManager for Storage {
    async fn start_storage(max_file_size: u32, path: String) -> Self {
        if !Path::new(&path).exists() {
            let res = create_dir_all(&path).await;
            if let Err(err) = res {
                eprintln!("Error starting storage: {err}");
                exit(3);
            }
        }

        Storage {
            max_file_size,
            path: path.clone(),
        }
    }

    fn make_path(&self, filename: &String) -> String {
        self.path.clone() + "/" + filename
    }

    async fn delete_file(&self, filename: &String) -> Result<(), errors::StorageErrors> {
        let res = remove_file(self.make_path(filename)).await;

        match res {
            Ok(_) => return Ok(()),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => return Err(errors::StorageErrors::NotFound),
                err => {
                    eprintln!("Error deleting file {}", err.to_string());
                    return Err(errors::StorageErrors::UnknownError);
                }
            },
        }
    }

    async fn store_file(
        &self,
        filename: &String,
        content: &Vec<u8>,
    ) -> Result<(), errors::StorageErrors> {
        println!("Storing in {}", self.path.clone() + "/" + filename);
        let file = File::create_new(self.make_path(filename)).await;

        if let Err(err) = file {
            eprintln!("Error creating file: {}", err);

            return Err(match err.kind() {
                std::io::ErrorKind::AlreadyExists => errors::StorageErrors::AlreadyExists,
                _ => errors::StorageErrors::UnknownError,
            });
        }

        let res = file.unwrap().write_all(&content).await;
        if let Err(err) = res {
            eprintln!("Error writing file {}", err);
            return Err(errors::StorageErrors::UnknownError);
        }

        Ok(())
    }

    async fn get_file(&self, filename: &String) -> Result<Vec<u8>, errors::StorageErrors> {
        let mut buf = vec![];
        let res = File::open(self.make_path(filename)).await;

        match res {
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => return Err(errors::StorageErrors::NotFound),
                err => {
                    eprintln!("Error getting file {}", err.to_string());
                    return Err(errors::StorageErrors::UnknownError);
                }
            },
            Ok(mut file) => match file.read_to_end(&mut buf).await {
                Ok(_) => Ok(buf),
                Err(err) => {
                    eprintln!("Error reading from file: {}", err.to_string());
                    return Err(errors::StorageErrors::UnknownError);
                }
            },
        }
    }
}
