use crate::common::{listdir, write_to_file};
use crate::encrypt::EncryptionChipher;
use crate::error::CommonError;
use std::error::Error;
use std::fs::File;
use std::io::{prelude, BufRead, Read};
use std::io::{BufReader, Seek};
use std::path::{Path, PathBuf};

// Global
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const META_PATH_NAME: &'static str = ".meta";

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

struct DecryptedNode {
    related_path: PathBuf,
    decrypted_content: Vec<u8>,
}

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

// pub fn encrypt_box(box_path: &Path, out_path: &Path) -> Result<()> {
//     Ok(())
// }

// encode node to single vector of u8
// fn encrypt_node(
//     file_abs_path: &Path,
//     root_abs_path: &Path,
//     encryption_chip: EncryptionChipher,
// ) -> Result<EncryptedNode> {
//     let f = File::open(file_abs_path)?;
//     // let f_reader = BufReader::new(f);

//     // f.read_buf_exact(&mut f_reader);
//     // // encode path bytes

//     // let num_node_path_bytes = .as_bytes().len() as u8;
//     let node_path: String =
// }
fn decrypt_node(node_path: &Path, encryption_chip: &EncryptionChipher) -> Result<DecryptedNode> {
    let mut f = File::open(node_path)?;
    let mut num_node_path_bytes: [u8; 1] = [0; 1];
    f.read_exact(&mut num_node_path_bytes)?;

    let mut node_path_bytes: Vec<u8> = vec![0; num_node_path_bytes[0] as usize];
    f.read_exact(&mut node_path_bytes)?;

    // file real content
    let mut node_content: Vec<u8> = Vec::new();
    f.read_to_end(&mut node_content)?;

    Ok(DecryptedNode {
        related_path: PathBuf::from(String::from_utf8(node_path_bytes)?),
        decrypted_content: node_content,
    })
}

// pub fn decrypt_box(box_path: &Path, extract_path: &Path) -> Result<()> {
//     let meta_path = box_path.join(META_PATH_NAME);
//     if !meta_path.is_dir() {
//         return Err(Box::new(CommonError::new(&format!(
//             "Meta directory does not exist: {:?}",
//             meta_path
//         ))));
//     }

//     // let box_name = match box_path.file_name() {
//     //     Some(name) => match name.to_str() {
//     //         Some(name) => name,
//     //         None => {
//     //             return Err(CommonError::InvalidPath(box_path.to_path_buf()).into());
//     //         }
//     //     },
//     //     None => return Err(CommonError::new("box name is empty").into()),
//     // };
//     Ok(())
// }

// pub fn decrypt_node(node_path: &Path, extract_path: &Path) -> Result {
//     let meta_path = node_path.join(".meta");
//     if !meta_path.is_dir() {
//         return Err(Box::new(CommonError(
//             "Meta directory does not exist".to_string(),
//         )));
//     }

//     let node_name = match node_path.file_name() {
//         Some(name) => name,
//         None => return Err(Box::new(CommonError("Node path is invalid".to_string()))),
//     };

//     let extract_node_path = extract_path.join(node_name);

//     if extract_node_path.exists() {
//         return Err(Box::new(CommonError(
//             "Node already exists in extract path".to_string(),
//         )));
//     }

//     // create extract node directory
//     fs::create_dir(&extract_node_path)?;

//     for subnode in listdir(node_path, true)?.iter() {
//         if subnode.is_dir() {
//             println!("warning: it's a dir: {:?}", subnode);
//             continue;
//         }
//         let file_bytes = fs::read(subnode).unwrap();
//         let decrypted_content = decrypt_bytes(file_bytes, |_| Ok(()))?;

//         /*
//          * extract node meta data (file path)
//          */
//         let meta_data_bytes_length = decrypted_content[0] as usize;
//         let meta_data_bytes = &decrypted_content[2..=meta_data_bytes_length];

//         let meta_data_string = String::from_utf8(meta_data_bytes.to_vec())?;
//         let meta_data_string = meta_data_string.replace("\\", "/");

//         let target_file_path = extract_node_path.join(meta_data_string);
//         println!("subnode path: {:?}", target_file_path);

//         let file_content = &decrypted_content[meta_data_bytes_length + 1..];

//         fs::create_dir_all(target_file_path.parent().unwrap())?;
//         write_to_file(&target_file_path, file_content, false).unwrap();
//     }

//     Ok(())
// }

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

    #[test]
    fn get_decrypt_node() {
        // get current dir path
        let node_path = Path::new(".")
            .canonicalize()
            .unwrap()
            .join("tests/encrypted_node.txt");
        let decrypted_node = decrypt_node(&node_path, ROT_13_ENCRYPTION_CHIPHER).unwrap();
        assert_eq!(
            decrypted_node.related_path.to_str().unwrap(),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(decrypted_node.decrypted_content[0], 65);

        if cfg!(windows) {
            assert_eq!(decrypted_node.decrypted_content[3], 10);
            assert_eq!(decrypted_node.decrypted_content.len(), 4);
        }
    }
}
