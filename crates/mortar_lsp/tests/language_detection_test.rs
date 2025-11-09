#[cfg(test)]
mod language_detection_tests {
    use mortar_compiler::Language;
    use mortar_lsp::backend::{detect_system_language, parse_language_from_args};

    #[test]
    fn test_parse_language_from_args() {
        // 这个测试只能验证函数不会panic，因为我们无法轻易模拟命令行参数
        let _result = parse_language_from_args();

        // 测试系统语言检测
        let system_lang = detect_system_language();
        assert!(matches!(system_lang, Language::English | Language::Chinese));
    }

    #[test]
    fn test_environment_language_detection() {
        // 测试系统语言检测函数
        let lang = detect_system_language();
        assert!(matches!(lang, Language::English | Language::Chinese));
    }
}
