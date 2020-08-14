use std::process::{Command, Stdio};
use std::env;
use std::fs::File;
use std::io::{Write};
extern crate rand;
mod encrypter;
fn exec(command: &str) {
    let mut process = Command::new("bash").arg("-c").arg(command).stdout(Stdio::piped()).spawn().expect("failed to execute");
    let _ = process.wait();
}

fn random_bytes(n: i64) -> Vec<u8>{
    return (0..n).map( |_| {
        rand::random::<u8>()
    }).collect();
}

fn save(filename: &str, data: Vec<u8>) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&data).expect("failed to write");
    file.flush().expect("failed to flush");
}

fn first_offset() -> Vec<u8> {
    return random_bytes(173);
}

fn last_offset() -> Vec<u8> {
    return random_bytes(135);
}

fn pack(command: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = command.clone();
    result.extend(&first_offset());
    result.extend(&data);
    result.extend(&last_offset());
    return result;
}


fn encrypt(filename: &str) {
    encrypter::encrypt_and_save_key(filename);
}

fn nats_in() {
    let args: Vec<String> = env::args().collect();
    /*
    exec("head -c 173 /dev/urandom > r.txt");
    exec("head -c 135 /dev/urandom > r2.txt");
    exec(&format!("natsme {}", args[3]));
    exec(&format!("cat {} r.txt {} r2.txt > {}.dm", args[2], args[3], args[2]));
    exec("rm r.txt r2.txt");
    exec(&format!("chmod +x {}.dm", args[2]));
    */
    encrypt(&args[3]);
}

fn nats_out() {
    let args: Vec<String> = env::args().collect();
    exec(&format!("sepfile {} {}", args[2], args[3]));
    exec(&format!("natsme {}.1 {} {}", args[2], args[4], args[5]));
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "in" {
        nats_in();
    } else if args[1] == "out" {
        nats_out();
    }
}
