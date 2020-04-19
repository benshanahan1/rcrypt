use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::io::Write;
use std::process::Command; // Run programs
use tempfile::{NamedTempFile, TempDir};

#[test]
fn end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let the_message = "Hello, world!\nThis is a test file to encrypt.";
    let mut input_file = NamedTempFile::new()?;
    writeln!(input_file, "{}", &the_message)?;

    let tmp_dir = TempDir::new()?;

    // Call rcrypt on a custom message and write output to tmp file
    let enc_output_file = &tmp_dir.path().join("enc_output_file");
    let mut cmd = Command::cargo_bin("rcrypt")?;
    cmd.arg("-e")
        .arg("somekey")
        .arg(input_file.path())
        .arg(&enc_output_file.as_path());
    cmd.assert().success();

    // Check that output file was created
    assert!(&enc_output_file.as_path().exists());

    // Now, load the encrypted output file and try to decrypt it
    let dec_output_file = &tmp_dir.path().join("dec_output_file");
    let mut cmd = Command::cargo_bin("rcrypt")?;
    cmd.arg("-d")
        .arg("somekey")
        .arg(&enc_output_file.as_path())
        .arg(&dec_output_file.as_path());
    cmd.assert().success();

    // Check that output file was created & that decrypted message matches original
    assert!(&dec_output_file.as_path().exists() == &true);
    let dec_output = fs::read_to_string(&dec_output_file.as_path())?;
    assert_eq!(dec_output.trim(), the_message);

    Ok(())
}
