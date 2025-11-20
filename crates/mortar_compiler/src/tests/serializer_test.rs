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

    assert_eq!(json["metadata"]["version"], env!("CARGO_PKG_VERSION"));
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

#[test]
fn test_serialize_empty_program() {
    use crate::parser::Program;

    let program = Program { body: vec![] };
    let result = Serializer::serialize_to_json(&program, false);
    assert!(result.is_ok());

    let json = result.unwrap();
    assert!(json.contains("metadata"));
    assert!(json.contains("nodes"));
    assert!(json.contains("functions"));
}

#[test]
fn test_serialize_to_json_pretty() {
    use crate::parser::Program;

    let program = Program { body: vec![] };
    let result = Serializer::serialize_to_json(&program, true);
    assert!(result.is_ok());

    let json = result.unwrap();
    // Pretty formatted should have indentation or newlines
    assert!(json.contains("  ") || json.contains("\n"));
}

#[test]
fn test_save_to_file_basic() {
    use crate::parser::Program;
    use tempfile::TempDir;

    let program = Program { body: vec![] };
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("output.mortared");

    let result = Serializer::save_to_file(&program, output_file.to_str().unwrap(), false);

    assert!(result.is_ok());
    assert!(output_file.exists());
}

#[test]
fn test_save_to_file_with_language() {
    use crate::{Language, parser::Program};
    use tempfile::TempDir;

    let program = Program { body: vec![] };
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("output.mortared");

    let result = Serializer::save_to_file_with_language(
        &program,
        output_file.to_str().unwrap(),
        true, // pretty
        Language::English,
    );

    assert!(result.is_ok());
    assert!(output_file.exists());
}

#[test]
fn test_serialize_variable_declarations() {
    use crate::parser::{Program, TopLevel, VarDecl, VarValue};

    let program = Program {
        body: vec![
            TopLevel::VarDecl(VarDecl {
                name: "player_name".to_string(),
                name_span: Some((0, 11)),
                type_name: "String".to_string(),
                value: None,
            }),
            TopLevel::VarDecl(VarDecl {
                name: "score".to_string(),
                name_span: Some((0, 5)),
                type_name: "Number".to_string(),
                value: Some(VarValue::Number(100.0)),
            }),
        ],
    };

    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    assert_eq!(json["variables"].as_array().unwrap().len(), 2);
    assert_eq!(json["variables"][0]["name"], "player_name");
    assert_eq!(json["variables"][0]["type"], "String");
    assert!(json["variables"][0]["value"].is_null());

    assert_eq!(json["variables"][1]["name"], "score");
    assert_eq!(json["variables"][1]["type"], "Number");
    assert_eq!(json["variables"][1]["value"], 100.0);
}

#[test]
fn test_serialize_constant_declarations() {
    use crate::parser::{ConstDecl, Program, TopLevel, VarValue};

    let program = Program {
        body: vec![
            TopLevel::ConstDecl(ConstDecl {
                is_public: true,
                name: "game_title".to_string(),
                name_span: Some((0, 10)),
                type_name: "String".to_string(),
                value: VarValue::String("My Game".to_string()),
            }),
            TopLevel::ConstDecl(ConstDecl {
                is_public: false,
                name: "max_level".to_string(),
                name_span: Some((0, 9)),
                type_name: "Number".to_string(),
                value: VarValue::Number(99.0),
            }),
        ],
    };

    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    assert_eq!(json["constants"].as_array().unwrap().len(), 2);

    assert_eq!(json["constants"][0]["name"], "game_title");
    assert_eq!(json["constants"][0]["type"], "String");
    assert_eq!(json["constants"][0]["value"], "My Game");
    assert_eq!(json["constants"][0]["public"], true);

    assert_eq!(json["constants"][1]["name"], "max_level");
    assert_eq!(json["constants"][1]["public"], false);
}

#[test]
fn test_serialize_enum_definitions() {
    use crate::parser::{EnumDef, Program, TopLevel};

    let program = Program {
        body: vec![TopLevel::EnumDef(EnumDef {
            name: "GameState".to_string(),
            name_span: Some((0, 9)),
            variants: vec![
                "menu".to_string(),
                "playing".to_string(),
                "paused".to_string(),
            ],
        })],
    };

    let json_string = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_string).unwrap();

    assert_eq!(json["enums"].as_array().unwrap().len(), 1);
    assert_eq!(json["enums"][0]["name"], "GameState");
    assert_eq!(json["enums"][0]["variants"].as_array().unwrap().len(), 3);
    assert_eq!(json["enums"][0]["variants"][0], "menu");
    assert_eq!(json["enums"][0]["variants"][1], "playing");
    assert_eq!(json["enums"][0]["variants"][2], "paused");
}
