// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use tauri::Manager;

use std::io::Error as IOError;
use std::{fs::File, io::Write};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Error as AesError, Nonce as AesNonce,
};

use serde::{Deserialize, Serialize};

use utils::{derive_key_from_password, read_file};

/// In production, it is crucial to use a unique, securely generated salt for each user
/// or encryption task to enhance security and mitigate the risk of attacks such as
/// rainbow table and brute-force attacks. The current implementation - is a playground.
const SALT: &str = "a9#Bf7@nS2r@1%vH34^sG8&n3k!Xz+L0pQ5!";

const ENCRYPTED_FILE_PATH: &str = "data.aes";

type Nonce = AesNonce<typenum::U12>;

#[derive(Serialize, Deserialize)]
struct EncryptionMetadata {
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl EncryptionMetadata {
    pub fn new(nonce: Nonce, ciphertext: Vec<u8>) -> Self {
        let nonce = nonce.to_vec();
        EncryptionMetadata { nonce, ciphertext }
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<(), IOError> {
        let metadata_bytes = match bincode::serialize(&self) {
            Ok(data) => data,
            Err(e) => return Err(IOError::new(std::io::ErrorKind::Other, e)),
        };

        let mut file = File::create(file_path)?;
        file.write_all(&metadata_bytes)?;

        Ok(())
    }

    pub fn read_from_file(file_path: &str) -> Result<Self, IOError> {
        let metadata_bytes = read_file(file_path)?;

        let encryption_metadata = match bincode::deserialize(&metadata_bytes) {
            Ok(metadata) => metadata,
            Err(e) => return Err(IOError::new(std::io::ErrorKind::Other, e)),
        };

        Ok(encryption_metadata)
    }
}

struct FileEncryptor;

impl FileEncryptor {
    pub fn encrypt(plaintext_file_path: &str, password: &str) -> Result<(), AesError> {
        let key = derive_key_from_password(password, SALT);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);

        let plaintext = read_file(plaintext_file_path).expect("Can't read file data");
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_slice())?;

        let encryption_metadata = EncryptionMetadata::new(nonce, ciphertext);

        // TODO: use logger
        match encryption_metadata.write_to_file(ENCRYPTED_FILE_PATH) {
            Ok(_) => println!("Successfully encrypted"),
            Err(_) => println!("Error while doing encryption"),
        }

        Ok(())
    }

    pub fn decrypt(encrypted_file_path: &str, password: &str) -> Result<(), AesError> {
        let key = derive_key_from_password(password, SALT);

        let encryption_metadata = EncryptionMetadata::read_from_file(encrypted_file_path)
            .expect("Can't read encrypted file");

        let nonce: Nonce = AesNonce::clone_from_slice(&encryption_metadata.nonce);

        let cipher = Aes256Gcm::new(&key);
        let plaintext = cipher.decrypt(&nonce, encryption_metadata.ciphertext.as_ref())?;

        dbg!(String::from_utf8(plaintext).unwrap());

        Ok(())
    }
}

#[tauri::command]
fn encrypt_file(path: &str, password: &str) {
    let _ = FileEncryptor::encrypt(path, password);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let w = app.get_webview_window("main").unwrap();
                w.open_devtools();
                w.close_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![encrypt_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
