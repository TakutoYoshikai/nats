# nats
<img src="./nats.png" width="500">
A program to hide file into executable binary

## Requirements
* macOS or Ubuntu (Windows is not checked yet).
* Cargo
* Rust

## Install
You can install nats quickly by below command. 
```bash
cargo install --git https://github.com/TakutoYoshikai/nats.git
```

## Build
You can build nats by myself to customize parameters if you want to improve the security.
### change this constants.
```rust:main.rs
//change this
const FIRST_OFFSET_LENGTH: i64 = 173;
//change this
const LAST_OFFSET_LENGTH: i64 = 135;
```

## How to use
hide secret file
```bash
nats -e <binary> -d <secret file> -k <key file(optional)>
# It generates ./<binary>.dm file.
# It generates ./key if key file is not given.
```

extract secret file

```bash
nats -x <binary that has secret data> -s <size of original binary> -k <key file> -s <original binary size(bytes)>
```

## LICENSE
MIT LICENSE
