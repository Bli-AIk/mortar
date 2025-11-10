use mortar_compiler::{
    Diagnostic as CompilerDiagnostic, DiagnosticCollector, Language, ParseHandler, Severity,
};
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
        content, file_name, false, // verbose_lexer
        language,
    );

    let mut lsp_diagnostics = convert_diagnostics_to_lsp(content, diagnostics.get_diagnostics());

    // Analyze the program if parsing succeeded
    let program = if let Ok(ref program) = parse_result {
        // Create a new diagnostics collector for analysis
        let mut analysis_diagnostics =
            DiagnosticCollector::new_with_language("analysis".to_string(), language);
        analysis_diagnostics.analyze_program(program);

        // Add analysis diagnostics to the result
        let analysis_lsp_diagnostics =
            convert_diagnostics_to_lsp(content, analysis_diagnostics.get_diagnostics());
        lsp_diagnostics.extend(analysis_lsp_diagnostics);

        Some(program.clone())
    } else {
        None
    };

    (lsp_diagnostics, program)
}
