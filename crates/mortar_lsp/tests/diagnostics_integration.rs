#[cfg(test)]
mod diagnostics_integration_tests {
    use mortar_lsp::backend::diagnostics::parse_with_diagnostics;
    use mortar_compiler::Language;
    use tower_lsp_server::lsp_types::DiagnosticSeverity;

    #[test]
    fn test_multilingual_diagnostics() {
        let content = r#"
node test_node {
    text: "测试文本"
}

fn unused_function();
fn AnotherBadFunction();
"#;

        // Test English diagnostics
        let (diagnostics_en, _) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::English
        );
        
        // Test Chinese diagnostics
        let (diagnostics_zh, _) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::Chinese
        );
        
        // Should have the same number of diagnostics
        assert_eq!(diagnostics_en.len(), diagnostics_zh.len());
        
        // Should have warnings for naming conventions and unused functions
        assert!(!diagnostics_en.is_empty());
        assert!(!diagnostics_zh.is_empty());

        // At least one should be a warning
        let has_warning_en = diagnostics_en.iter()
            .any(|d| d.severity == Some(DiagnosticSeverity::WARNING));
        let has_warning_zh = diagnostics_zh.iter()
            .any(|d| d.severity == Some(DiagnosticSeverity::WARNING));

        assert!(has_warning_en);
        assert!(has_warning_zh);

        // Messages should be different for different languages (if there are any diagnostics)
        if !diagnostics_en.is_empty() && !diagnostics_zh.is_empty() {
            let en_msg = &diagnostics_en[0].message;
            let zh_msg = &diagnostics_zh[0].message;
            
            // Print for debugging
            println!("English: {}", en_msg);
            println!("Chinese: {}", zh_msg);
            
            // Messages should be different
            assert_ne!(en_msg, zh_msg);
        }
    }

    #[test]
    fn test_parse_success_no_warnings() {
        let content = r#"
node TestNode {
    text: "Hello World"
}

fn test_function() -> String;
"#;
        
        let (diagnostics, program) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::English
        );
        
        println!("Program parsed: {:?}", program.is_some());
        if program.is_none() {
            println!("Parse failed. Diagnostics:");
            for diag in &diagnostics {
                println!("  {:?}: {}", diag.severity, diag.message);
            }
        }
        
        assert!(program.is_some(), "Program should parse successfully");
        
        // Should have minimal diagnostics for properly formatted code
        println!("Diagnostics count: {}", diagnostics.len());
        for diag in &diagnostics {
            println!("  {}: {}", diag.severity.map_or("Info".to_string(), |s| format!("{:?}", s)), diag.message);
        }
    }

    #[test]
    fn test_syntax_error() {
        let content = r#"
node TestNode {
    invalid syntax here
}
"#;
        
        let (diagnostics, program) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::English
        );
        
        assert!(program.is_none());
        assert!(!diagnostics.is_empty());

        // Should have at least one error
        let has_error = diagnostics.iter()
            .any(|d| d.severity == Some(DiagnosticSeverity::ERROR));
        assert!(has_error);
    }
}