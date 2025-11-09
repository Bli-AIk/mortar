use mortar_compiler::{Diagnostic as CompilerDiagnostic, DiagnosticCollector, Language, ParseHandler, Severity};
use tower_lsp_server::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/// Convert compiler diagnostics to LSP diagnostics
pub fn convert_diagnostics_to_lsp(
    content: &str,
    diagnostics: &[CompilerDiagnostic],
) -> Vec<Diagnostic> {
    diagnostics
        .iter()
        .map(|diag| convert_single_diagnostic(content, diag))
        .collect()
}

/// Convert a single compiler diagnostic to LSP diagnostic
fn convert_single_diagnostic(content: &str, diag: &CompilerDiagnostic) -> Diagnostic {
    let severity = match diag.severity {
        Severity::Error => Some(DiagnosticSeverity::ERROR),
        Severity::Warning => Some(DiagnosticSeverity::WARNING),
    };

    let range = if let Some((start, end)) = diag.span {
        byte_span_to_lsp_range(content, start, end)
    } else {
        Range::new(Position::new(0, 0), Position::new(0, 0))
    };

    Diagnostic {
        range,
        severity,
        code: None,
        code_description: None,
        source: Some("mortar".to_string()),
        message: diag.message.clone(),
        related_information: None,
        tags: None,
        data: None,
    }
}

/// Convert byte positions to LSP Range
fn byte_span_to_lsp_range(content: &str, start: usize, end: usize) -> Range {
    let start_pos = byte_offset_to_position(content, start);
    let end_pos = byte_offset_to_position(content, end);
    Range::new(start_pos, end_pos)
}

/// Convert byte offset to LSP Position
fn byte_offset_to_position(content: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut character = 0u32;
    let mut current_offset = 0;

    for ch in content.chars() {
        if current_offset >= offset {
            break;
        }
        
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
        
        current_offset += ch.len_utf8();
    }

    Position::new(line, character)
}

/// Parse content and get diagnostics with language support
pub fn parse_with_diagnostics(
    content: &str,
    file_name: String,
    language: Language,
) -> (Vec<Diagnostic>, Option<mortar_compiler::Program>) {
    let (parse_result, diagnostics) = ParseHandler::parse_source_code_with_diagnostics_and_language(
        content,
        file_name,
        false, // verbose_lexer
        language,
    );

    let mut lsp_diagnostics = convert_diagnostics_to_lsp(content, diagnostics.get_diagnostics());

    // Analyze the program if parsing succeeded
    let program = if let Ok(ref program) = parse_result {
        // Create a new diagnostics collector for analysis
        let mut analysis_diagnostics = DiagnosticCollector::new_with_language(
            "analysis".to_string(),
            language,
        );
        analysis_diagnostics.analyze_program(program);
        
        // Add analysis diagnostics to the result
        let analysis_lsp_diagnostics = convert_diagnostics_to_lsp(content, analysis_diagnostics.get_diagnostics());
        lsp_diagnostics.extend(analysis_lsp_diagnostics);

        Some(program.clone())
    } else {
        None
    };

    (lsp_diagnostics, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_offset_to_position() {
        let content = "Hello\nWorld";
        
        // Test start of file
        assert_eq!(byte_offset_to_position(content, 0), Position::new(0, 0));
        
        // Test position 'H' (position 0)
        assert_eq!(byte_offset_to_position(content, 0), Position::new(0, 0));
        
        // Test position 'e' (position 1)
        assert_eq!(byte_offset_to_position(content, 1), Position::new(0, 1));
        
        // Test position after newline
        assert_eq!(byte_offset_to_position(content, 6), Position::new(1, 0)); // 'W'
        
        // Test position 'o' in "World"
        assert_eq!(byte_offset_to_position(content, 7), Position::new(1, 1));
    }

    #[test]
    fn test_parse_with_diagnostics_success() {
        let content = r#"
node TestNode {
    text: "Hello World"
}
"#;
        
        let (_diagnostics, program) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::English
        );
        
        assert!(program.is_some());
        // Should have no errors, but might have warnings about naming
    }

    #[test]
    fn test_parse_with_diagnostics_with_warnings() {
        let content = r#"
node test_node {
    text: "Hello World"
}
fn unused_function();
"#;
        
        let (diagnostics, program) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::English
        );
        
        assert!(program.is_some());
        // Should have warnings for naming convention and unused function
        let warning_count = diagnostics
            .iter()
            .filter(|d| d.severity == Some(DiagnosticSeverity::WARNING))
            .count();
        assert!(warning_count >= 1);
    }

    #[test]
    fn test_language_support() {
        let content = r#"
node test_node {
    text: "Hello World"
}
"#;
        
        let (diagnostics_en, _) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::English
        );
        
        let (diagnostics_zh, _) = parse_with_diagnostics(
            content, 
            "test.mortar".to_string(), 
            Language::Chinese
        );
        
        // Both should have the same number of diagnostics
        assert_eq!(diagnostics_en.len(), diagnostics_zh.len());
        
        // But messages should be in different languages (if there are warnings)
        if !diagnostics_en.is_empty() && !diagnostics_zh.is_empty() {
            // Should have different message content for different languages
            let en_msg = &diagnostics_en[0].message;
            let zh_msg = &diagnostics_zh[0].message;
            assert_ne!(en_msg, zh_msg);
        }
    }
}