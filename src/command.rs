use crate::error::CommonError;
use std::error::Error;
use std::path::{Path, PathBuf};

// Global
// type Result<T> = std::result::Result<T, Box<dyn Error>>;
// const META_PATH_NAME: &'static str = ".meta";

/**
 * encrypted node structure
 * 1byte    ...byte     ...byte
 * path_len | path_byte | node content
 */

#[cfg(test)]
mod tests {}
