use std::fs;
use std::str;
use std::path::Path;

fn rot_13_decode(content: Vec<u8>) -> Vec<u8> {
   return content; 
}

pub fn decrypt_blob(nodefile_path: &Path) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    // read first 8bit to get the length of the metadata bytes
    let node_file = fs::read(nodefile_path).unwrap();
    let node_file = rot_13_decode(node_file);
    let num_bytes_of_metadata = node_file[0];
    let meta_bytes = &node_file[1..=num_bytes_of_metadata as usize];
    let node_path = str::from_utf8(meta_bytes).unwrap();
    println!("node_path: {}", node_path);
    let node_file_content = &node_file[(1+num_bytes_of_metadata) as usize..];
    return Ok((node_path[1..].to_string(), node_file_content.to_vec()));
}