use std::fmt;

// Custom error types for server ops
#[derive(Debug)]
pub enum ServerError {
    IoError(std::io::Error),
    EnvError(std::env::VarError),
    ParseError(std::num::ParseIntError),
}

impl std::error::Error for ServerError {}

// Display implementation for error formatting
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::IoError(e) => write!(f, "IO Error: {}", e),
            ServerError::EnvError(e) => write!(f, "Environment Error: {}", e),
            ServerError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
} 