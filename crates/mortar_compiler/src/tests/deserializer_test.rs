use crate::deserializer::Deserializer;
use serde_json::Value;
use tempfile::TempDir;
// Create a program, serialize it, then deserialize it
use crate::ast::{FunctionDecl, NodeDef, NodeStmt, Program, TopLevel};

#[test]
fn test_deserialize_basic_json() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "nodes": [
            {
                "name": "Start",
                "content": [
                    {
                        "type": "text",
                        "value": "Hello, World!"
                    }
                ]
            }
        ],
        "functions": []
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    assert_eq!(data.metadata.version, "0.4.0");
    assert_eq!(data.nodes.len(), 1);
    let node = &data.nodes[0];
    assert_eq!(node.name, "Start");
    assert_eq!(node.content.len(), 1);
    if let Value::Object(item) = &node.content[0] {
        assert_eq!(item["type"], "text");
        assert_eq!(item["value"], "Hello, World!");
    } else {
        panic!("Expected content item to be an object");
    }
}

#[test]
fn test_deserialize_with_events() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "nodes": [
            {
                "name": "TestNode",
                "content": [
                    {
                        "type": "text",
                        "value": "Test text",
                        "events": [
                            {
                                "index": 0.5,
                                "actions": [
                                    {
                                        "type": "play_sound",
                                        "args": ["sound.mp3"]
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        ],
        "functions": []
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    let node = &data.nodes[0];
    assert_eq!(node.content.len(), 1);

    if let Value::Object(item) = &node.content[0] {
        assert_eq!(item["type"], "text");
        assert!(item["events"].is_array());
        let events = item["events"].as_array().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["index"], 0.5);
        assert_eq!(events[0]["actions"][0]["type"], "play_sound");
        assert_eq!(events[0]["actions"][0]["args"][0], "sound.mp3");
    } else {
        panic!("Expected content item to be an object");
    }
}

#[test]
fn test_deserialize_with_choices() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "nodes": [
            {
                "name": "ChoiceNode",
                "content": [
                    {
                        "type": "text",
                        "value": "What do you choose?"
                    },
                    {
                        "type": "choice",
                        "options": [
                            {
                                "text": "Option A",
                                "next": "NodeA"
                            },
                            {
                                "text": "Option B",
                                "next": "NodeB"
                            }
                        ]
                    }
                ]
            }
        ],
        "functions": []
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    let node = &data.nodes[0];
    assert_eq!(node.content.len(), 2);
    if let Value::Object(item) = &node.content[1] {
        assert_eq!(item["type"], "choice");
        let choices = item["options"].as_array().unwrap();
        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0]["text"], "Option A");
        assert_eq!(choices[0]["next"], "NodeA");
    } else {
        panic!("Expected content item to be an object");
    }
}

#[test]
fn test_deserialize_with_functions() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "nodes": [],
        "functions": [
            {
                "name": "test_func",
                "params": [
                    {
                        "name": "param1",
                        "type": "String"
                    }
                ],
                "return": "Number"
            }
        ]
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    assert_eq!(data.functions.len(), 1);
    let func = &data.functions[0];
    assert_eq!(func.name, "test_func");
    assert_eq!(func.params.len(), 1);
    assert_eq!(func.params[0].name, "param1");
    assert_eq!(func.params[0].param_type, "String");
    assert_eq!(func.return_type.as_ref().unwrap(), "Number");
}

#[test]
fn test_deserialize_with_variables() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "variables": [
            {
                "name": "player_name",
                "type": "String"
            },
            {
                "name": "score",
                "type": "Number",
                "value": 100
            }
        ],
        "nodes": [],
        "functions": []
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    assert_eq!(data.variables.len(), 2);
    assert_eq!(data.variables[0].name, "player_name");
    assert!(data.variables[0].value.is_none());
    assert_eq!(data.variables[1].name, "score");
    assert_eq!(
        data.variables[1].value.as_ref().unwrap().as_f64().unwrap(),
        100.0
    );
}

#[test]
fn test_deserialize_with_constants() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "constants": [
            {
                "name": "MAX_LEVEL",
                "type": "Number",
                "value": 99,
                "public": true
            }
        ],
        "nodes": [],
        "functions": []
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    assert_eq!(data.constants.len(), 1);
    assert_eq!(data.constants[0].name, "MAX_LEVEL");
    assert_eq!(data.constants[0].value.as_f64().unwrap(), 99.0);
    assert!(data.constants[0].public);
}

