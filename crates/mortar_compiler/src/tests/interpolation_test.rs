#[cfg(test)]
mod interpolation_tests {
    use crate::diagnostics::*;
    use crate::parser::*;

    fn create_diagnostic_collector() -> DiagnosticCollector {
        DiagnosticCollector::new("test.mortar".to_string())
    }

    fn create_test_program_with_functions(
        functions: Vec<FunctionDecl>,
        nodes: Vec<NodeDef>,
    ) -> Program {
        let mut body = Vec::new();

        for func in functions {
            body.push(TopLevel::FunctionDecl(func));
        }

        for node in nodes {
            body.push(TopLevel::NodeDef(node));
        }

        Program { body }
    }

    #[test]
    fn test_interpolated_string_function_call_analysis() {
        let mut collector = create_diagnostic_collector();

        let function = FunctionDecl {
            name: "get_name".to_string(),
            name_span: Some((0, 8)),
            params: vec![],
            return_type: Some("String".to_string()),
        };

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Hello, ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "get_name".to_string(),
                    name_span: Some((10, 18)),
                    args: vec![],
                }),
                StringPart::Text("!".to_string()),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((20, 28)),
            body: vec![NodeStmt::InterpolatedText(interpolated_string)],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should have no errors for correct function calls
        let diagnostics = collector.get_diagnostics();
        assert!(
            !diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::FunctionNotFound { .. }))
        );
    }

    #[test]
    fn test_interpolated_string_undefined_function() {
        let mut collector = create_diagnostic_collector();

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Hello, ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "undefined_function".to_string(),
                    name_span: Some((10, 28)),
                    args: vec![],
                }),
                StringPart::Text("!".to_string()),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((30, 38)),
            body: vec![NodeStmt::InterpolatedText(interpolated_string)],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![], vec![node]);
        collector.analyze_program(&program);

        // Should have error for undefined function
        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::FunctionNotFound { .. }))
        );
    }

    #[test]
    fn test_interpolated_string_argument_count_mismatch() {
        let mut collector = create_diagnostic_collector();

        let function = FunctionDecl {
            name: "greet_user".to_string(),
            name_span: Some((0, 10)),
            params: vec![Param {
                name: "name".to_string(),
                type_name: "String".to_string(),
            }],
            return_type: Some("String".to_string()),
        };

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Message: ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "greet_user".to_string(),
                    name_span: Some((15, 25)),
                    args: vec![], // Missing required argument
                }),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((30, 38)),
            body: vec![NodeStmt::InterpolatedText(interpolated_string)],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should have error for argument count mismatch
        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ArgumentCountMismatch { .. }))
        );
    }

    #[test]
    fn test_interpolated_string_argument_type_mismatch() {
        let mut collector = create_diagnostic_collector();

        let function = FunctionDecl {
            name: "format_number".to_string(),
            name_span: Some((0, 13)),
            params: vec![Param {
                name: "num".to_string(),
                type_name: "Number".to_string(),
            }],
            return_type: Some("String".to_string()),
        };

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Result: ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "format_number".to_string(),
                    name_span: Some((18, 31)),
                    args: vec![Arg::String("not_a_number".to_string())], // Wrong type
                }),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((35, 43)),
            body: vec![NodeStmt::InterpolatedText(interpolated_string)],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should have error for argument type mismatch
        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ArgumentTypeMismatch { .. }))
        );
    }

    #[test]
    fn test_interpolated_string_multiple_expressions() {
        let mut collector = create_diagnostic_collector();

        let get_name_func = FunctionDecl {
            name: "get_name".to_string(),
            name_span: Some((0, 8)),
            params: vec![],
            return_type: Some("String".to_string()),
        };

        let get_score_func = FunctionDecl {
            name: "get_score".to_string(),
            name_span: Some((10, 19)),
            params: vec![],
            return_type: Some("Number".to_string()),
        };

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Player ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "get_name".to_string(),
                    name_span: Some((25, 33)),
                    args: vec![],
                }),
                StringPart::Text(" has score ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "get_score".to_string(),
                    name_span: Some((45, 54)),
                    args: vec![],
                }),
                StringPart::Text("!".to_string()),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((60, 68)),
            body: vec![NodeStmt::InterpolatedText(interpolated_string)],
            jump: None,
        };

        let program =
            create_test_program_with_functions(vec![get_name_func, get_score_func], vec![node]);
        collector.analyze_program(&program);

        // Should have no errors for multiple valid expressions
        let diagnostics = collector.get_diagnostics();
        assert!(!diagnostics.iter().any(|d| matches!(
            d.kind,
            DiagnosticKind::FunctionNotFound { .. }
                | DiagnosticKind::ArgumentCountMismatch { .. }
                | DiagnosticKind::ArgumentTypeMismatch { .. }
        )));
    }

    #[test]
    fn test_interpolated_string_with_nested_function_call() {
        let mut collector = create_diagnostic_collector();

        let get_user_id_func = FunctionDecl {
            name: "get_user_id".to_string(),
            name_span: Some((0, 11)),
            params: vec![],
            return_type: Some("Number".to_string()),
        };

        let format_user_func = FunctionDecl {
            name: "format_user".to_string(),
            name_span: Some((15, 26)),
            params: vec![Param {
                name: "id".to_string(),
                type_name: "Number".to_string(),
            }],
            return_type: Some("String".to_string()),
        };

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Current user: ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "format_user".to_string(),
                    name_span: Some((30, 41)),
                    args: vec![Arg::FuncCall(Box::new(FuncCall {
                        name: "get_user_id".to_string(),
                        name_span: Some((42, 53)),
                        args: vec![],
                    }))],
                }),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((60, 68)),
            body: vec![NodeStmt::InterpolatedText(interpolated_string)],
            jump: None,
        };

        let program = create_test_program_with_functions(
            vec![get_user_id_func, format_user_func],
            vec![node],
        );
        collector.analyze_program(&program);

        // Should have no errors for nested function calls with correct types
        let diagnostics = collector.get_diagnostics();
        assert!(!diagnostics.iter().any(|d| matches!(
            d.kind,
            DiagnosticKind::FunctionNotFound { .. }
                | DiagnosticKind::ArgumentCountMismatch { .. }
                | DiagnosticKind::ArgumentTypeMismatch { .. }
        )));
    }

    #[test]
    fn test_mixed_text_and_interpolated_text() {
        let mut collector = create_diagnostic_collector();

        let function = FunctionDecl {
            name: "get_time".to_string(),
            name_span: Some((0, 8)),
            params: vec![],
            return_type: Some("String".to_string()),
        };

        let interpolated_string = InterpolatedString {
            parts: vec![
                StringPart::Text("Current time: ".to_string()),
                StringPart::Expression(FuncCall {
                    name: "get_time".to_string(),
                    name_span: Some((15, 23)),
                    args: vec![],
                }),
            ],
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((30, 38)),
            body: vec![
                NodeStmt::Text("This is regular text.".to_string()),
                NodeStmt::InterpolatedText(interpolated_string),
                NodeStmt::Text("This is also regular text.".to_string()),
            ],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should handle mixed text types correctly
        let diagnostics = collector.get_diagnostics();
        assert!(
            !diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::FunctionNotFound { .. }))
        );
    }
}
