#[allow(dead_code)]
mod encrypt;
#[allow(unused_imports)]
use encrypt::encrypt_file;
use clap::Parser;
use std::path::Path;
use std::fs;

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
    let src_file_or_directory_path = Path::new(&args.src);
    let dst_path = Path::new(&args.dst);


    // validate src_file_or_directory_path
    if !src_file_or_directory_path.exists() {
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

    if src_file_or_directory_path.is_file() {
        // read file
        let content = fs::read(src_file_or_directory_path).unwrap();
        let encrypt_content: Vec<u8> = encrypt_file("".to_string(), content);
        println!("{:?}", encrypt_content);
    }

        
    
}
