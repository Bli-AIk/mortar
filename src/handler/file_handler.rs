use owo_colors::OwoColorize;
use std::fs;

/// Handle file operations
pub struct FileHandler;

impl FileHandler {
    /// Read and validate the source file
    pub fn read_source_file(path: &str) -> Result<String, ()> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => {
                eprintln!();
                eprintln!(
                    "{} {}",
                    format!("Failed to read '{}':", path).bright_red(),
                    e
                );
                std::process::exit(1);
            }
        }
    }
}
