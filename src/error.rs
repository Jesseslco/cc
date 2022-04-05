use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CommonError(pub String);

impl CommonError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Error for CommonError {}
impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}
