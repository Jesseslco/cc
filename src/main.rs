#[allow(dead_code)]
mod encrypt;
mod utils;
#[allow(unused_imports)]
use encrypt::encrypt_file;
#[allow(unused_imports)]
use utils::get_node_path;
use clap::Parser;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use uuid::Uuid;


fn get_uuid4() -> String {
    let uuid = Uuid::new_v4();
    return uuid.to_string();
}

fn get_node_tree_path(top_tree_path: &Path, node_path: &Path) -> Result<(), Arc<dyn Error>> {
    // let tree_name = top_tree_path.file_name().expect("failed to file name").to_str().expect("failed").to_owned();
    Ok(())
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

    // read file
    let content = fs::read(src_path).unwrap();
    let encrypt_content: Vec<u8> = encrypt_file("".to_string(), content);

    // create target folder
    let target_dir_path = dst_path.join(src_path.file_name().unwrap());
    if target_dir_path.exists() {
        println!("Target directory already exists");
        return;
    } else {
        fs::create_dir(target_dir_path).unwrap();
    }

    // create metafile dir
    let metafile_dir = dst_path.join(".meta");
    if metafile_dir.exists() {
        println!("Metafile directory already exists");
        return;
    } else {
        fs::create_dir(metafile_dir).unwrap();
    }

    // if meta
    // write content to file
}
