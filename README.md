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

### Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (git checkout -b feature/AmazingFeature)
3. Commit your Changes (git commit -m 'Add some AmazingFeature')
4. Push to the Branch (git push origin feature/AmazingFeature)
5. Open a Pull Request

## LICENSE
MIT LICENSE
