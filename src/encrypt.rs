use argon2::{self};
use std::fs;
use std::str;

const SUFFIX: &str = ".fckd";
const SALT: &str = "sogoddamnsalty";

fn strip_suffix(file: &String) {
    if &file[file.len() - 5..] == SUFFIX {
        let cutoff = &file[..file.len() - 5];
        fs::rename(file, cutoff).unwrap();
    }
}

fn add_suffix(file: &String) {
    let mut new_file_name = file.clone();
    new_file_name.push_str(SUFFIX);
    fs::rename(file, new_file_name).unwrap();
}

fn get_key(password: &String) -> String {
    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), SALT.as_bytes(), &config).unwrap();
    return base64::encode(hash[32..64].to_string());
}

pub fn encrypt_files(files: Vec<String>, password: String) -> usize {
    for file in &files {
        encrypt_file(&file, &password);
    }
    return files.len();
}

fn encrypt_file(file: &String, password: &String) {
    let key = get_key(password);

    let fernet = fernet::Fernet::new(&key).unwrap();
    let ciphertext = fernet.encrypt(&fs::read(&file).unwrap());
    fs::write(&file, ciphertext).unwrap();

    add_suffix(&file);
}

pub fn decrypt_files(files: Vec<String>, password: String) -> usize {
    for file in &files {
        decrypt_file(&file, &password);
    }
    return files.len();
}

fn decrypt_file(file: &String, password: &String) {
    let key = get_key(password);

    let fernet = fernet::Fernet::new(&key).unwrap();

    match &str::from_utf8(&fs::read(&file).unwrap()) {
        Ok(_) => {
            let decrypted_plaintext = fernet.decrypt(&str::from_utf8(&fs::read(&file).unwrap()).unwrap());
            match decrypted_plaintext {
                Ok(decrypted_plaintext) => {
                    fs::write(&file, decrypted_plaintext).unwrap();
                    strip_suffix(&file);
                }
                Err(_) => {
                    eprintln!("error: invalid password for file: \'{}\'", &file);
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            eprintln!(
                "error: this file does not seem to be encrypted: \'{}\'",
                &file
            );
            std::process::exit(1);
        }
    }
}
