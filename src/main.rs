mod decrypt;
#[allow(dead_code)]
mod encrypt;
use decrypt::decrypt_blob;
use encrypt::encode_to_blob;

use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[allow(dead_code)]
fn get_uuid4() -> String {
    let uuid = Uuid::new_v4();
    return uuid.to_string();
}

#[allow(dead_code)]
fn simple_encrypt_folder_name(folder_name: &str) -> String {
    return folder_name.to_string();
}

#[allow(dead_code)]
fn simple_decrypt_foler_name(encrypted_folder_name: &str) -> String {
    return encrypted_folder_name.to_string();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    action: String,

    #[clap(short, long)]
    src: String,

    #[clap(short, long)]
    dst: String,
}

#[derive(Debug)]
struct WtfError(String);

impl std::error::Error for WtfError {
    fn description(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for WtfError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let args = Args::parse();
    let command = args.action.as_str();
    let src_path = Path::new(&args.src);
    let dst_path = Path::new(&args.dst);

    // get abs path
    let src_path = if !src_path.is_absolute() {
        fs::canonicalize(src_path).expect("failed to get abs path")
    } else {
        src_path.to_owned()
    };

    let dst_path = if !dst_path.is_absolute() {
        fs::canonicalize(dst_path).expect("failed to get abs path")
    } else {
        dst_path.to_owned()
    };

    // validate src_directory_path
    if !src_path.exists() {
        println!("Source file or directory does not exist");
        return;
    }

    // validate dst_path
    if !dst_path.exists() {
        println!("Destination directory does not exist");
        return;
    } else if !dst_path.is_dir() {
        println!("Destination path is not a directory");
        return;
    }

    if !src_path.is_dir() {
        println!("Source path is not a directory");
        return;
    }

    match command {
        "encrypt" => {
            encrypt_folder(&src_path, &dst_path).expect("failed to encrypt folder");
        }
        "decrypt" => {
            decrypt_folder(&src_path, &dst_path).expect("failed to decrypt folder");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

// get file or directory path recrusively from a path
fn list_dir(src: &Path, mut results: Vec<PathBuf>) -> Vec<PathBuf> {
    if src.is_dir() {
        for entry in fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                results = list_dir(&path, results);
            } else {
                // add new path to vec
                // println!("{:?}", &path);
                results.push(path);
            }
        }
    }
    results
}

fn write_content_to_file(file_path: &Path, content: &Vec<u8>) {
    let mut file = File::create(file_path).unwrap();
    file.write_all(content).unwrap();
}

fn encrypt_folder(src_path: &Path, dst_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // get all file recursively in src directory
    let files_in_directory: Vec<PathBuf> = Vec::new();
    let files_in_directory: Vec<PathBuf> = list_dir(&src_path, files_in_directory);

    println!("num of files: {}", &files_in_directory.len());

    if files_in_directory.len() == 0 {
        println!("No files found in source directory");
        return Err(Box::new(WtfError(
            "No files found in source directory".to_string(),
        )));
    }

    // create target directory
    let target_dir_path = dst_path.join(src_path.file_name().unwrap());
    if target_dir_path.exists() {
        println!("Target directory already exists");
        return Err(Box::new(WtfError(
            "Target directory already exists".to_string(),
        )));
    } else {
        fs::create_dir(&target_dir_path).expect("failed to create target directory");
    }
    // create .meta directory
    let meta_dir_path = target_dir_path.join(".meta");
    if meta_dir_path.exists() {
        println!("Meta directory already exists");
        return Err(Box::new(WtfError(
            "Meta directory already exists".to_string(),
        )));
    } else {
        fs::create_dir(&meta_dir_path).expect("failed to create meta directory");
    }

    for file_path in files_in_directory.iter() {
        let file_blob = encode_to_blob(file_path, src_path.to_owned());
        let new_file_path = meta_dir_path.join(get_uuid4());

        write_content_to_file(&new_file_path, &file_blob);

        println!("file_blob: {:?}", file_path);
    }
    Ok(())
}

fn decrypt_folder(src_path: &Path, dst_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let meta_path = src_path.join(".meta");
    if !meta_path.exists() {
        println!("Meta directory does not exist");
        return Err(Box::new(WtfError(
            "Meta directory does not exist".to_string(),
        )));
    }

    let project_name = src_path.file_name().unwrap().to_str().unwrap();
    let target_dir_path = dst_path.join(project_name);

    let all_files: Vec<PathBuf> = list_dir(&meta_path, Vec::new());

    for file in all_files.iter() {
        let (file_path, file_content) = decrypt_blob(file).unwrap();

        // file absolute path
        let file_path = target_dir_path.join(file_path);
        println!("target_dir_path: {:?}", &target_dir_path);
        println!("file_path: {:?}", file_path);

        // create file parent path if not exists
        if !file_path.parent().unwrap().exists() {
            fs::create_dir_all(file_path.parent().unwrap())
                .expect("failed to create file parent directory");
        }
        write_content_to_file(&file_path, &file_content);
    }
    // let files_in_directory: Vec<PathBuf> = Vec::new();
    // let files_in_directory: Vec<PathBuf> = list_dir(&src_path, files_in_directory);

    // println!("num of files: {}", &files_in_directory.len());

    // if files_in_directory.len() == 0 {
    //     println!("No files found in source directory");
    //     return Err(Box::new(WtfError("No files found in source directory".to_string())));
    // }
    Ok(())
}
