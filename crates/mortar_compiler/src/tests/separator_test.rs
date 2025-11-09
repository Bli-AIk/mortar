#[cfg(test)]
mod separator_tests {
    use crate::parser::{Arg, ChoiceDest, NodeStmt, ParseHandler, Program, TopLevel};

    fn parse_source(source: &str) -> Result<Program, String> {
        ParseHandler::parse_source_code(source)
    }

    #[test]
    fn test_text_statements_no_separators() {
        let source = r#"
            node TestNode {
                text: "First text"
                text: "Second text"
                text: "Third text"
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            assert_eq!(node.body.len(), 3);
            for stmt in &node.body {
                assert!(matches!(stmt, NodeStmt::Text(_)));
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_text_statements_with_semicolons() {
        let source = r#"
            node TestNode {
                text: "First text";
                text: "Second text";
                text: "Third text";
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            assert_eq!(node.body.len(), 3);
            for stmt in &node.body {
                assert!(matches!(stmt, NodeStmt::Text(_)));
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_text_statements_with_commas() {
        let source = r#"
            node TestNode {
                text: "First text",
                text: "Second text",
                text: "Third text",
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            assert_eq!(node.body.len(), 3);
            for stmt in &node.body {
                assert!(matches!(stmt, NodeStmt::Text(_)));
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_text_statements_mixed_separators() {
        let source = r#"
            node TestNode {
                text: "First text";
                text: "Second text",
                text: "Third text"
                text: "Fourth text";
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            assert_eq!(node.body.len(), 4);
            for stmt in &node.body {
                assert!(matches!(stmt, NodeStmt::Text(_)));
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_events_no_separators() {
        let source = r#"
            node TestNode {
                events: [
                    0 play_sound("test.wav")
                    1.5 set_color("red")
                    3 fade_out()
                ]
            }
            fn play_sound(file: String)
            fn set_color(color: String)
            fn fade_out()
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Events(events) = &node.body[0] {
                assert_eq!(events.len(), 3);
                assert_eq!(events[0].index, 0.0);
                assert_eq!(events[1].index, 1.5);
                assert_eq!(events[2].index, 3.0);
            } else {
                panic!("Expected Events");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_events_with_separators() {
        let source = r#"
            node TestNode {
                events: [
                    0, play_sound("test.wav");
                    1.5; set_color("red"),
                    3, fade_out();
                ]
            }
            fn play_sound(file: String)
            fn set_color(color: String)
            fn fade_out()
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Events(events) = &node.body[0] {
                assert_eq!(events.len(), 3);
                assert_eq!(events[0].index, 0.0);
                assert_eq!(events[1].index, 1.5);
                assert_eq!(events[2].index, 3.0);
            } else {
                panic!("Expected Events");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_choice_no_separators() {
        let source = r#"
            node TestNode {
                choice: [
                    "Option 1" -> next1
                    "Option 2" -> next2
                    "Option 3" -> next3
                ]
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Choice(choices) = &node.body[0] {
                assert_eq!(choices.len(), 3);
                assert_eq!(choices[0].text, "Option 1");
                assert_eq!(choices[1].text, "Option 2");
                assert_eq!(choices[2].text, "Option 3");
            } else {
                panic!("Expected Choice");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_choice_with_separators() {
        let source = r#"
            node TestNode {
                choice: [
                    "Option 1" -> next1;
                    "Option 2" -> next2,
                    "Option 3" -> next3;
                ]
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Choice(choices) = &node.body[0] {
                assert_eq!(choices.len(), 3);
                assert_eq!(choices[0].text, "Option 1");
                assert_eq!(choices[1].text, "Option 2");
                assert_eq!(choices[2].text, "Option 3");
            } else {
                panic!("Expected Choice");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_nested_choice_no_separators() {
        let source = r#"
            node TestNode {
                choice: [
                    "Main choice" -> [
                        "Sub 1" -> sub1
                        "Sub 2" -> sub2
                    ]
                    "Another choice" -> other
                ]
            }
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Choice(choices) = &node.body[0] {
                assert_eq!(choices.len(), 2);
                if let ChoiceDest::NestedChoices(nested) = &choices[0].target {
                    assert_eq!(nested.len(), 2);
                    assert_eq!(nested[0].text, "Sub 1");
                    assert_eq!(nested[1].text, "Sub 2");
                } else {
                    panic!("Expected nested choices");
                }
            } else {
                panic!("Expected Choice");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_function_params_no_separators() {
        let source = r#"
            fn test_func(param1: String param2: Number param3: Bool) -> String
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::FunctionDecl(func) = &program.body[0] {
            assert_eq!(func.params.len(), 3);
            assert_eq!(func.params[0].name, "param1");
            assert_eq!(func.params[0].type_name, "String");
            assert_eq!(func.params[1].name, "param2");
            assert_eq!(func.params[1].type_name, "Number");
            assert_eq!(func.params[2].name, "param3");
            assert_eq!(func.params[2].type_name, "Boolean");
        } else {
            panic!("Expected FunctionDecl");
        }
    }

    #[test]
    fn test_function_params_with_separators() {
        let source = r#"
            fn test_func(param1: String; param2: Number, param3: Bool) -> String
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::FunctionDecl(func) = &program.body[0] {
            assert_eq!(func.params.len(), 3);
            assert_eq!(func.params[0].name, "param1");
            assert_eq!(func.params[0].type_name, "String");
            assert_eq!(func.params[1].name, "param2");
            assert_eq!(func.params[1].type_name, "Number");
            assert_eq!(func.params[2].name, "param3");
            assert_eq!(func.params[2].type_name, "Boolean");
        } else {
            panic!("Expected FunctionDecl");
        }
    }

    #[test]
    fn test_function_call_params_no_separators() {
        let source = r#"
            node TestNode {
                events: [
                    0 some_func("arg1" "arg2" 42)
                ]
            }
            fn some_func(a: String, b: String, c: Number)
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Events(events) = &node.body[0] {
                let call = &events[0].action.call;
                assert_eq!(call.name, "some_func");
                assert_eq!(call.args.len(), 3);
                if let Arg::String(s) = &call.args[0] {
                    assert_eq!(s, "arg1");
                } else {
                    panic!("Expected string arg");
                }
                if let Arg::String(s) = &call.args[1] {
                    assert_eq!(s, "arg2");
                } else {
                    panic!("Expected string arg");
                }
                if let Arg::Number(n) = &call.args[2] {
                    assert_eq!(*n, 42.0);
                } else {
                    panic!("Expected number arg");
                }
            } else {
                panic!("Expected Events");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_function_call_params_with_separators() {
        let source = r#"
            node TestNode {
                events: [
                    0 some_func("arg1"; "arg2", 42)
                ]
            }
            fn some_func(a: String, b: String, c: Number)
        "#;

        let program = parse_source(source).unwrap();
        if let TopLevel::NodeDef(node) = &program.body[0] {
            if let NodeStmt::Events(events) = &node.body[0] {
                let call = &events[0].action.call;
                assert_eq!(call.name, "some_func");
                assert_eq!(call.args.len(), 3);
            } else {
                panic!("Expected Events");
            }
        } else {
            panic!("Expected NodeDef");
        }
    }

    #[test]
    fn test_multiple_top_level_with_separators() {
        let source = r#"
            fn func1();
            node node1 {};
            fn func2(),
            node node2 {
                text: "hello";
            };
        "#;

        let program = parse_source(source).unwrap();
        assert_eq!(program.body.len(), 4);
        assert!(matches!(program.body[0], TopLevel::FunctionDecl(_)));
        assert!(matches!(program.body[1], TopLevel::NodeDef(_)));
        assert!(matches!(program.body[2], TopLevel::FunctionDecl(_)));
        assert!(matches!(program.body[3], TopLevel::NodeDef(_)));
    }

    #[test]
    fn test_comprehensive_mixed_separators() {
        let source = r#"
            // Comment at top
            node ComplexNode {
                text: "First text";
                events: [
                    0, play_sound("sound1");
                    1.5 set_color("blue"),
                    3; fade_out()
                ];
                choice: [
                    "Option A" -> [
                        "Sub A1" -> subA1;
                        "Sub A2" -> subA2,
                    ],
                    "Option B" -> optionB;
                    "Option C" -> return
                ]
                text: "Final text",
            };
            
            function test_mixed(a: String; b: Number, c: Bool) -> Bool;
            
            fn another_func(x: String) -> String,
        "#;

        let program = parse_source(source);
        assert!(
            program.is_ok(),
            "Complex mixed separators should parse successfully"
        );
        let program = program.unwrap();
        assert_eq!(program.body.len(), 3);
    }
}
