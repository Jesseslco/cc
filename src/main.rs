#[allow(dead_code)]
mod encrypt;
use encrypt::encode_to_blob;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use uuid::Uuid;


#[allow(dead_code)]
fn get_uuid4() -> String {
    let uuid = Uuid::new_v4();
    return uuid.to_string();
}

#[allow(dead_code)]
fn get_node_tree_path(top_tree_path: &Path, node_path: &Path) -> Option<String>{
    let top_tree_path_str = top_tree_path.to_str().unwrap();
    let node_path_str = node_path.to_str().unwrap();
    if node_path_str.starts_with(top_tree_path_str) {
        let node_tree_path_str = node_path_str.replace(top_tree_path_str, "");
        return Some(node_tree_path_str);
    } else {
        None
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    src: String,

    #[clap(short, long)]
    dst: String
}


fn main() {
    let args = Args::parse();
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

    // get all file recursively in src directory
    let files_in_directory: Vec<PathBuf> = Vec::new();
    let files_in_directory: Vec<PathBuf> = list_dir(&src_path, files_in_directory);

    println!("num of files: {}", files_in_directory.len());

    if files_in_directory.len() == 0 {
        println!("No files found in source directory");
        return;
    }


    // create target directory
    let target_dir_path = dst_path.join(src_path.file_name().unwrap());
    if target_dir_path.exists() {
        println!("Target directory already exists");
        return;
    } else {
        fs::create_dir(&target_dir_path).expect("failed to create target directory");
    }
    // create .meta directory
    let meta_dir_path = target_dir_path.join(".meta");
    if meta_dir_path.exists() {
        println!("Meta directory already exists");
        return;
    } else {
        fs::create_dir(&meta_dir_path).expect("failed to create meta directory");
    }

    for file_path in files_in_directory.iter() {
        let file_blob = encode_to_blob(file_path);
        println!("file_blob: {:?}", file_path);
    }


    // for every file, encrypt it in to blob, then save it to dst_path

    // // read file
    // let content = fs::read(&src_path).unwrap();
    // let encrypt_content: Vec<u8> = encrypt_file("".to_string(), content);

    // // create target folder
    // let target_dir_path = dst_path.join(&src_path.file_name().unwrap());
    // if target_dir_path.exists() {
    //     println!("Target directory already exists");
    //     return;
    // } else {
    //     fs::create_dir(target_dir_path).unwrap();
    // }

    // // create metafile dir
    // let metafile_dir = dst_path.join(".meta");
    // if metafile_dir.exists() {
    //     println!("Metafile directory already exists");
    //     return;
    // } else {
    //     fs::create_dir(metafile_dir).unwrap();
    // }

    // if meta
    // write content to file
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