use std::env;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::{Write, Read};
use dirs;
extern crate rand;
mod encryptor;
use serde::{Serialize, Deserialize};

//change this 
const FIRST_OFFSET_LENGTH: i64 = 64;
//change this 
const LAST_OFFSET_LENGTH: i64 = 64;


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    first_offset_length: i64,
    last_offset_length: i64,
}
fn config_path() -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(".nats_config.json");
    return path.to_str().unwrap().to_string();
}

fn load_config() -> (i64, i64) {
    let path: &str = &config_path();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return (FIRST_OFFSET_LENGTH, LAST_OFFSET_LENGTH),
    };
    let reader = BufReader::new(file);
    let deserialized: Config = match serde_json::from_reader(reader) {
        Ok(config) => config,
        Err(_) => return (FIRST_OFFSET_LENGTH, LAST_OFFSET_LENGTH),
    };
    return (deserialized.first_offset_length, deserialized.last_offset_length);
}

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


fn pack(command: &Vec<u8>, data: &Vec<u8>, first_offset_length: i64, last_offset_length: i64) -> Vec<u8> {
    let mut result: Vec<u8> = command.clone();
    result.extend(&random_bytes(first_offset_length));
    result.extend(data);
    result.extend(&random_bytes(last_offset_length));
    return result;
}

fn nats_in() {
    let args: Vec<String> = env::args().collect();
    let (first_offset_length, last_offset_length) = load_config();
    if args.len() >= 5 {
        let keypath: &str = &args[4];
        let (key, iv) = encryptor::load_key(keypath);
        let mut data = load(&args[3]);
        data = encryptor::encrypt(&data, &key, &iv).unwrap();
        let command = load(&args[2]);
        let packed = pack(&command, &data, first_offset_length, last_offset_length);
        save(&format!("{}.dm", args[2]), &packed);
        return;
    }
    let filename: &str = &format!("{}.enc", args[3]);
    encryptor::encrypt_one_file(&args[3], filename);
    let command = load(&args[2]);
    let data = load(filename);
    let packed = pack(&command, &data, first_offset_length, last_offset_length);
    save(&format!("{}.dm", args[2]), &packed);
}

fn nats_out() {
    let args: Vec<String> = env::args().collect();
    let (first_offset_length, last_offset_length) = load_config();
    let src = load(&args[2]);
    let from: i64 = args[3].parse::<i64>().unwrap() + first_offset_length;
    let len: i64 = (src.len() as i64) - from - last_offset_length;
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
