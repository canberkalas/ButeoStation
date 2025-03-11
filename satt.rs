use std::net::UdpSocket;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn encrypt_data(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

pub fn decrypt_data(key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(ciphertext).unwrap()
}

pub fn send_data_via_satellite(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) {
    let encrypted_data = encrypt_data(key, iv, data);
    
    let socket = UdpSocket::bind("0.0.0.0:34254").expect("Could not bind to address");
    socket.send_to(&encrypted_data, "satellite.endpoint:34254").expect("Could not send data");
}

pub fn receive_data_via_satellite(key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let socket = UdpSocket::bind("satellite.endpoint:34254").expect("Could not bind to address");
    let mut buffer = [0u8; 512];
    
    let (amt, _src) = socket.recv_from(&mut buffer).expect("Could not receive data");
    let encrypted_data = &buffer[..amt];
    
    decrypt_data(key, iv, encrypted_data)
}
