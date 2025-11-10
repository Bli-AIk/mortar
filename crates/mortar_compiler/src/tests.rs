#[cfg(test)]
mod tests {
    use crate::{Language, Program, Serializer};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_serialize_empty_program() {
        let program = Program { body: vec![] };
        let result = Serializer::serialize_to_json(&program, false);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("metadata"));
        assert!(json.contains("nodes"));
        assert!(json.contains("functions"));
    }

    #[test]
    fn test_serialize_to_json_pretty() {
        let program = Program { body: vec![] };
        let result = Serializer::serialize_to_json(&program, true);
        assert!(result.is_ok());

        let json = result.unwrap();
        // Pretty formatted should have indentation or newlines
        assert!(json.contains("  ") || json.contains("\n"));
    }

    #[test]
    fn test_save_to_file_basic() {
        let program = Program { body: vec![] };
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("output.mortared");

        let result = Serializer::save_to_file(&program, output_file.to_str().unwrap(), false);

        assert!(result.is_ok());
        assert!(output_file.exists());
    }

    #[test]
    fn test_save_to_file_with_language() {
        let program = Program { body: vec![] };
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("output.mortared");

        let result = Serializer::save_to_file_with_language(
            &program,
            output_file.to_str().unwrap(),
            true, // pretty
            Language::English,
        );

        assert!(result.is_ok());
        assert!(output_file.exists());
    }
}
