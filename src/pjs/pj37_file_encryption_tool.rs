use aes::Aes256;
use argon2::{Argon2, PasswordHasher, password_hash};
use cipher::{KeyIvInit, StreamCipher};
use rand::{Rng, RngExt, random};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write, stdin, stdout};
use std::{fs, io};

type Aes265Ctr = ctr::Ctr32BE<Aes256>;
pub fn aes256ctr() {
    loop {
        println!("select from bellow: \n\t1-enrypt file\n\t2-decrypt file");
        let input = get_input("option: ");
        let file_path = get_input("enter the file path: ");
        let key = get_input("enter the password you : ");
        let output_path = get_input("enter the output file path");

        match input.as_str() {
            "1" => {
                encrypt(&file_path, &key, &output_path).unwrap();
            }
            "2" => {
                 decrypt(&file_path, &key, &output_path).unwrap()
            }
            _ => continue,
        }
    }
}

fn random_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    rand::rng().fill_bytes(&mut iv);
    iv
}
fn encrypt(file: &str, key: &str, output: &str) -> io::Result<()> {
    let iv = random_iv();
    let mut pwd = [0u8; 32];
    let salt = random_iv();
    Argon2::default()
        .hash_password_into(&key.as_bytes(), &salt, &mut pwd)
        .unwrap();
    let mut cipher = Aes265Ctr::new_from_slices(&pwd, &iv).unwrap();
    let mut data = fs::read(file)?;
    cipher.apply_keystream(&mut data);

    fs::write(output, data)?;
    Ok(())
}
fn decrypt(file: &str, key: &str, output: &str) -> io::Result<()> {
    encrypt(file, key, output)
}
fn get_input(message: &str) -> String {
    print!("{message}");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}
