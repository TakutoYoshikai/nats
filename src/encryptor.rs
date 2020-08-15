
extern crate crypto;
extern crate rand;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::option::Option;

use crypto::{ buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use rand::Rng;
use rand::OsRng;

pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Option<Vec<u8>> {
    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
        match result {
            Ok(_) => (),
            Err(_) => return None,
        }
        let result = result.unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Some(final_result)
}

fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Option<Vec<u8>> {
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
        match result {
            Ok(_) => (),
            Err(_) => return None,
        }
        let result = result.unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Some(final_result)
}

pub fn load_key(path: &str) -> ([u8; 32], [u8; 16]) {
    let mut f = File::open(path).expect("failed");
    let metadata = fs::metadata(path).expect("failed");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("failed");
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    for (i, byte) in buffer.iter().enumerate() {
        if i < 32 {
            key[i] = *byte;
        } else {
            iv[i - 32] = *byte;
        }
    }
    return (key, iv);
}

fn encrypt_file(path: &str, key: &[u8], iv: &[u8]) -> Option<Vec<u8>>{
    let f = File::open(path);
    match f {
        Ok(_) => (),
        Err(_) => return None,
    }
    let mut f = f.unwrap();
    let metadata = fs::metadata(path);
    match metadata {
        Ok(_) => (),
        Err(_) => return None,
    }
    let metadata = metadata.unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    match f.read(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    }
    let encrypted_data = encrypt(&buffer, &key, &iv);
    return encrypted_data;
}

fn decrypt_file(path: &str, key: &[u8], iv: &[u8]) -> Option<Vec<u8>>{
    let f = File::open(path);
    match f {
        Ok(_) => (),
        Err(_) => return None,
    }
    let mut f = f.unwrap();
    let metadata = fs::metadata(path);
    match metadata {
        Ok(_) => (),
        Err(_) => return None,
    }
    let metadata = metadata.unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    match f.read(&mut buffer) {
        Ok(_) => (),
        Err(_) => return None,
    }
    let decrypted_data = decrypt(&buffer, &key, &iv);
    if decrypted_data.is_none() {
        println!("{}", path);
    }
    return decrypted_data;
}

fn save_file(path: &str, data: &Vec<u8>) {
    let out = File::create(path);
    match out {
        Ok(_) => (),
        Err(_) => return,      
    }
    let out = out.unwrap();
    let mut writer = BufWriter::new(out);
    match writer.write(data) {
        Ok(_) => (),
        Err(_) => (),
    }
}


fn make_key_file(key: &[u8; 32], iv: &[u8; 16]) {
    let mut data:Vec<u8> = Vec::new();
    for (_, b) in key.iter().enumerate() {
        data.push(*b);
    }
    for (_, b) in iv.iter().enumerate() {
        data.push(*b);
    }
    save_file("./key", &data);
}

pub fn encrypt_one_file(from: &str, to: &str) {
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    let data = encrypt_file(from, &key, &iv);
    if data.is_none() {
        return;
    }
    save_file(to, &data.unwrap());
    make_key_file(&key, &iv);
}

pub fn decrypt_one_file(path: &str, keypath: &str) {
    let (key, iv) = load_key(keypath);
    let data = decrypt_file(path, &key, &iv);
    if data.is_none() {
        return;
    }
    save_file(path, &data.unwrap());
}
