use std::process::{Command, Stdio};
use std::env;
fn exec(command: &str) {
    let mut process = Command::new("bash").arg("-c").arg(command).stdout(Stdio::piped()).spawn().expect("failed to execute");
    let _ = process.wait();
}
fn nats_in() {
    let args: Vec<String> = env::args().collect();
    exec("head -c 173 /dev/urandom > r.txt");
    exec("head -c 135 /dev/urandom > r2.txt");
    exec(&format!("natsme {}", args[3]));
    exec(&format!("cat {} r.txt {} r2.txt > {}.dm", args[2], args[3], args[2]));
    exec("rm r.txt r2.txt");
    exec(&format!("chmod +x {}.dm", args[2]));
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
