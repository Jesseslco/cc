// build mod tree
mod command;
mod common;
mod decrypt;
mod encrypt;
mod error;

use encrypt::{EncryptionChipher, ROT_13_ENCRYPTION_CHIPHER};

use clap::Parser;
use std::fs;
use std::path::Path;

const ENCRYPTION_CHIPHER: [&EncryptionChipher; 1] = [ROT_13_ENCRYPTION_CHIPHER];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    command: String,

    /**
     * rot13
     */
    #[clap(short, long)]
    method: String,

    #[clap(short, long)]
    src: String,

    #[clap(short, long)]
    dst: String,
}

fn main() {
    let args = Args::parse();
    let command = args.command.as_str();
    let src_path = Path::new(&args.src);
    let dst_path = Path::new(&args.dst);

    /*
     * get the absolute path of the src directory
     */
    let src_path = if !src_path.is_absolute() {
        fs::canonicalize(src_path).expect("failed to get abs path")
    } else {
        src_path.to_owned()
    };

    /*
     * get the absolute path of the dst directory
     */
    let dst_path = if !dst_path.is_absolute() {
        fs::canonicalize(dst_path).expect("failed to get abs path")
    } else {
        dst_path.to_owned()
    };

    /*
     * src path must exist and be a directory
     */
    if !src_path.exists() {
        println!("Source file or directory does not exist");
        return;
    } else if !src_path.is_dir() {
        println!("Source file is not a directory");
        return;
    }

    /*
     * dst path must exist and be a directory
     */
    if !dst_path.exists() {
        println!("Destination directory does not exist");
        return;
    } else if !dst_path.is_dir() {
        println!("Destination path is not a directory");
        return;
    }

    let encrytion_chipher = ENCRYPTION_CHIPHER
        .iter()
        .find(|c| c.chip_name == args.method.as_str());

    let encryption_chipher = match encrytion_chipher {
        Some(c) => c,
        None => {
            println!("Encryption chipher: {} not found", args.method.as_str());
            return;
        }
    };

    match command {
        "encrypt" => {}
        "decrypt" => {}
        _ => {
            println!("Command: {} not found", command);
            return;
        }
    }
}

// fn encrypt_folder(src_path: &Path, dst_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     // get all file recursively in src directory
//     let files_in_directory: Vec<PathBuf> = Vec::new();
//     let files_in_directory: Vec<PathBuf> = list_dir(&src_path, files_in_directory);

//     println!("num of files: {}", &files_in_directory.len());

//     if files_in_directory.len() == 0 {
//         println!("No files found in source directory");
//         return Err(Box::new(WtfError(
//             "No files found in source directory".to_string(),
//         )));
//     }

//     // create target directory
//     let target_dir_path = dst_path.join(src_path.file_name().unwrap());
//     if target_dir_path.exists() {
//         println!("Target directory already exists");
//         return Err(Box::new(WtfError(
//             "Target directory already exists".to_string(),
//         )));
//     } else {
//         fs::create_dir(&target_dir_path).expect("failed to create target directory");
//     }
//     // create .meta directory
//     let meta_dir_path = target_dir_path.join(".meta");
//     if meta_dir_path.exists() {
//         println!("Meta directory already exists");
//         return Err(Box::new(WtfError(
//             "Meta directory already exists".to_string(),
//         )));
//     } else {
//         fs::create_dir(&meta_dir_path).expect("failed to create meta directory");
//     }

//     for file_path in files_in_directory.iter() {
//         let file_blob = encode_to_blob(file_path, src_path.to_owned());
//         let new_file_path = meta_dir_path.join(get_uuid4());

//         write_content_to_file(&new_file_path, &file_blob);

//         println!("file_blob: {:?}", file_path);
//     }
//     Ok(())
// }

// fn decrypt_folder(src_path: &Path, dst_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let meta_path = src_path.join(".meta");
//     if !meta_path.exists() {
//         println!("Meta directory does not exist");
//         return Err(Box::new(WtfError(
//             "Meta directory does not exist".to_string(),
//         )));
//     }

//     let project_name = src_path.file_name().unwrap().to_str().unwrap();
//     let target_dir_path = dst_path.join(project_name);

//     let all_files: Vec<PathBuf> = list_dir(&meta_path, Vec::new());

//     for file in all_files.iter() {
//         let (file_path, file_content) = decrypt_blob(file).unwrap();

//         // file absolute path
//         let file_path = target_dir_path.join(file_path);
//         println!("target_dir_path: {:?}", &target_dir_path);
//         println!("file_path: {:?}", file_path);

//         // create file parent path if not exists
//         if !file_path.parent().unwrap().exists() {
//             fs::create_dir_all(file_path.parent().unwrap())
//                 .expect("failed to create file parent directory");
//         }
//         write_content_to_file(&file_path, &file_content);
//     }
//     // let files_in_directory: Vec<PathBuf> = Vec::new();
//     // let files_in_directory: Vec<PathBuf> = list_dir(&src_path, files_in_directory);

//     // println!("num of files: {}", &files_in_directory.len());

//     // if files_in_directory.len() == 0 {
//     //     println!("No files found in source directory");
//     //     return Err(Box::new(WtfError("No files found in source directory".to_string())));
//     // }
//     Ok(())
// }
