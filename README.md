# rcrypt

Simple XOR encryption CLI written in Rust.

You can use this CLI to encrypt / decrypt a file on disk. For example:

```bash
rcrypt -e "myS3cre+key!" /path/to/a /path/to/b
```

Running this command will encrypt `/path/to/a` with the secret key and place it at `/path/to/b` on disk. To decrypt:

```bash
rcrypt -d "myS3cre+key!" /path/to/b /path/to/c
```

The contents of files `/path/to/a` and `/path/to/c` should be identical.

#### Disclaimer

This tool was written as a first venture into using Rust. The encryption method employed here is poor at best and is only remotely secure if the random key provided is exactly as long as the message you are trying to encrypt. Therefore, you should probably not use this library for anything requiring actual encryption. See [this thread](https://crypto.stackexchange.com/questions/47/with-sufficient-randomness-is-xor-an-acceptable-mechanism-for-encrypting) for more information.

## Install

In order to build this code on your own machine, you need to have the [Rust compiler and package manager](https://www.rust-lang.org/tools/install) installed.

Clone the repo to your machine, build, and install with `cargo`:

```bash
cargo install --path .
```

## Usage

```
$ rcrypt --help
rcrypt 0.1.0
Benjamin Shanahan <benshanahan1@gmail.com>
XOR encryption / decryption CLI.

USAGE:
    rcrypt [FLAGS] <KEY> <INPUT> <OUTPUT>

FLAGS:
    -d, --decrypt    Decrypt
    -e, --encrypt    Encrypt
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <KEY>       Key
    <INPUT>     Input file path
    <OUTPUT>    Output file path
```

## Develop

Build and run with `cargo`:

```bash
cargo build
cargo run -- --help
```

## Test

Run unit and integration tests with `cargo`:

```bash
cargo test
```
