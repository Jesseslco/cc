use crate::error::CommonError;
use std::error::Error;
use std::path::{Path, PathBuf};

// Global
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const META_PATH_NAME: &'static str = ".meta";

pub struct DecryptedNode {
    related_path: PathBuf,
    decrypted_content: Vec<u8>,
}
/*
 * some concepts
 * box is the black box contains nodes, they are under .meta folder
 * box name is often encrypted with a simple shift algorithm
 * node is a encrypted file or a encrypted folder
 * decryption will end up with a folder
 */

/**
 * encrypted node structure
 * 1byte    ...byte     ...byte
 * path_len | path_byte | node content
 */

struct EncryptedNode {
    related_path: PathBuf,
    decrypted_content: Vec<u8>,
}

fn get_relative_path(file_abs_path: &Path, root_abs_path: &Path) -> Result<String> {
    let root_abs_path_str = match root_abs_path.to_str() {
        Some(s) => s,
        None => {
            return Err(Box::new(CommonError(
                "root_abs_path is not valid".to_string(),
            )))
        }
    };

    let file_abs_path_str = match file_abs_path.to_str() {
        Some(s) => s,
        None => {
            return Err(Box::new(CommonError(
                "file_abs_path is not valid".to_string(),
            )))
        }
    };

    if !file_abs_path.starts_with(&root_abs_path_str) {
        return Err(Box::new(CommonError(format!(
            "{} is not under root_abs_path: {}",
            file_abs_path_str, root_abs_path_str
        ))));
    }

    let relative_file_path = file_abs_path_str.replace(root_abs_path_str, "");

    Ok(relative_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encrypt::ROT_13_ENCRYPTION_CHIPHER;

    #[test]
    fn test_get_relative_path() {
        let file_abs_path = Path::new("/home/user/test/test.txt");
        let root_abs_path = Path::new("/home/user/test");
        let relative_file_path: String = get_relative_path(file_abs_path, root_abs_path).unwrap();
        assert_eq!(relative_file_path.as_str(), "/test.txt");
    }
}
