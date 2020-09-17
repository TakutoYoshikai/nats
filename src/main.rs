use std::env;
use std::process;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::{Write, Read};
use dirs;
extern crate rand;
mod encryptor;
use serde::{Serialize, Deserialize};
use getopts::Options;

//change this 
const FIRST_OFFSET_LENGTH: i64 = 64;
//change this 
const LAST_OFFSET_LENGTH: i64 = 64;


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    first_offset_length: i64,
    last_offset_length: i64,
}

#[derive(Debug, PartialEq)]
enum NatsMode {
    Extract,
    Embed,
}
#[derive(Debug)]
struct Args {
    mode: NatsMode,
    key: Option<String>,
    size: Option<i64>,
    binary: String,
    data: Option<String>,
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

fn nats_in(binary: &str, data: &str, keypath: Option<String>) {
    let (first_offset_length, last_offset_length) = load_config();
    if keypath != None {
        let keypath: &str = &keypath.unwrap();
        let (key, iv) = encryptor::load_key(keypath);
        let mut data = load(data);
        data = encryptor::encrypt(&data, &key, &iv).unwrap();
        let command = load(binary);
        let packed = pack(&command, &data, first_offset_length, last_offset_length);
        save(&format!("{}.dm", binary), &packed);
        return;
    }
    let filename: &str = &format!("{}.enc", data);
    encryptor::encrypt_one_file(data, filename);
    let command = load(binary);
    let data = load(filename);
    let packed = pack(&command, &data, first_offset_length, last_offset_length);
    save(&format!("{}.dm", binary), &packed);
}

fn nats_out(binary: &str, size: i64, key: &str) {
    let (first_offset_length, last_offset_length) = load_config();
    let src = load(binary);
    let from: i64 = size + first_offset_length;
    let len: i64 = (src.len() as i64) - from - last_offset_length;
    let mut out: Vec<u8> = Vec::new();
    for i in 0..len {
        out.push(src[(from + i) as usize]);
    }
    save("nats.out", &out);
    encryptor::decrypt_one_file("nats.out", key);
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn validate_args(args: &Args) -> bool {
    match args.mode {
        NatsMode::Extract => {
            return args.size != None && args.key != None;
        },
        NatsMode::Embed => {
            return args.data != None;
        }
    }
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "help");
    opts.optflag("x", "extract", "extract data from executable binary");
    opts.optflag("e", "embed", "embed data into executable binary");
    opts.optopt("k", "key", "key", "KEY");
    opts.optopt("b", "binary", "binary", "BINARY");
    opts.optopt("s", "size", "size", "SIZE");
    opts.optopt("d", "data", "data", "DATA");
    if args.len() == 1 {
        print_usage(&program, &opts);
    }
    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| panic!(f.to_string()));
    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }
    let mut mode: NatsMode = NatsMode::Embed;
    if matches.opt_present("x") {
        mode = NatsMode::Extract;
    }
    let mut size: Option<i64> = None;
    if mode == NatsMode::Extract {
        size = Some(matches.opt_str("s").unwrap().parse::<i64>().expect("it needs size(number)"));
    }
    let result: Args = Args {
        mode: mode,
        key: matches.opt_str("k"),
        size: size,
        binary: matches.opt_str("b").expect("it needs binary name"),
        data: matches.opt_str("d"),
    };
    if !validate_args(&result) {
        print_usage(&program, &opts);
    }
    return result;
}
fn main() {
    let args = parse_args();
    if args.mode == NatsMode::Extract {
        nats_out(&args.binary, args.size.unwrap(), &args.key.unwrap());
        return;
    }
    if args.mode == NatsMode::Embed {
        nats_in(&args.binary, &args.data.unwrap(), args.key); 
        return;
    }
}
