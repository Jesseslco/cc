// use std::fs;
// use std::path::Path;
// use std::str;

// fn rot_13_decode(content: Vec<u8>) -> Vec<u8> {
//     let mut result: Vec<u8> = Vec::new();
//     for c in content.iter() {
//         if *c >= b'a' && *c <= b'z' {
//             let mut new_c = *c - 13;
//             if new_c < b'a' {
//                 new_c = new_c + 26;
//             }
//             result.push(new_c);
//         } else if *c >= b'A' && *c <= b'Z' {
//             let mut new_c = *c - 13;
//             if new_c < b'A' {
//                 new_c = new_c + 26;
//             }
//             result.push(new_c);
//         } else {
//             result.push(*c);
//         }
//     }
//     return result;
// }

// pub fn decrypt_blob(nodefile_path: &Path) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
//     // read first 8bit to get the length of the metadata bytes
//     let node_file = fs::read(nodefile_path).unwrap();
//     let node_file = rot_13_decode(node_file);
//     let num_bytes_of_metadata = node_file[0];
//     let meta_bytes = &node_file[1..=num_bytes_of_metadata as usize];
//     let node_path = str::from_utf8(meta_bytes).unwrap();
//     println!("node_path: {}", node_path);
//     let node_file_content = &node_file[(1 + num_bytes_of_metadata) as usize..];
//     return Ok((node_path[1..].to_string(), node_file_content.to_vec()));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
