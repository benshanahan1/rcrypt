//! Helper functions.

/// Convert a string into a bytes vector.
pub fn string_to_bytes(s: &String) -> Vec<u8> {
    return s.as_bytes().to_vec();
}

/// Resize key.
pub fn resize_key(mut key: Vec<u8>, new_size: usize) -> Vec<u8> {
    let key_len = key.len();
    if key_len == new_size {
        // If key and msg are same length, return immediately
        return key;
    } else if key_len > new_size {
        // If key len is greater than msg len, truncate key
        key.truncate(new_size);
    } else {
        // If key len is less than msg len, repeat key enough times to exceed
        // msg len, then truncate key
        let n_repeats = new_size / key_len + 1;
        key = key[..].repeat(n_repeats);
        key.truncate(new_size);
    }
    return key;
}

/// Encrypt a message with a given key.
///
/// # Examples
///
/// ```
/// let output = crypt::do_xor(&input_bytes, &key_bytes).unwrap();
/// ```
///
/// # Errors
///
/// The code will return an error if `message_vec` and `key_vec` are not the same length.
pub fn do_xor(message_vec: &Vec<u8>, key_vec: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    if message_vec.len() != key_vec.len() {
        return Err("message length must match key length");
    }

    let mut rv: Vec<u8> = Vec::new();

    for (mb, kb) in message_vec.iter().zip(key_vec.iter()) {
        let val = mb ^ kb;
        rv.push(val);
    }

    Ok(rv)
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
    fn test_resize_key() {
        // Size is 0
        let key = vec![1, 2, 3, 4, 5];
        assert_eq!(resize_key(key, 0), vec![]);
        // Size is less than key length
        let key = vec![1, 2, 3, 4, 5];
        assert_eq!(resize_key(key, 3), vec![1, 2, 3]);
        // Size is equal to key length
        let key = vec![1, 2, 3, 4, 5];
        assert_eq!(resize_key(key, 5), vec![1, 2, 3, 4, 5]);
        // Size is greater than key length
        let key = vec![1, 2, 3, 4, 5];
        assert_eq!(resize_key(key, 7), vec![1, 2, 3, 4, 5, 1, 2]);
        // Size is more than 2x greater than key length
        let key = vec![1, 2, 3, 4, 5];
        assert_eq!(
            resize_key(key, 12),
            vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2]
        );
    }

    #[test]
    fn test_do_xor() {
        // Test that XOR is working as expected
        let msg = vec![61, 62, 63, 64, 65, 87, 88, 89, 90];
        let key = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected = vec![60, 60, 60, 68, 68, 81, 95, 81, 83];
        let result = do_xor(&msg, &key).unwrap();
        assert_eq!(result, expected);

        // Test that error occurs when message and key length are not same
        let msg = vec![61, 62, 63, 64, 65, 87, 88, 89, 90];
        let key = vec![1, 2, 3, 4, 5];
        let result = do_xor(&msg, &key);
        assert!(result.is_err());
    }
}
