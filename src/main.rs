mod crypto;

fn main() {
    let msg = String::from("Hello, world!");
    let key = String::from("secret");

    let msg_bytes = crypto::string_to_bytes(&msg);
    let key_bytes = crypto::string_to_bytes(&key);

    println!("Original key: {:?}", &key_bytes);

    let key_bytes_resized = crypto::resize_key(key_bytes, msg_bytes.len());

    println!("Resized key: {:?}", &key_bytes_resized);

    let enc_msg = crypto::do_xor(&msg_bytes, &key_bytes_resized);

    println!("Original message: {:?}", &msg_bytes);
    println!("Encrypted message: {:?}", &enc_msg);

    let dec_msg = crypto::do_xor(&enc_msg, &key_bytes_resized);

    println!("Decrypted message: {:?}", &dec_msg);
}
