# nats-io
<img src="./nats.png" width="500">
A program to hide file into executable binary

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
nats-io in <binary> <secret file>
```

extract secret file

```bash
nats-io out <binary that has secret data> <size of original binary> <key>
```

## LICENSE
MIT LICENSE
