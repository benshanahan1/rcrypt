use std::path::PathBuf;
use std::io;
use failure::ResultExt;
use exitfailure::ExitFailure;

/// Convert a string into a bytes vector.
pub fn string_to_bytes(s: &String) -> Vec<u8> {
    return s.as_bytes().to_vec();
}

/// Resize key.
pub fn resize_key(mut key: Vec<u8>, msg_len: usize) -> Vec<u8> {
    let key_len = key.len();
    if key_len == msg_len {
        // If key and msg are same length, return immediately
        return key;
    } else if key_len > msg_len {
        // If key len is greater than msg len, truncate key
        key.truncate(msg_len);
    } else {
        // If key len is less than msg len, repeat key enough times to exceed msg len, then
        // truncate key
        let n_repeats = msg_len / key_len + 1;
        for _ in 0..n_repeats {
            let mut key_copy = key.to_vec();
            key.append(&mut key_copy);
        }
        key.truncate(msg_len);
    }
    return key;
}

/// Encrypt a message with a given key.
pub fn do_xor(msg_vec: &Vec<u8>, key_vec: &Vec<u8>) -> Vec<u8> {
    let mut enc_vec: Vec<u8> = Vec::new();

    for (mb, kb) in msg_vec.iter().zip(key_vec.iter()) {
        let val = mb ^ kb;
        enc_vec.push(val);
    }

    return enc_vec;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_to_bytes() {
        let s = String::from("hello");
        let expected: Vec<u8> = vec![104, 101, 108, 108, 111];
        assert_eq!(string_to_bytes(&s), expected);
    }

    #[test]
    fn test_resize_key() {}

    #[test]
    fn test_do_xor() {}
}
