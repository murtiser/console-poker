use aead::generic_array::GenericArray;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key
};
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
};

fn get_cipher() 
-> aes_gcm::AesGcm<aes_gcm::aes::Aes256, aead::generic_array::typenum::UInt<aead::generic_array::typenum::UInt<aead::generic_array::typenum::UInt<aead::generic_array::typenum::UInt<aead::generic_array::typenum::UTerm, aead::consts::B1>, aead::consts::B1>, aead::consts::B0>, aead::consts::B0>> 
{
    match File::open("key.bin") {
        Ok(mut file) => {
            let mut file_contents: Vec<u8> = Vec::new();
            file.read_to_end(&mut file_contents).unwrap();
            let file_bytes = file_contents.as_slice();
            let key = Key::<Aes256Gcm>::from_slice(file_bytes);
            return Aes256Gcm::new(&key);
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let mut key_file = File::create("key.bin").expect("Problem with creating file");
                let key = Aes256Gcm::generate_key(OsRng);

                key_file.write(&key).unwrap();
                return Aes256Gcm::new(&key);
            }
            _ => panic!("Programm stopped cuz {error:?}"),
        },
    }
}

pub fn encrypt_money(money: &[u8]) -> Vec<u8> {
    let cipher = get_cipher();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    match OpenOptions::new().write(true).open("nonce.bin") {
        Ok(mut nonce_file) => {
            nonce_file.set_len(0).unwrap();
            nonce_file.write_all(&nonce).unwrap();
        },

        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    let mut nonce_file = File::create("nonce.bin").unwrap();
                    nonce_file.write_all(&nonce).unwrap();
                }
                _ => panic!("Problem with opening file {error:?}"),
            }
        },
    }

    cipher.encrypt(&nonce, money).unwrap()
}

pub fn decrypt_money(encr_money: &Vec<u8>) -> Vec<u8> {
    let cipher = get_cipher();
    let nonce_readed = std::fs::read("nonce.bin").unwrap();
    let nonce = GenericArray::from_slice(&nonce_readed);
    let encr_money_arr = encr_money.as_slice();

    cipher.decrypt(&nonce, encr_money_arr).unwrap()
}