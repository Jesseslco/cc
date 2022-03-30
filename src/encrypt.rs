use std::path::PathBuf;
use std::fs;

fn encode_node_metadata(node_path: &str) -> Vec<u8> {

    let mut result: Vec<u8> = Vec::new();
    let mut node_path_bytes = node_path.as_bytes().to_vec();

    let metadata_bytes_length: u8 = node_path_bytes.len() as u8;
    println!("metadata_bytes_length: {}", node_path_bytes.len());
    println!("metadata_bytes_length: {}", &metadata_bytes_length);
    result.push(metadata_bytes_length);
    result.append(&mut node_path_bytes);
    return result;
    
}
// use rot-13 algorithm
// node path should be based on the src directory
fn encrypt_file(node_path: &str, file_content: Vec<u8>) -> Vec<u8> {
    let mut metadata = encode_node_metadata(node_path);
    let mut content = file_content;
    metadata.append(&mut content);
    let mut packed_content = metadata;
    for c in packed_content.iter_mut() {
        if *c >= b'a' && *c <= b'z' {
            *c = *c + 13;
            if *c > b'z' {
                *c = *c - 26;
            }
        } else if *c >= b'A' && *c <= b'Z' {
            *c = *c + 13;
            if *c > b'Z' {
                *c = *c - 26;
            }
        }
    }
    return packed_content;
}

fn get_node_tree_path(top_tree_path: PathBuf, node_path: &PathBuf) -> Option<String>{
    let top_tree_path_str = top_tree_path.to_str().unwrap();
    let node_path_str = node_path.to_str().unwrap();
    if node_path_str.starts_with(top_tree_path_str) {
        let node_tree_path_str = node_path_str.replace(top_tree_path_str, "");
        return Some(node_tree_path_str);
    } else {
        None
    }
}

pub fn encode_to_blob(file_path: &PathBuf, top_tree_path: PathBuf) -> Vec<u8> {
    let file_content: Vec<u8> = fs::read(&file_path).unwrap();
    let node_path = get_node_tree_path(top_tree_path, file_path).unwrap();
    let encrypted_blob = encrypt_file(&node_path, file_content);
    return encrypted_blob;
}