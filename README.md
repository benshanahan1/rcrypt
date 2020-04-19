# rcrypt

Simple XOR encryption CLI written in Rust.

## Install

Clone the repo to your machine, build, and install with `cargo`:

```bash
cargo install --path .
```

Usage:

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
