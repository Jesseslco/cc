#[allow(unused_imports)]
use crate::common::listdir;
use crate::error::CommonError;
use crate::node::{Node, NodeType};
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn encrypt_to_box(project_path: &Path, box_path: &Path) -> Result<()> {
    if box_path.is_dir() {
        return Err(Box::new(CommonError::new(
            format!("box path: {} exists", box_path.display()).as_str(),
        )));
    }

    // create box_path
    fs::create_dir(box_path)?;

    // create meta path
    let meta_path = box_path.join(".meta");
    fs::create_dir(&meta_path)?;

    let files = listdir(project_path, true)?;

    for file_or_dir_abs_path in files.iter() {
        let node = Node::from_file(&file_or_dir_abs_path, &project_path)?;
        println!("node: {:?}", &node);
    }
    Ok(())
}
