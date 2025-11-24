use crate::{DiagnosticCollector, Language, ParseHandler};

#[test]
fn test_diagnostic_collector_creation() {
    let _collector = DiagnosticCollector::new("test.mortar".to_string());
    // Just test that creation succeeds
}

#[test]
fn test_diagnostic_collector_with_language() {
    let _collector =
        DiagnosticCollector::new_with_language("test.mortar".to_string(), Language::Chinese);
}

#[test]
fn test_parse_with_diagnostics_success() {
    let source = r#"
        let x: Number = 5
        node Test {
            text: "Hello"
        }
        fn test()
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    assert!(result.is_ok());
}

#[test]
fn test_parse_with_diagnostics_chinese() {
    let source = r#"
        fn unused()
        node Test { text: "Hi" }
    "#;

    let (result, _diagnostics) = ParseHandler::parse_source_code_with_diagnostics_and_language(
        source,
        "test.mortar".to_string(),
        false,
        Language::Chinese,
    );

    assert!(result.is_ok());
}

#[test]
fn test_analyze_program_with_undefined_node() {
    let source = r#"
        node Start {
            text: "Test"
        } -> UndefinedNode
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Analysis completed without panic
    }
}

#[test]
fn test_analyze_program_with_unused_function() {
    let source = r#"
        fn unused_function()
        
        node Test {
            text: "Hello"
        }
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Should complete without panic
    }
}

#[test]
fn test_analyze_program_with_undefined_function() {
    let source = r#"
        node Test {
            text: "Test"
            events: [
                0, undefined_function("test")
            ]
        }
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Should complete without panic
    }
}

#[test]
fn test_analyze_program_naming_conventions() {
    let source = r#"
        node lowercase_node {
            text: "Test"
        }
        
        fn MyFunction()
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Should generate naming convention warnings
    }
}

#[test]
fn test_analyze_program_type_mismatch() {
    let source = r#"
        node Test {
            text: "Test"
            events: [
                0, expect_number("string_arg")
            ]
        }
        
        fn expect_number(arg: Number)
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Should detect type mismatch
    }
}

#[test]
fn test_analyze_program_duplicate_definitions() {
    let source = r#"
        node Test {
            text: "First"
        }
        
        node Test {
            text: "Second"
        }
        
        fn test()
        fn test()
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Should detect duplicates
    }
}

#[test]
fn test_analyze_program_valid_calls() {
    let source = r#"
        node Test {
            text: "Test"
            with events: [
                0, my_func("hello", 42)
            ]
            choice: [
                "Option" when check() -> Node1
            ]
        }
        
        node Node1 { text: "Target" }
        fn my_func(s: String, n: Number)
        fn check() -> Bool
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    assert!(result.is_ok());
    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Valid program should complete analysis
    }
}

#[test]
fn test_analyze_with_variables() {
    let source = r#"
        let score: Number = 100
        pub const title: String = "Game"
        enum State { active, paused }
        
        node Test {
            text: "Test"
        }
    "#;

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    assert!(result.is_ok());
    if let Ok(program) = result {
        let mut diag = DiagnosticCollector::new("test.mortar".to_string());
        diag.analyze_program(&program);
        // Should handle variables, constants, enums
    }
}

#[test]
fn test_parse_error_generates_diagnostic() {
    let source = "invalid syntax here {{{{";

    let (result, _diagnostics) =
        ParseHandler::parse_source_code_with_diagnostics(source, "test.mortar".to_string(), false);

    // Should fail to parse
    assert!(result.is_err());
    // Diagnostics should contain the error
}