#[test]
fn test_deserialize_with_enums() {
    let json = r#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "enums": [
            {
                "name": "GameState",
                "variants": ["menu", "playing", "paused"]
            }
        ],
        "nodes": [],
        "functions": []
    }"#;

    let data = Deserializer::from_json(json).unwrap();
    assert_eq!(data.enums.len(), 1);
    assert_eq!(data.enums[0].name, "GameState");
    assert_eq!(data.enums[0].variants.len(), 3);
    assert_eq!(data.enums[0].variants[0], "menu");
}

#[test]
fn test_deserialize_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.mortared");

    let json = r#"{
        "metadata": { "version": "0.4.0", "generated_at": "2025-11-20T00:00:00Z" },
        "nodes": [
            {
                "name": "FileTest",
                "content": [{"type": "text", "value": "From file"}]
            }
        ],
        "functions": []
    }"#;

    std::fs::write(&file_path, json).unwrap();

    let data = Deserializer::from_file(&file_path).unwrap();
    assert_eq!(data.nodes[0].name, "FileTest");
    assert_eq!(data.nodes[0].content.len(), 1);
}

#[test]
fn test_deserialize_from_bytes() {
    let json_bytes = br#"{
        "metadata": {
            "version": "0.4.0",
            "generated_at": "2025-11-20T00:00:00Z"
        },
        "nodes": [],
        "functions": []
    }"#;

    let data = Deserializer::from_bytes(json_bytes).unwrap();
    assert_eq!(data.metadata.version, "0.4.0");
}

#[test]
fn test_mortared_data_helper_methods() {
    let json = r#"{
        "metadata": { "version": "0.4.0", "generated_at": "2025-11-20T00:00:00Z" },
        "nodes": [
            {"name": "Node1", "content": [{"type": "text", "value": "Text1"}]},
            {"name": "Node2", "content": [{"type": "text", "value": "Text2"}]}
        ],
        "functions": [
            {"name": "func1", "params": []}
        ],
        "variables": [
            {"name": "var1", "type": "String"}
        ]
    }"#;

    let data = Deserializer::from_json(json).unwrap();

    // Test get_node
    assert!(data.get_node("Node1").is_some());
    assert!(data.get_node("NonExistent").is_none());

    // Test get_function
    assert!(data.get_function("func1").is_some());
    assert!(data.get_function("func2").is_none());

    // Test get_variable
    assert!(data.get_variable("var1").is_some());
    assert!(data.get_variable("var2").is_none());

    // Test node_names
    let names = data.node_names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"Node1"));
    assert!(names.contains(&"Node2"));
}

#[test]
fn test_deserialize_error_invalid_json() {
    let invalid_json = "{ invalid json }";
    let result = Deserializer::from_json(invalid_json);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Deserialization error"));
}

#[test]
fn test_roundtrip_serialization() {
    // Create a program, serialize it, then deserialize it
    use crate::serializer::Serializer;

    let program = Program {
        body: vec![
            TopLevel::NodeDef(NodeDef {
                name: "TestNode".to_string(),
                name_span: Some((0, 8)),
                body: vec![NodeStmt::Text("Test content".to_string())],
                jump: None,
            }),
            TopLevel::FunctionDecl(FunctionDecl {
                name: "test_func".to_string(),
                name_span: Some((0, 9)),
                params: vec![],
                return_type: None,
            }),
        ],
    };

    // Serialize
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();

    // Deserialize
    let data = Deserializer::from_json(&json_str).unwrap();

    // Verify
    assert_eq!(data.nodes.len(), 1);
    let node = &data.nodes[0];
    assert_eq!(node.name, "TestNode");
    assert_eq!(node.content.len(), 1);

    if let Value::Object(item) = &node.content[0] {
        assert_eq!(item["type"], "text");
        assert_eq!(item["value"], "Test content");
    } else {
        panic!("Expected content item to be an object");
    }

    assert_eq!(data.functions.len(), 1);
    assert_eq!(data.functions[0].name, "test_func");
}
