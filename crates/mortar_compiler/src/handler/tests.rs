#[cfg(test)]
mod tests {
    use crate::handler::file_handler::{FileError, FileHandler};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_file_handler_read_valid_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.mortar");
        let content = "node start { text: \"Hello\" }";

        fs::write(&test_file, content).unwrap();

        let result = FileHandler::read_source_file(test_file.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_file_handler_read_nonexistent() {
        let result = FileHandler::read_source_file("nonexistent.mortar");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_error_display() {
        let not_found_error = FileError::NotFound;
        let display = format!("{}", not_found_error);
        assert_eq!(display, "File not found");
    }

    #[test]
    fn test_file_error_debug() {
        let not_found_error = FileError::NotFound;
        let debug = format!("{:?}", not_found_error);
        assert!(debug.contains("NotFound"));
    }
}
