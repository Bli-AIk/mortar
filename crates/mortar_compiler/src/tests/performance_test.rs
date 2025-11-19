use crate::parser::*;
use crate::token::{Token, tokenize};

#[test]
fn test_tokenize_performance_keywords() {
    let source = "event run with ref timeline tl wait index action duration";
    let tokens = tokenize(source);

    assert_eq!(tokens[0].token, Token::Event);
    assert_eq!(tokens[1].token, Token::Run);
    assert_eq!(tokens[2].token, Token::With);
    assert_eq!(tokens[3].token, Token::Ref);
    assert_eq!(tokens[4].token, Token::Timeline);
    assert_eq!(tokens[5].token, Token::Timeline); // 'tl' is also Timeline
    assert_eq!(tokens[6].token, Token::Wait);
    assert_eq!(tokens[7].token, Token::Index);
    assert_eq!(tokens[8].token, Token::Action);
    assert_eq!(tokens[9].token, Token::Duration);
}

#[test]
fn test_parse_event_def() {
    let source = r#"
        event Basic {
            index: 0
            action: set_background("bg_forest.png")
        }
        
        fn set_background(file: String)
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();
    assert_eq!(program.body.len(), 2);

    match &program.body[0] {
        TopLevel::EventDef(event_def) => {
            assert_eq!(event_def.name, "Basic");
            assert_eq!(event_def.index, Some(0.0));
            assert_eq!(event_def.action.call.name, "set_background");
            assert_eq!(event_def.action.call.args.len(), 1);
        }
        _ => panic!("Expected EventDef"),
    }
}

#[test]
fn test_parse_event_def_with_duration() {
    let source = r#"
        event ShowAlice {
            action: show_character("alice.png", "center")
            duration: 2.0
        }
        
        fn show_character(image: String, position: String)
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::EventDef(event_def) => {
            assert_eq!(event_def.name, "ShowAlice");
            assert_eq!(event_def.index, None);
            assert_eq!(event_def.duration, Some(2.0));
        }
        _ => panic!("Expected EventDef"),
    }
}

#[test]
fn test_parse_timeline_def() {
    let source = r#"
        timeline IntroScene {
            run ShowAlice
            wait 2.0
            run PlayMusic
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::TimelineDef(timeline_def) => {
            assert_eq!(timeline_def.name, "IntroScene");
            assert_eq!(timeline_def.body.len(), 3);

            match &timeline_def.body[0] {
                TimelineStmt::Run(run_stmt) => {
                    assert_eq!(run_stmt.event_name, "ShowAlice");
                }
                _ => panic!("Expected Run statement"),
            }

            match &timeline_def.body[1] {
                TimelineStmt::Wait(duration) => {
                    assert_eq!(*duration, 2.0);
                }
                _ => panic!("Expected Wait statement"),
            }
        }
        _ => panic!("Expected TimelineDef"),
    }
}

#[test]
fn test_parse_run_stmt_in_node() {
    let source = r#"
        node ExampleNode {
            run Basic
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::NodeDef(node_def) => {
            assert_eq!(node_def.body.len(), 1);

            match &node_def.body[0] {
                NodeStmt::Run(run_stmt) => {
                    assert_eq!(run_stmt.event_name, "Basic");
                    assert_eq!(run_stmt.index_override, None);
                }
                _ => panic!("Expected Run statement"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_run_with_index() {
    let source = r#"
        node ExampleNode {
            run Basic with 1.5
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::NodeDef(node_def) => match &node_def.body[0] {
            NodeStmt::Run(run_stmt) => {
                assert_eq!(run_stmt.event_name, "Basic");
                match &run_stmt.index_override {
                    Some(IndexOverride::Value(v)) => assert_eq!(*v, 1.5),
                    _ => panic!("Expected Value index override"),
                }
            }
            _ => panic!("Expected Run statement"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_run_with_ref() {
    let source = r#"
        node ExampleNode {
            run Basic with ref test_index
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::NodeDef(node_def) => match &node_def.body[0] {
            NodeStmt::Run(run_stmt) => {
                assert_eq!(run_stmt.event_name, "Basic");
                match &run_stmt.index_override {
                    Some(IndexOverride::Reference(name)) => assert_eq!(name, "test_index"),
                    _ => panic!("Expected Reference index override"),
                }
            }
            _ => panic!("Expected Run statement"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_with_events() {
    let source = r#"
        node ExampleNode {
            text: "Hello"
            with Basic
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::NodeDef(node_def) => {
            assert_eq!(node_def.body.len(), 2);

            match &node_def.body[1] {
                NodeStmt::WithEvents(with_events) => {
                    assert_eq!(with_events.events.len(), 1);
                    match &with_events.events[0] {
                        WithEventItem::EventRef(name, _) => assert_eq!(name, "Basic"),
                        _ => panic!("Expected EventRef"),
                    }
                }
                _ => panic!("Expected WithEvents statement"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_with_events_list() {
    let source = r#"
        node ExampleNode {
            text: "Hello"
            with events: [
                Basic
                Advanced
            ]
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::NodeDef(node_def) => match &node_def.body[1] {
            NodeStmt::WithEvents(with_events) => {
                assert_eq!(with_events.events.len(), 2);
            }
            _ => panic!("Expected WithEvents statement"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_run_with_args() {
    let source = r#"
        timeline IntroScene {
            run DialogueNode("Start")
        }
    "#;

    let program = ParseHandler::parse_source_code(source, false).unwrap();

    match &program.body[0] {
        TopLevel::TimelineDef(timeline_def) => match &timeline_def.body[0] {
            TimelineStmt::Run(run_stmt) => {
                assert_eq!(run_stmt.event_name, "DialogueNode");
                assert_eq!(run_stmt.args.len(), 1);
                match &run_stmt.args[0] {
                    Arg::String(s) => assert_eq!(s, "Start"),
                    _ => panic!("Expected String argument"),
                }
            }
            _ => panic!("Expected Run statement"),
        },
        _ => panic!("Expected TimelineDef"),
    }
}
