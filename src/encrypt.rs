use std::path::PathBuf;
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
        if *c == b'/' || *c == b'\\' {
            process_path_delimiter(c);
        }

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

        if *c == b'/' || *c == b'\\' {
            process_path_delimiter(c);
        }
    }
    Ok(String::from_utf8(str_bytes)?)
}

// fn encode_node_metadata(node_path: &str) -> Vec<u8> {
//     let mut result: Vec<u8> = Vec::new();
//     let mut node_path_bytes = node_path.as_bytes().to_vec();

//     let metadata_bytes_length: u8 = node_path_bytes.len() as u8;
//     println!("metadata_bytes_length: {}", node_path_bytes.len());
//     println!("metadata_bytes_length: {}", &metadata_bytes_length);
//     result.push(metadata_bytes_length);
//     result.append(&mut node_path_bytes);
//     return result;
// }

// // use rot-13 algorithm
// // node path should be based on the src directory
// fn encrypt_file(node_path: &str, file_content: Vec<u8>) -> Vec<u8> {
//     let mut metadata = encode_node_metadata(node_path);
//     let mut content = file_content;
//     metadata.append(&mut content);
//     let mut packed_content = metadata;
//     for c in packed_content.iter_mut() {}
//     return packed_content;
// }

// fn get_node_tree_path(top_tree_path: PathBuf, node_path: &PathBuf) -> Option<String> {
//     let top_tree_path_str = top_tree_path.to_str().unwrap();
//     let node_path_str = node_path.to_str().unwrap();
//     if node_path_str.starts_with(top_tree_path_str) {
//         let node_tree_path_str = node_path_str.replace(top_tree_path_str, "");
//         return Some(node_tree_path_str);
//     } else {
//         None
//     }
// }

// pub fn encode_to_blob(file_path: &PathBuf, top_tree_path: PathBuf) -> Vec<u8> {
//     let file_content: Vec<u8> = fs::read(&file_path).unwrap();
//     let node_path = get_node_tree_path(top_tree_path, file_path).unwrap();
//     let encrypted_blob = encrypt_file(&node_path, file_content);
//     return encrypted_blob;
// }

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

pub fn decrypt_bytes(
    mut bytes: Vec<u8>,
    decrypt_method: fn(&mut u8) -> Result<()>,
) -> Result<Vec<u8>> {
    for i in bytes.iter_mut() {
        decrypt_method(i)?;
    }
    Ok(bytes)
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
        let encrypted_bytes = encrypt_bytes(c, encrypt_byte_rot13).unwrap();
        let decrypted_bytes = decrypt_bytes(encrypted_bytes, decrypt_byte_rot_13).unwrap();
        assert_eq!(c_copy, decrypted_bytes);
    }

    // #[test]
    // fn wtf_test() {
    //     use std::fs;
    //     let current_dir = fs::canonicalize(".").unwrap();
    //     let encrypted_file_bytes = fs::read(current_dir.join("tests/encrypted_node_2")).unwrap();
    //     assert_eq!(encrypted_file_bytes[0], (41 as u8));
    //     let decrypted_file_bytes = decrypt_bytes(encrypted_file_bytes, decrypt_byte_rot_13).unwrap();

    //     let decrypted_path = String::from_utf8(decrypted_file_bytes[1..=41].to_vec()).unwrap();
    //     assert_eq!("dsad", &decrypted_path);
    // }
}
