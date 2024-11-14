use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use crc::crc64;
use std::fs::File;
use std::io::Write;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn encrypt_data(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

pub fn decrypt_data(key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(ciphertext).unwrap()
}

pub fn crc64_check(data: &[u8]) -> u64 {
    crc64::checksum_ecma(data)
}

pub fn send_data_to_cockpit() {
    let key = [0u8; 16];
    let iv = [0u8; 16];
    let data = vec![1, 2, 3, 4]; // Örnek veri
    let encrypted_data = encrypt_data(&key, &iv, &data);
    
    let mut file_out = File::create("/serial1").expect("Cannot open serial port");
    file_out.write_all(&encrypted_data).expect("Failed to write to serial port");
}
