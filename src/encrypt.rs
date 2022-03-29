pub fn encode_node_metadata(node_path: String) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut node_path_bytes = node_path.as_bytes().to_vec();

    let metadata_bytes_length: u8 = node_path_bytes.len() as u8;
    result.push(metadata_bytes_length);
    result.append(&mut node_path_bytes);
    return result;
    
}
// use rot-13 algorithm
// node path should be based on the src directory
pub fn encrypt_file(node_path: String, file_content: Vec<u8>) -> Vec<u8> {
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

    // let mut new_file_content = Vec::new();
    // new_file_content.append(&mut new_content);
    // let mut new_node_path = node_path;
    // new_node_path.push_str(".rot13");
    // let mut new_file = File::create(new_node_path).unwrap();
    // new_file.write_all(&new_file_content).unwrap();
}