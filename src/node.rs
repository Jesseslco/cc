use crate::common::get_uuid4_str;
use crate::encryption::EncryptionChipher;
use crate::error::CommonError;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum NodeType {
    File,
    Folder,
}
#[derive(Debug)]
pub struct Node {
    node_path: String,
    content: Vec<u8>,
    node_type: NodeType,
}

impl Node {
    pub fn from_blob(blob_file: &Path, chipher: &'static EncryptionChipher) -> Result<Node> {
        let mut file = File::open(blob_file)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        /*
         * encrypted node also called blob is at least 3 bytes, one for header
         * one for node type
         * and at least one bytes for node path bytes
         */
        if content.len() < 3 {
            return Err(Box::new(CommonError::new(
                "invalid blob file: content bytes < 2",
            )));
        }

        for i in content.iter_mut() {
            (chipher.encrypt)(i)?;
        }

        let header: u8 = content[0];
        let node_type: u8 = content[1];
        let node_type: NodeType = match node_type {
            0 => NodeType::File,
            1 => NodeType::Folder,
            _ => {
                return Err(Box::new(CommonError::new(
                    format!("invalid node type: {}", node_type).as_str(),
                )))
            }
        };
        let node_path = String::from_utf8(content[2..=(header + 1) as usize].to_vec())?;

        match node_type {
            NodeType::File => {
                let removed_metadata = content.drain(0..(header as usize) + 2);
                drop(removed_metadata);
                return Ok(Node {
                    node_path,
                    content,
                    node_type,
                });
            }
            NodeType::Folder => {
                content.clear();
                return Ok(Node {
                    node_path,
                    content,
                    node_type,
                });
            }
        }
    }

    pub fn from_file(file_abs_path: &Path, project_abs_path: &Path) -> Result<Node> {
        println!("{}", file_abs_path.to_str().unwrap());
        println!("{}", project_abs_path.to_str().unwrap());
        let node_path = file_abs_path
            .strip_prefix(project_abs_path)
            .unwrap()
            .to_str()
            .unwrap();
        let node_type = if fs::metadata(file_abs_path)?.is_dir() {
            NodeType::Folder
        } else {
            NodeType::File
        };

        let content = match node_type {
            NodeType::File => {
                let content = fs::read(file_abs_path)?;
                content
            }
            NodeType::Folder => Vec::new(),
        };
        Ok(Node {
            node_path: node_path.to_string(),
            content,
            node_type,
        })
    }

    /**
     * basically decoding the node
     */
    pub fn save(&self, box_abs_path: &Path) -> Result<()> {
        if !box_abs_path.is_dir() {
            fs::create_dir(box_abs_path)?;
        }
        let mut file_abs_path = box_abs_path.to_path_buf();
        file_abs_path.push(self.get_localized_node_path());

        /*
         * create file parent path all if not exists
         */
        if file_abs_path.parent().is_some() {
            fs::create_dir_all(file_abs_path.parent().unwrap())?;
        } else {
            return Err(Box::new(CommonError::new("invalid node file path")));
        }

        match self.node_type {
            NodeType::File => {
                let mut file = File::create(file_abs_path)?;
                file.write_all(&self.content)?;
            }
            NodeType::Folder => fs::create_dir(file_abs_path)?,
        }
        Ok(())
    }

    fn get_localized_node_path(&self) -> PathBuf {
        if cfg!(windows) {
            PathBuf::from(self.node_path.replace("/", "\\"))
        } else {
            PathBuf::from(self.node_path.replace("\\", "/"))
        }
    }

    /**
     * basically encrypt a node
     */
    pub fn save_with_encryption(
        &self,
        box_abs_path: &Path,
        encryption_chipher: &'static EncryptionChipher,
    ) -> Result<()> {
        let mut file_abs_path = box_abs_path.to_path_buf();
        file_abs_path.push(".meta/");
        file_abs_path.push(get_uuid4_str()); // give a random file name

        if let Some(meta_path) = file_abs_path.parent() {
            if !meta_path.is_dir() {
                fs::create_dir(meta_path)?;
            }
        }

        let blob = self.encode_to_blob(encryption_chipher)?;
        let mut file = File::create(file_abs_path)?;
        file.write_all(&blob)?;

        Ok(())
    }

    fn encode_to_blob(&self, encryption_chipher: &'static EncryptionChipher) -> Result<Vec<u8>> {
        let mut blob = Vec::new();
        let node_path_bytes = self.node_path.as_bytes();
        let header = node_path_bytes.len() as u8;
        let node_type_num: u8 = match self.node_type {
            NodeType::File => 0,
            NodeType::Folder => 1,
        };
        blob.push(header);
        blob.push(node_type_num);
        blob.extend_from_slice(node_path_bytes);

        let mut file_content = self.content.clone();
        for c in file_content.iter_mut() {
            (encryption_chipher.encrypt)(c)?;
        }
        blob.append(&mut file_content);
        Ok(blob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encryption::ROT_13_ENCRYPTION_CHIPHER;

    #[test]
    fn test_node_from_encrypted_file() {
        let encrypted_blob = Path::new(".")
            .canonicalize()
            .unwrap()
            .join("tests/encrypted_node.txt");
        let node = Node::from_blob(&encrypted_blob, ROT_13_ENCRYPTION_CHIPHER).unwrap();
        assert_eq!(node.node_path.as_str(), "nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn");
        assert_eq!(node.content.len(), 4);
        assert!(matches!(node.node_type, NodeType::File));
    }

    #[test]
    fn test_node_from_file() {
        let file_abs_path = Path::new(".")
            .canonicalize()
            .unwrap()
            .join("tests/demo_folder");
        let box_abs_path = Path::new(".").canonicalize().unwrap().join("tests");
        let node = Node::from_file(&file_abs_path, &box_abs_path).unwrap();
        assert_eq!(node.node_path.as_str(), "demo_folder");
        assert!(matches!(node.node_type, NodeType::Folder));
    }
}
