type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct EncryptionChipher {
    pub chip_name: &'static str,
    pub encrypt: fn(&mut u8) -> Result<()>,
    pub decrypt: fn(&mut u8) -> Result<()>,
}

pub const ROT_13_ENCRYPTION_CHIPHER: &'static EncryptionChipher = &EncryptionChipher {
    chip_name: "rot-13",
    encrypt: encrypt_byte_rot13,
    decrypt: decrypt_byte_rot_13,
};

fn process_path_delimiter(c: &mut u8) -> () {
    if cfg!(windows) {
        if *c == b'/' {
            *c = b'\\';
        }
    } else {
        if *c == b'\\' {
            *c = b'/';
        }
    }
}

/**
 * @brief encrypt string
 * example: /home/user/src/ -> dsacxzczxczx
 * translate \ to /
 * mostly for encrypting project name
 */
pub fn simple_encrypt_string(input: &str) -> Result<String> {
    // TODO: check system type: windows or linux
    let mut str_bytes = input.bytes().collect::<Vec<u8>>();
    for c in str_bytes.iter_mut() {
        // if *c == b'/' || *c == b'\\' {
        //     process_path_delimiter(c);
        // }

        if (*c + 3) > 255 as u8 {
            *c = *c + 3 - 255;
        } else {
            *c = *c + 3;
        }
    }
    Ok(String::from_utf8(str_bytes)?)
}

/**
 * @brief decrypt string
 * example: dasdaskjckxzjc -> /home/user/src/
 */
pub fn simpledecrypt_string(input: &str) -> Result<String> {
    let mut str_bytes = input.bytes().collect::<Vec<u8>>();
    for c in str_bytes.iter_mut() {
        if (*c - 3) < 0 as u8 {
            *c = *c - 3 + 255;
        } else {
            *c = *c - 3;
        }

        // if *c == b'/' || *c == b'\\' {
        //     process_path_delimiter(c);
        // }
    }
    Ok(String::from_utf8(str_bytes)?)
}

pub fn encrypt_byte_rot13(c: &mut u8) -> Result<()> {
    if *c >= b'a' && *c <= b'z' {
        *c = *c + 13;
        if *c > b'z' {
            *c = *c - 26;
        }
    } else if *c >= b'A' && *c <= b'Z' {
        *c = *c + 13;
        if *c > b'Z' {
            *c = *c - 26;
        }
    }
    Ok(())
}

pub fn decrypt_byte_rot_13(c: &mut u8) -> Result<()> {
    if *c >= b'a' && *c <= b'z' {
        *c -= 13;
        if *c < b'a' {
            *c += 26;
        }
    } else if *c >= b'A' && *c <= b'Z' {
        *c -= 13;
        if *c < b'A' {
            *c = *c + 26;
        }
    } else {
        // do nothing
    }
    Ok(())
}

/**
 * @breif encrypt_method is swapable, this is a fucking injection example
 */
pub fn encrypt_bytes(
    mut bytes: Vec<u8>,
    encrypt_method: fn(&mut u8) -> Result<()>,
) -> Result<Vec<u8>> {
    for i in bytes.iter_mut() {
        encrypt_method(i)?;
    }
    Ok(bytes)
}

pub fn decrypt_bytes(bytes: &mut Vec<u8>, decrypt_method: fn(&mut u8) -> Result<()>) -> Result<()> {
    for i in bytes.iter_mut() {
        decrypt_method(i)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_string() {
        assert_eq!(simple_encrypt_string("abc").unwrap(), String::from("def"));
        assert_eq!(simple_encrypt_string("def").unwrap(), String::from("ghi"));

        if cfg!(windows) {
            assert_eq!(
                simple_encrypt_string(r#"\\\"#).unwrap(),
                String::from(r#"___"#)
            );
            assert_eq!(
                simple_encrypt_string(r#"///"#).unwrap(),
                String::from(r#"___"#)
            );
        } else {
            assert_eq!(
                simple_encrypt_string(r#"\\\"#).unwrap(),
                String::from(r#"222"#)
            );
            assert_eq!(
                simple_encrypt_string(r#"///"#).unwrap(),
                String::from(r#"222"#)
            );
        }
    }

    #[test]
    fn test_decrypt_string() {
        assert_eq!(simpledecrypt_string("def").unwrap(), String::from("abc"));
        assert_eq!(simpledecrypt_string("ghi").unwrap(), String::from("def"));

        if cfg!(windows) {
            assert_eq!(
                simpledecrypt_string(r#"___"#).unwrap(),
                String::from(r#"\\\"#)
            );
            assert_eq!(
                simpledecrypt_string(r#"222"#).unwrap(),
                String::from(r#"\\\"#)
            );
        } else {
            assert_eq!(
                simpledecrypt_string(r#"222"#).unwrap(),
                String::from(r#"///"#)
            );
            assert_eq!(
                simpledecrypt_string(r#"___"#).unwrap(),
                String::from(r#"///"#)
            );
        }
    }

    #[test]
    fn test_encrypt_and_decrypt_string() {
        let encrypted_str = simple_encrypt_string("abc").unwrap();
        assert_eq!(
            simpledecrypt_string(&encrypted_str).unwrap(),
            String::from("abc")
        );
    }

    #[test]
    fn test_encrypt_and_decrypt_byte() {
        let mut c = b'a';
        encrypt_byte_rot13(&mut c).unwrap();
        assert_eq!(c, b'n');
        decrypt_byte_rot_13(&mut c).unwrap();
        assert_eq!(c, b'a');

        let c = b"Abdsa///dsa".to_vec();
        let c_copy = c.clone();
        let mut encrypted_bytes = encrypt_bytes(c, encrypt_byte_rot13).unwrap();
        decrypt_bytes(&mut encrypted_bytes, decrypt_byte_rot_13).unwrap();
        assert_eq!(c_copy, encrypted_bytes);
    }
}
