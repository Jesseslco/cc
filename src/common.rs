use crate::error::CommonError;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/**
 * generate a random uuid string
 */
pub fn get_uuid4_str() -> String {
    let uuid = Uuid::new_v4();
    return uuid.to_string();
}

/**
 * get file or directory path recrusively from a path recrusively get all file
 * path in a directory
 */
pub fn listdir(src_dir: &Path, if_recrusive: bool) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files_or_dirs_in_directory: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(src_dir)? {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() && if_recrusive {
            let mut sub_files_dirs = listdir(&path, if_recrusive)?;
            files_or_dirs_in_directory.append(&mut sub_files_dirs);
            files_or_dirs_in_directory.push(path);
        } else {
            // add new path to vec
            files_or_dirs_in_directory.push(path);
        }
    }

    Ok(files_or_dirs_in_directory)
}

mod tests {
    use super::*;
    #[test]
    fn test_list_dir() {
        let test_dir_path = std::env::current_dir().unwrap().join("tests/demo_folder");
        let test_dir_path = fs::canonicalize(test_dir_path).unwrap();
        let files = listdir(&test_dir_path, false).unwrap();
        assert_eq!(files.len(), 1);
        let files = listdir(&test_dir_path, true).unwrap();
        assert_eq!(files.len(), 3);
    }
}
