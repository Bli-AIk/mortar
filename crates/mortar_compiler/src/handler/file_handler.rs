use std::fs;
use std::io;

#[derive(Debug)]
pub enum FileError {
    IoError(io::Error),
    NotFound,
}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::IoError(error)
    }
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::IoError(err) => write!(f, "IO error: {}", err),
            FileError::NotFound => write!(f, "File not found"),
        }
    }
}

impl std::error::Error for FileError {}

/// Handle file operations
pub struct FileHandler;

impl FileHandler {
    /// Read and validate the source file
    pub fn read_source_file(path: &str) -> Result<String, FileError> {
        fs::read_to_string(path).map_err(FileError::from)
    }
}
