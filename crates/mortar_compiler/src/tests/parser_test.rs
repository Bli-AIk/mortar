use crate::parser::{
    Arg, ChoiceDest, ChoiceItem, Condition, Event, EventAction, FuncCall, FunctionDecl, NodeDef,
    NodeJump, NodeStmt, Param, ParseHandler, Program, TopLevel, WithEventItem, WithEventsStmt,
};

fn check_parsing(source: &str, expected_program: Program) {
    let program = ParseHandler::parse_source_code(source, false).unwrap();
    assert_eq!(program, expected_program);
}

#[test]
fn test_parse_empty_source() {
    let source = "";
    let program = ParseHandler::parse_source_code(source, false).unwrap();
    assert_eq!(program.body.len(), 0);
}

#[test]
fn test_parse_node_def() {
    let source = r#"
        node start_node {
            text: "Hello, world!"
        } -> next_node
    "#;
    let expected = Program {
        body: vec![TopLevel::NodeDef(NodeDef {
            name: "start_node".to_string(),
            name_span: Some((14, 24)), // Approximate span for "start_node"
            body: vec![NodeStmt::Text("Hello, world!".to_string())],
            jump: Some(NodeJump::Identifier(
                "next_node".to_string(),
                Some((74, 83)),
            )), // Updated to actual span
        })],
    };
    check_parsing(source, expected);
}

#[test]
fn test_parse_function_decl() {
    let source = r#"
        fn my_function(param1: String, param2: Number) -> String
    "#;
    let expected = Program {
        body: vec![TopLevel::FunctionDecl(FunctionDecl {
            name: "my_function".to_string(),
            name_span: Some((12, 23)), // Updated to actual span for "my_function"
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
            return_type: Some("String".to_string()),
        })],
    };
    check_parsing(source, expected);
}

#[test]
fn test_parse_node_with_events() {
    let source = r#"
        node event_node {
            text: "Hello"
            with events: [
                0, say("hello")
                1.5, say("world").wait(1)
            ]
        }
    "#;
    let expected = Program {
        body: vec![TopLevel::NodeDef(NodeDef {
            name: "event_node".to_string(),
            name_span: Some((14, 24)), // Approximate span
            body: vec![
                NodeStmt::Text("Hello".to_string()),
                NodeStmt::WithEvents(WithEventsStmt {
                    events: vec![
                        WithEventItem::InlineEvent(Event {
                            index: 0.0,
                            action: EventAction {
                                call: FuncCall {
                                    name: "say".to_string(),
                                    name_span: Some((99, 102)),
                                    args: vec![Arg::String("hello".to_string())],
                                },
                                chains: vec![],
                            },
                        }),
                        WithEventItem::InlineEvent(Event {
                            index: 1.5,
                            action: EventAction {
                                call: FuncCall {
                                    name: "say".to_string(),
                                    name_span: Some((133, 136)),
                                    args: vec![Arg::String("world".to_string())],
                                },
                                chains: vec![FuncCall {
                                    name: "wait".to_string(),
                                    name_span: Some((146, 150)),
                                    args: vec![Arg::Number(1.0)],
                                }],
                            },
                        }),
                    ],
                }),
            ],
            jump: None,
        })],
    };
    check_parsing(source, expected);
}

#[test]
fn test_parse_node_with_choices() {
    let source = r#"
        node choice_node {
            choice: [
                "Choice 1" -> next_node,
                "Choice 2".when(is_ready) -> return,
                ("Choice 3").when(check(arg1)) -> break,
                "Choice 4" -> [
                    "Nested 1" -> nested_node
                ]
            ]
        }
    "#;
    let expected = Program {
        body: vec![TopLevel::NodeDef(NodeDef {
            name: "choice_node".to_string(),
            name_span: Some((14, 25)), // Approximate span
            body: vec![NodeStmt::Choice(vec![
                ChoiceItem {
                    text: "Choice 1".to_string(),
                    condition: None,
                    target: ChoiceDest::Identifier("next_node".to_string(), Some((80, 89))),
                },
                ChoiceItem {
                    text: "Choice 2".to_string(),
                    condition: Some(Condition::Identifier("is_ready".to_string())),
                    target: ChoiceDest::Return,
                },
                ChoiceItem {
                    text: "Choice 3".to_string(),
                    condition: Some(Condition::FuncCall(FuncCall {
                        name: "check".to_string(),
                        name_span: Some((178, 183)), // Updated to actual span
                        args: vec![Arg::Identifier("arg1".to_string())],
                    })),
                    target: ChoiceDest::Break,
                },
                ChoiceItem {
                    text: "Choice 4".to_string(),
                    condition: None,
                    target: ChoiceDest::NestedChoices(vec![ChoiceItem {
                        text: "Nested 1".to_string(),
                        condition: None,
                        target: ChoiceDest::Identifier("nested_node".to_string(), Some((267, 278))),
                    }]),
                },
            ])],
            jump: None,
        })],
    };
    check_parsing(source, expected);
}

#[test]
fn test_parse_multiple_top_level() {
    let source = r#"
        fn func1()
        node node1 {}
        fn func2()
        node node2 {}
    "#;
    let program = ParseHandler::parse_source_code(source, false).unwrap();
    assert_eq!(program.body.len(), 4);
    assert!(matches!(program.body[0], TopLevel::FunctionDecl(_)));
    assert!(matches!(program.body[1], TopLevel::NodeDef(_)));
    assert!(matches!(program.body[2], TopLevel::FunctionDecl(_)));
    assert!(matches!(program.body[3], TopLevel::NodeDef(_)));
}
