// build mod tree
mod action;
mod command;
mod common;
mod encryption;
mod error;
mod node;

use encryption::{EncryptionChipher, ROT_13_ENCRYPTION_CHIPHER};

use action::encrypt::encrypt_to_box;
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
        "encrypt" => {
            let box_path = dst_path.join(src_path.file_name().unwrap());
            encrypt_to_box(&src_path, &box_path).expect("failed to encrypt")
        }
        "decrypt" => {}
        _ => {
            println!("Command: {} not found", command);
            return;
        }
    }
}
