use crate::parser::{NodeStmt, ParseHandler, StringPart, TopLevel};

#[test]
fn test_parse_placeholder_in_text() {
    let source = r#"
        node Test {
            text: $"Hello {name}!"
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::InterpolatedText(interp) => {
                assert_eq!(interp.parts.len(), 3);
                assert!(matches!(&interp.parts[0], StringPart::Text(_)));
                assert!(matches!(&interp.parts[1], StringPart::Placeholder(_)));
                assert!(matches!(&interp.parts[2], StringPart::Text(_)));

                if let StringPart::Placeholder(name) = &interp.parts[1] {
                    assert_eq!(name, "name");
                }
            }
            _ => panic!("Expected InterpolatedText"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_simple_branch() {
    let source = r#"
        let is_forest: Bool
        
        node Test {
            text: $"Location: {place}"
            
            place: branch [
                is_forest, "森林"
                is_city, "城市"
            ]
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            // Should have text and branch
            assert_eq!(node.body.len(), 2);

            match &node.body[1] {
                NodeStmt::Branch(branch) => {
                    assert_eq!(branch.name, "place");
                    assert!(branch.enum_type.is_none());
                    assert_eq!(branch.cases.len(), 2);
                    assert_eq!(branch.cases[0].condition, "is_forest");
                    assert_eq!(branch.cases[0].text, "森林");
                    assert_eq!(branch.cases[1].condition, "is_city");
                    assert_eq!(branch.cases[1].text, "城市");
                }
                _ => panic!("Expected Branch"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_branch_with_enum() {
    let source = r#"
        enum GameState { active, paused, stopped }
        
        node Test {
            text: $"Status: {status}"
            
            status: branch<GameState> [
                active, "运行中"
                paused, "已暂停"
                stopped, "已停止"
            ]
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[1] {
            NodeStmt::Branch(branch) => {
                assert_eq!(branch.name, "status");
                assert_eq!(branch.enum_type.as_ref().unwrap(), "GameState");
                assert_eq!(branch.cases.len(), 3);
            }
            _ => panic!("Expected Branch"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_branch_with_events() {
    let source = r#"
        enum Color { red, blue }
        
        node Test {
            text: $"Color: {color}"
            
            color: branch<Color> [
                red, "红色", events: [
                    0, set_color("red")
                ]
                blue, "蓝色", events: [
                    0, set_color("blue")
                ]
            ]
        }
        
        fn set_color(c: String)
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            match &node.body[1] {
                NodeStmt::Branch(branch) => {
                    assert_eq!(branch.name, "color");
                    assert_eq!(branch.cases.len(), 2);

                    // Check that events are captured
                    assert!(branch.cases[0].events.is_some());
                    assert!(branch.cases[1].events.is_some());

                    let events = branch.cases[0].events.as_ref().unwrap();
                    assert_eq!(events.len(), 1);
                }
                _ => panic!("Expected Branch"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_multiple_placeholders() {
    let source = r#"
        node Test {
            text: $"Hello {name}, you are in {place}."
            
            name: branch [
                is_male, "先生"
                is_female, "女士"
            ]
            
            place: branch [
                is_forest, "森林"
                is_city, "城市"
            ]
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::NodeDef(node) => {
            // Should have 1 text and 2 branches
            assert_eq!(node.body.len(), 3);

            match &node.body[0] {
                NodeStmt::InterpolatedText(interp) => {
                    // Count placeholders
                    let placeholder_count = interp
                        .parts
                        .iter()
                        .filter(|p| matches!(p, StringPart::Placeholder(_)))
                        .count();
                    assert_eq!(placeholder_count, 2);
                }
                _ => panic!("Expected InterpolatedText"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_serialize_branch() {
    use crate::Serializer;
    use serde_json::Value;

    let source = r#"
        enum Status { online, offline }
        
        node Test {
            text: $"Status: {status}"
            
            status: branch<Status> [
                online, "在线"
                offline, "离线"
            ]
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    // Check branches in JSON
    assert!(json["nodes"][0]["branches"].is_array());

    let branches = json["nodes"][0]["branches"].as_array().unwrap();
    assert_eq!(branches.len(), 1);

    let branch = &branches[0];
    assert_eq!(branch["name"], "status");
    assert_eq!(branch["enum_type"], "Status");

    let cases = branch["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0]["condition"], "online");
    assert_eq!(cases[0]["text"], "在线");
}

#[test]
fn test_branch_without_enum_type() {
    let source = r#"
        let flag: Bool
        
        node Test {
            text: $"Value: {value}"
            
            value: branch [
                flag, "是"
            ]
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[1] {
            NodeStmt::Branch(branch) => {
                assert!(branch.enum_type.is_none());
            }
            _ => panic!("Expected Branch"),
        },
        _ => panic!("Expected NodeDef"),
    }
}
use crate::parser::*;

#[test]
fn test_parse_branch_variable_with_enum() {
    let source = r#"
        enum GameState {
            start
            playing
            game_over
        }
        
        let current_state: GameState
        
        let status: branch<current_state> [
            start, "游戏开始"
            playing, "游戏进行中"
            game_over, "游戏结束"
        ]
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();

    // Should have: enum, variable, branch variable
    assert_eq!(program.body.len(), 3);

    // Check branch variable
    match &program.body[2] {
        TopLevel::VarDecl(var_decl) => {
            assert_eq!(var_decl.name, "status");
            assert_eq!(var_decl.type_name, "Branch");

            if let Some(VarValue::Branch(branch_value)) = &var_decl.value {
                assert_eq!(branch_value.enum_type.as_ref().unwrap(), "current_state");
                assert_eq!(branch_value.cases.len(), 3);
                assert_eq!(branch_value.cases[0].condition, "start");
                assert_eq!(branch_value.cases[0].text, "游戏开始");
            } else {
                panic!("Expected Branch value");
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_branch_variable_with_bool() {
    let source = r#"
        let is_forest: Bool
        
        let place: branch [
            is_forest, "森林"
            is_city, "城市"
        ]
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();

    // Check branch variable
    match &program.body[1] {
        TopLevel::VarDecl(var_decl) => {
            assert_eq!(var_decl.name, "place");
            assert_eq!(var_decl.type_name, "Branch");

            if let Some(VarValue::Branch(branch_value)) = &var_decl.value {
                assert!(branch_value.enum_type.is_none());
                assert_eq!(branch_value.cases.len(), 2);
            } else {
                panic!("Expected Branch value");
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_serialize_branch_variable() {
    use crate::Serializer;
    use serde_json::Value;

    let source = r#"
        enum Status { online, offline }
        let current_status: Status
        
        let status_text: branch<current_status> [
            online, "在线"
            offline, "离线"
        ]
        
        node Test {
            text: $"状态: {status_text}"
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    // Check that branch variable is in variables array
    let variables = json["variables"].as_array().unwrap();
    let branch_var = variables
        .iter()
        .find(|v| v["name"] == "status_text")
        .unwrap();

    assert_eq!(branch_var["type"], "Branch");
    assert_eq!(branch_var["value"]["enum_type"], "current_status");

    let cases = branch_var["value"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0]["condition"], "online");
    assert_eq!(cases[0]["text"], "在线");
}
