#[cfg(test)]
mod type_checking_tests {
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
    fn test_function_argument_count_mismatch() {
        let mut collector = create_diagnostic_collector();

        // Function expects 2 parameters, but call provides 1
        let function = FunctionDecl {
            name: "test_func".to_string(),
            name_span: Some((0, 9)),
            params: vec![
                Param {
                    name: "param1".to_string(),
                    type_name: "String".to_string(),
                },
                Param {
                    name: "param2".to_string(),
                    type_name: "Number".to_string(),
                },
            ],
            return_type: None,
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((10, 18)),
            body: vec![NodeStmt::Events(vec![Event {
                index: 0.0,
                action: EventAction {
                    call: FuncCall {
                        name: "test_func".to_string(),
                        name_span: Some((20, 29)),
                        args: vec![Arg::String("hello".to_string())], // Only 1 arg
                    },
                    chains: vec![],
                },
            }])],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ArgumentCountMismatch { .. }))
        );
    }

    #[test]
    fn test_function_argument_type_mismatch() {
        let mut collector = create_diagnostic_collector();

        // Function expects String, but call provides Number
        let function = FunctionDecl {
            name: "test_func".to_string(),
            name_span: Some((0, 9)),
            params: vec![Param {
                name: "param1".to_string(),
                type_name: "String".to_string(),
            }],
            return_type: None,
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((10, 18)),
            body: vec![NodeStmt::Events(vec![Event {
                index: 0.0,
                action: EventAction {
                    call: FuncCall {
                        name: "test_func".to_string(),
                        name_span: Some((20, 29)),
                        args: vec![Arg::Number(42.0)], // Wrong type
                    },
                    chains: vec![],
                },
            }])],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ArgumentTypeMismatch { .. }))
        );
    }

    #[test]
    fn test_when_condition_type_mismatch() {
        let mut collector = create_diagnostic_collector();

        // Function returns String, but when condition expects Boolean
        let function = FunctionDecl {
            name: "get_name".to_string(),
            name_span: Some((0, 8)),
            params: vec![],
            return_type: Some("String".to_string()), // Wrong return type for condition
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((10, 18)),
            body: vec![NodeStmt::Choice(vec![ChoiceItem {
                text: "Test choice".to_string(),
                condition: Some(Condition::FuncCall(FuncCall {
                    name: "get_name".to_string(),
                    name_span: Some((20, 28)),
                    args: vec![],
                })),
                target: ChoiceDest::Return,
            }])],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ConditionTypeMismatch { .. }))
        );
    }

    #[test]
    fn test_valid_boolean_when_condition() {
        let mut collector = create_diagnostic_collector();

        // Function returns Boolean - should be valid
        let function = FunctionDecl {
            name: "has_item".to_string(),
            name_span: Some((0, 8)),
            params: vec![],
            return_type: Some("Boolean".to_string()),
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((10, 18)),
            body: vec![NodeStmt::Choice(vec![ChoiceItem {
                text: "Test choice".to_string(),
                condition: Some(Condition::FuncCall(FuncCall {
                    name: "has_item".to_string(),
                    name_span: Some((20, 28)),
                    args: vec![],
                })),
                target: ChoiceDest::Return,
            }])],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should not have condition type errors (may have other errors like unused functions)
        let diagnostics = collector.get_diagnostics();
        assert!(
            !diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ConditionTypeMismatch { .. }))
        );
    }

    #[test]
    fn test_bool_type_alias_compatibility() {
        let mut collector = create_diagnostic_collector();

        // Function returns Bool (alias) - should be valid for Boolean condition
        let function = FunctionDecl {
            name: "is_ready".to_string(),
            name_span: Some((0, 8)),
            params: vec![],
            return_type: Some("Bool".to_string()), // Using Bool alias
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((10, 18)),
            body: vec![NodeStmt::Choice(vec![ChoiceItem {
                text: "Test choice".to_string(),
                condition: Some(Condition::FuncCall(FuncCall {
                    name: "is_ready".to_string(),
                    name_span: Some((20, 28)),
                    args: vec![],
                })),
                target: ChoiceDest::Return,
            }])],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should not have condition type errors
        let diagnostics = collector.get_diagnostics();
        assert!(
            !diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ConditionTypeMismatch { .. }))
        );
    }

    #[test]
    fn test_nested_function_call_type_checking() {
        let mut collector = create_diagnostic_collector();

        // Function that takes a string parameter
        let outer_function = FunctionDecl {
            name: "process".to_string(),
            name_span: Some((0, 7)),
            params: vec![Param {
                name: "input".to_string(),
                type_name: "String".to_string(),
            }],
            return_type: None,
        };

        // Function that returns a number (incompatible)
        let inner_function = FunctionDecl {
            name: "get_number".to_string(),
            name_span: Some((8, 18)),
            params: vec![],
            return_type: Some("Number".to_string()),
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((20, 28)),
            body: vec![NodeStmt::Events(vec![Event {
                index: 0.0,
                action: EventAction {
                    call: FuncCall {
                        name: "process".to_string(),
                        name_span: Some((30, 37)),
                        args: vec![Arg::FuncCall(Box::new(FuncCall {
                            name: "get_number".to_string(),
                            name_span: Some((38, 48)),
                            args: vec![],
                        }))],
                    },
                    chains: vec![],
                },
            }])],
            jump: None,
        };

        let program =
            create_test_program_with_functions(vec![outer_function, inner_function], vec![node]);
        collector.analyze_program(&program);

        assert!(collector.has_errors());
        let diagnostics = collector.get_diagnostics();
        assert!(
            diagnostics
                .iter()
                .any(|d| matches!(d.kind, DiagnosticKind::ArgumentTypeMismatch { .. }))
        );
    }

    #[test]
    fn test_correct_types_no_errors() {
        let mut collector = create_diagnostic_collector();

        let function = FunctionDecl {
            name: "process".to_string(),
            name_span: Some((0, 7)),
            params: vec![
                Param {
                    name: "text".to_string(),
                    type_name: "String".to_string(),
                },
                Param {
                    name: "count".to_string(),
                    type_name: "Number".to_string(),
                },
            ],
            return_type: None,
        };

        let node = NodeDef {
            name: "TestNode".to_string(),
            name_span: Some((10, 18)),
            body: vec![NodeStmt::Events(vec![Event {
                index: 0.0,
                action: EventAction {
                    call: FuncCall {
                        name: "process".to_string(),
                        name_span: Some((20, 27)),
                        args: vec![Arg::String("hello".to_string()), Arg::Number(42.0)],
                    },
                    chains: vec![],
                },
            }])],
            jump: None,
        };

        let program = create_test_program_with_functions(vec![function], vec![node]);
        collector.analyze_program(&program);

        // Should not have type mismatch errors (may have other warnings like unused functions)
        let diagnostics = collector.get_diagnostics();
        assert!(!diagnostics.iter().any(|d| matches!(
            d.kind,
            DiagnosticKind::ArgumentTypeMismatch { .. }
                | DiagnosticKind::ArgumentCountMismatch { .. }
                | DiagnosticKind::ConditionTypeMismatch { .. }
        )));
    }
}
