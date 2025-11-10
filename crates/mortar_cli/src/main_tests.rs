#[cfg(test)]
mod tests {
    use crate::i18n::Language as CliLanguage;
    use crate::{build_command, cli_language_to_compiler_language};
    use mortar_compiler::Language;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_cli_language_conversion() {
        assert_eq!(
            cli_language_to_compiler_language(CliLanguage::English),
            Language::English
        );
        assert_eq!(
            cli_language_to_compiler_language(CliLanguage::Chinese),
            Language::Chinese
        );
    }

    #[test]
    fn test_build_command_basic() {
        let cmd = build_command(CliLanguage::English);
        assert_eq!(cmd.get_name(), "mortar");
        assert!(cmd.get_version().is_some());
    }

    #[test]
    fn test_build_command_chinese() {
        let cmd = build_command(CliLanguage::Chinese);
        assert_eq!(cmd.get_name(), "mortar");

        // Should have different help text for Chinese
        let about = cmd.get_about();
        assert!(about.is_some());
    }

    #[test]
    fn test_command_parsing() {
        let cmd = build_command(CliLanguage::English);

        // Test basic parsing with required argument
        let matches = cmd.try_get_matches_from(vec!["mortar", "test.mortar"]);
        assert!(matches.is_ok());

        let matches = matches.unwrap();
        let input = matches.get_one::<String>("input");
        assert!(input.is_some());
        assert_eq!(input.unwrap(), "test.mortar");
    }

    #[test]
    fn test_command_parsing_with_flags() {
        let cmd = build_command(CliLanguage::English);

        let matches =
            cmd.try_get_matches_from(vec!["mortar", "test.mortar", "--pretty", "--check"]);
        assert!(matches.is_ok());

        let matches = matches.unwrap();
        assert!(matches.get_flag("pretty"));
        assert!(matches.get_flag("check-only"));
    }

    #[test]
    fn test_main_workflow_components() {
        // Test individual components that main() uses
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.mortar");

        fs::write(&test_file, "node start { text: \"Hello\" }").unwrap();

        // This tests that the workflow doesn't crash
        let content = mortar_compiler::FileHandler::read_source_file(test_file.to_str().unwrap());
        assert!(content.is_ok());
        assert!(content.unwrap().contains("Hello"));
    }
}
