use std::{future::Future, path::Path, process::exit};

use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use super::{config::Storage, errors};

pub trait StorageManager {
    fn start_storage(max_file_size: u32, path: String) -> impl Future<Output = Storage> + Send;
    fn store_file(
        &self,
        filename: &String,
        content: &Vec<u8>,
    ) -> impl Future<Output = Result<(), errors::StorageErrors>> + Send;
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

    async fn store_file(
        &self,
        filename: &String,
        content: &Vec<u8>,
    ) -> Result<(), errors::StorageErrors> {
        println!("Storing in {}", self.path.clone() + "/" + filename);
        let file = File::create_new(self.path.clone() + "/" + filename).await;

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
}
