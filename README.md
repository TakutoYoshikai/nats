# nats
<img src="./nats.png" width="500">
A program to hide file into executable binary

## Install
```bash
cargo install --git https://github.com/TakutoYoshikai/nats.git
```

## Build
### change this constants.
```rust:main.rs
//change this
const FIRST_OFFSET_LENGTH: i64 = 173;
//change this
const LAST_OFFSET_LENGTH: i64 = 135;
```

### release build
```bash
cargo build --release
```

## How to use
hide secret file
```bash
nats -e -b <binary> -d <secret file>
```

extract secret file

```bash
nats -x -b <binary that has secret data> -s <size of original binary> -k <key>
```

## LICENSE
MIT LICENSE
