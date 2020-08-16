use std::env;
use std::fs::File;
use std::fs;
use std::io::{Write, Read};
extern crate rand;
mod encryptor;

const FIRST_OFFSET_LENGTH: i64 = 173;
const LAST_OFFSET_LENGTH: i64 = 135;

fn random_bytes(n: i64) -> Vec<u8>{
    return (0..n).map( |_| {
        rand::random::<u8>()
    }).collect();
}

fn load(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("failed to load");
    let metadata = fs::metadata(filename).expect("failed to load");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("failed to load");
    return buffer;
}

fn save(filename: &str, data: &Vec<u8>) {
    let mut file = File::create(filename).expect("failed to create");
    file.write_all(data).expect("failed to write");
    file.flush().expect("failed to flush");
}

fn first_offset() -> Vec<u8> {
    return random_bytes(FIRST_OFFSET_LENGTH);
}

fn last_offset() -> Vec<u8> {
    return random_bytes(LAST_OFFSET_LENGTH);
}

fn pack(command: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = command.clone();
    result.extend(&first_offset());
    result.extend(data);
    result.extend(&last_offset());
    return result;
}

fn nats_in() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 5 {
        let keypath: &str = &args[4];
        let (key, iv) = encryptor::load_key(keypath);
        let mut data = load(&args[3]);
        data = encryptor::encrypt(&data, &key, &iv).unwrap();
        let command = load(&args[2]);
        let packed = pack(&command, &data);
        save(&format!("{}.dm", args[2]), &packed);
        return;
    }
    let filename: &str = &format!("{}.enc", args[3]);
    encryptor::encrypt_one_file(&args[3], filename);
    let command = load(&args[2]);
    let data = load(filename);
    let packed = pack(&command, &data);
    save(&format!("{}.dm", args[2]), &packed);
}

fn nats_out() {
    let args: Vec<String> = env::args().collect();
    let src = load(&args[2]);
    let from: i64 = args[3].parse::<i64>().unwrap() + FIRST_OFFSET_LENGTH;
    let len: i64 = (src.len() as i64) - from - LAST_OFFSET_LENGTH;
    let mut out: Vec<u8> = Vec::new();
    for i in 0..len {
        out.push(src[(from + i) as usize]);
    }
    save("nats.out", &out);
    encryptor::decrypt_one_file("nats.out", &args[4]);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "in" {
        nats_in();
    } else if args[1] == "out" {
        nats_out();
    }
}
