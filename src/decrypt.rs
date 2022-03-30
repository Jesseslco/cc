use std::fs;
use std::str;
use std::path::Path;
pub fn decrypt_blob(nodefile_path: &Path) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    // read first 8bit to get the length of the metadata bytes
    let node_file = fs::read(nodefile_path)?;
    let num_bytes_of_metadata = node_file[0];
    let meta_bytes = &node_file[1..(num_bytes_of_metadata+1) as usize];
    let node_path = str::from_utf8(meta_bytes).unwrap();
    let node_file_content = &node_file[(1+num_bytes_of_metadata) as usize..];
    return Ok((node_path[1..].to_string(), node_file_content.to_vec()));
}