use std::fs::File;
use std::io::{Error as IOError, Read};

use aes_gcm::{Aes256Gcm, Key};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub fn derive_key_from_password(password: &str, salt: &str) -> Key<Aes256Gcm> {
    let iterations_num = 100_000;

    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        salt.as_bytes(),
        iterations_num,
        &mut key,
    );
    let key = Key::<Aes256Gcm>::from_slice(&key);

    *key
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>, IOError> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}
