# rcrypt

Simple XOR encryption CLI written in Rust.

## Development

Build using `cargo`:

```bash
cargo build
```

Run with `cargo`:

```bash
cargo run -- --help
```

This will output something on the order of:

```
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

## Release

```bash
cargo build --release
```

Compiled binary is at './target/release/rcrypt'

To install the binary on your machine:

```bash
cargo install --path .
```
