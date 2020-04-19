use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod crypt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rcrypt", about = "XOR encryption / decryption CLI.")]
struct CliOpt {
    /// Encrypt
    #[structopt(short, long)]
    encrypt: bool,

    /// Decrypt
    #[structopt(short, long)]
    decrypt: bool,

    /// Key
    #[structopt(name = "KEY")]
    key: String,

    /// Input file path
    #[structopt(name = "INPUT")]
    input: PathBuf,

    /// Output file path
    #[structopt(name = "OUTPUT")]
    output: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let opt = CliOpt::from_args();

    // Read input file into a string
    let input_file_contents = fs::read_to_string(&opt.input.as_path())
        .with_context(|_| format!("could not read file: {}", &opt.input.display()))?;

    // Convert input and key strings to bytes vectors
    let input_bytes = crypt::string_to_bytes(&input_file_contents);
    let key_bytes = crypt::string_to_bytes(&opt.key);

    // Resize key to match length of input message
    let key_bytes_resized = crypt::resize_key(key_bytes, input_bytes.len());

    // Perform XOR encryption
    let output_file_contents = crypt::do_xor(&input_bytes, &key_bytes_resized).unwrap();

    // Save output string to file
    fs::write(&opt.output.as_path(), output_file_contents)
        .with_context(|_| format!("failed to write output file: {}", &opt.output.display()))?;

    println!("Created file: {}", &opt.output.display());

    Ok(())
}
