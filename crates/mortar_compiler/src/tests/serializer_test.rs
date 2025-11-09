use crate::parser::{
    Arg, ChoiceDest, ChoiceItem, Condition, Event, EventAction, FuncCall, FunctionDecl, NodeDef,
    NodeJump, NodeStmt, Param, Program, TopLevel,
};
use crate::serializer::Serializer;
use serde_json::Value;

fn create_test_program() -> Program {
    Program {
        body: vec![
            TopLevel::FunctionDecl(FunctionDecl {
                name: "my_function".to_string(),
                name_span: Some((0, 11)), // Approximate span
                params: vec![Param {
                    name: "p1".to_string(),
                    type_name: "String".to_string(),
                }],
                return_type: Some("Number".to_string()),
            }),
            TopLevel::NodeDef(NodeDef {
                name: "start_node".to_string(),
                name_span: Some((0, 10)), // Approximate span
                body: vec![
                    NodeStmt::Text("This is the first line.".to_string()),
                    NodeStmt::Events(vec![Event {
                        index: 0.5,
                        action: EventAction {
                            call: FuncCall {
                                name: "play_sound".to_string(),
                                name_span: Some((0, 10)), // Approximate span
                                args: vec![Arg::String("music.mp3".to_string())],
                            },
                            chains: vec![],
                        },
                    }]),
                    NodeStmt::Text("This is the second line.".to_string()),
                    NodeStmt::Choice(vec![
                        ChoiceItem {
                            text: "Go to next".to_string(),
                            condition: None,
                            target: ChoiceDest::Identifier("next_node".to_string(), Some((0, 9))),
                        },
                        ChoiceItem {
                            text: "Stay here".to_string(),
                            condition: Some(Condition::Identifier("has_item".to_string())),
                            target: ChoiceDest::Break,
                        },
                    ]),
                ],
                jump: Some(NodeJump::Identifier(
                    "default_next".to_string(),
                    Some((0, 12)),
                )),
            }),
        ],
    }
}

#[test]
fn test_serialize_program() {
    let program = create_test_program();
    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    assert_eq!(json["metadata"]["version"], "0.1.0");
    assert!(json["metadata"]["generated_at"].is_string());

    assert_eq!(json["functions"].as_array().unwrap().len(), 1);
    assert_eq!(json["nodes"].as_array().unwrap().len(), 1);
}

#[test]
fn test_serialize_function_decl() {
    let program = create_test_program();
    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    let function = &json["functions"][0];
    assert_eq!(function["name"], "my_function");
    assert_eq!(function["params"][0]["name"], "p1");
    assert_eq!(function["params"][0]["type"], "String");
    assert_eq!(function["return"], "Number");
}

#[test]
fn test_serialize_node_def() {
    let program = create_test_program();
    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    let node = &json["nodes"][0];
    assert_eq!(node["name"], "start_node");
    assert_eq!(node["next"], "default_next");
    assert_eq!(node["texts"].as_array().unwrap().len(), 2);
    assert_eq!(node["choice"].as_array().unwrap().len(), 2);
}

#[test]
fn test_serialize_text_and_events() {
    let program = create_test_program();
    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    let node = &json["nodes"][0];
    let text1 = &node["texts"][0];
    assert_eq!(text1["text"], "This is the first line.");
    assert_eq!(text1["events"][0]["index"], 0.5);
    assert_eq!(text1["events"][0]["actions"][0]["type"], "play_sound");
    assert_eq!(text1["events"][0]["actions"][0]["args"][0], "music.mp3");

    let text2 = &node["texts"][1];
    assert_eq!(text2["text"], "This is the second line.");
    assert!(text2["events"].is_null());
}

#[test]
fn test_serialize_choices() {
    let program = create_test_program();
    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    let node = &json["nodes"][0];
    let choice1 = &node["choice"][0];
    assert_eq!(choice1["text"], "Go to next");
    assert!(choice1["condition"].is_null());
    assert_eq!(choice1["next"], "next_node");

    let choice2 = &node["choice"][1];
    assert_eq!(choice2["text"], "Stay here");
    assert_eq!(choice2["condition"]["type"], "has_item");
    assert_eq!(choice2["action"], "break");
}
