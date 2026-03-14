//! # control_flow_test.rs
//!
//! # control_flow_test.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Tests for control flow constructs like if-else statements.
//!
//! 控制流结构（如 if-else 语句）的测试。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! Verifies the parsing of nested if-else blocks and conditions.
//!
//! 验证嵌套 if-else 块和条件的解析。

use crate::ast::{ComparisonOp, FuncCall, IfCondition, NodeStmt, TopLevel};
use crate::parser::ParseHandler;

#[test]
fn test_parse_simple_if() {
    let source = r#"
        let score: Number
        
        node Test {
            if score > 100 {
                text: "High score!"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            assert_eq!(node.body.len(), 1);
            match &node.body[0] {
                NodeStmt::IfElse(if_else) => {
                    assert!(if_else.else_body.is_none());
                    assert_eq!(if_else.then_body.len(), 1);
                }
                _ => panic!("Expected IfElse"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_if_else() {
    let source = r#"
        let score: Number
        
        node Test {
            if score > 100 {
                text: "High score!"
            } else {
                text: "Low score."
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => {
                assert!(if_else.else_body.is_some());
                assert_eq!(if_else.then_body.len(), 1);
                assert_eq!(if_else.else_body.as_ref().unwrap().len(), 1);
            }
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_boolean_condition() {
    let source = r#"
        let is_winner: Bool
        
        node Test {
            if is_winner {
                text: "Winner!"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::Identifier(name) => {
                    assert_eq!(name, "is_winner");
                }
                _ => panic!("Expected Identifier condition"),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_comparison_operators() {
    let test_cases = vec![
        (">", ComparisonOp::Greater),
        ("<", ComparisonOp::Less),
        (">=", ComparisonOp::GreaterEqual),
        ("<=", ComparisonOp::LessEqual),
        ("==", ComparisonOp::Equal),
        ("!=", ComparisonOp::NotEqual),
    ];

    for (op_str, expected_op) in test_cases {
        let source = format!(
            r#"
            let a: Number
            let b: Number
            
            node Test {{
                if a {} b {{
                    text: "test"
                }}
            }}
        "#,
            op_str
        );

        let result = ParseHandler::parse_source_code(&source, false);
        assert!(result.is_ok(), "Failed to parse operator {}", op_str);

        let program = result.unwrap();
        match &program.body[2] {
            TopLevel::NodeDef(node) => match &node.body[0] {
                NodeStmt::IfElse(if_else) => match &if_else.condition {
                    IfCondition::Binary(binary) => {
                        assert_eq!(binary.operator, expected_op);
                    }
                    _ => panic!("Expected Binary condition for {}", op_str),
                },
                _ => panic!("Expected IfElse for {}", op_str),
            },
            _ => panic!("Expected NodeDef for {}", op_str),
        }
    }
}

#[test]
fn test_parse_logical_and() {
    let source = r#"
        let a: Bool
        let b: Bool
        
        node Test {
            if a && b {
                text: "Both true"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[2] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::Binary(binary) => {
                    assert_eq!(binary.operator, ComparisonOp::And);
                }
                _ => panic!("Expected Binary condition"),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_logical_or() {
    let source = r#"
        let a: Bool
        let b: Bool
        
        node Test {
            if a || b {
                text: "At least one true"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[2] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::Binary(binary) => {
                    assert_eq!(binary.operator, ComparisonOp::Or);
                }
                _ => panic!("Expected Binary condition"),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_not_operator() {
    let source = r#"
        let flag: Bool
        
        node Test {
            if !flag {
                text: "Not flag"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    let TopLevel::NodeDef(node) = &program.body[1] else {
        panic!("Expected NodeDef");
    };
    let NodeStmt::IfElse(if_else) = &node.body[0] else {
        panic!("Expected IfElse");
    };
    let IfCondition::Unary(unary) = &if_else.condition else {
        panic!("Expected Unary condition");
    };
    let IfCondition::Identifier(name) = &unary.operand else {
        panic!("Expected Identifier operand");
    };
    assert_eq!(name, "flag");
}

#[test]
fn test_serialize_if_else() {
    use crate::Serializer;
    use serde_json::Value;

    let source = r#"
        let score: Number
        
        node Test {
            if score > 100 {
                text: "High score!"
            } else {
                text: "Low score."
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    // Check that node has a content array with conditional texts
    assert!(json["nodes"][0]["content"].is_array());
    let content = json["nodes"][0]["content"].as_array().unwrap();
    assert_eq!(content.len(), 2);

    // Check first text (then body)
    assert_eq!(content[0]["type"], "text");
    assert_eq!(content[0]["value"], "High score!");
    assert!(content[0]["condition"].is_object());
    assert_eq!(content[0]["condition"]["type"], "binary");
    assert_eq!(content[0]["condition"]["operator"], ">");

    // Check second text (else body with negated condition)
    assert_eq!(content[1]["type"], "text");
    assert_eq!(content[1]["value"], "Low score.");
    assert!(content[1]["condition"].is_object());
    assert_eq!(content[1]["condition"]["type"], "unary");
    assert_eq!(content[1]["condition"]["operator"], "!");
}

#[test]
fn test_parse_func_call_in_if() {
    let source = r#"
        fn is_active() -> Bool

        node Test {
            if is_active() {
                text: "Active"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::FuncCall(fc) => {
                    assert_eq!(fc.name, "is_active");
                    assert!(fc.args.is_empty());
                }
                other => panic!("Expected FuncCall condition, got {:?}", other),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_func_call_comparison_in_if() {
    let source = r#"
        fn get_hp() -> Number

        node Test {
            if get_hp() > 0 {
                text: "Alive"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::Binary(binary) => {
                    assert_eq!(binary.operator, ComparisonOp::Greater);
                    assert!(
                        matches!(&binary.left, IfCondition::FuncCall(FuncCall { name, .. }) if name == "get_hp")
                    );
                }
                other => panic!("Expected Binary condition, got {:?}", other),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_func_call_with_args_in_if() {
    let source = r#"
        fn has_item(name: String) -> Bool

        node Test {
            if has_item("sword") {
                text: "Armed"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::FuncCall(fc) => {
                    assert_eq!(fc.name, "has_item");
                    assert_eq!(fc.args.len(), 1);
                }
                other => panic!("Expected FuncCall condition, got {:?}", other),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_func_call_negation_in_if() {
    let source = r#"
        fn is_dead() -> Bool

        node Test {
            if !is_dead() {
                text: "Alive"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[1] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::Unary(unary) => {
                    assert!(
                        matches!(&unary.operand, IfCondition::FuncCall(FuncCall { name, .. }) if name == "is_dead")
                    );
                }
                other => panic!("Expected Unary condition, got {:?}", other),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_func_call_combined_in_if() {
    let source = r#"
        fn get_hp() -> Number
        fn is_active() -> Bool

        node Test {
            if get_hp() > 0 && is_active() {
                text: "Ready"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[2] {
        TopLevel::NodeDef(node) => match &node.body[0] {
            NodeStmt::IfElse(if_else) => match &if_else.condition {
                IfCondition::Binary(binary) => {
                    assert_eq!(binary.operator, ComparisonOp::And);
                    // left is get_hp() > 0 (a Binary)
                    assert!(matches!(&binary.left, IfCondition::Binary(_)));
                    // right is is_active() (a FuncCall)
                    assert!(
                        matches!(&binary.right, IfCondition::FuncCall(FuncCall { name, .. }) if name == "is_active")
                    );
                }
                other => panic!("Expected Binary(&&) condition, got {:?}", other),
            },
            _ => panic!("Expected IfElse"),
        },
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_serialize_func_call_in_if() {
    use crate::Serializer;
    use serde_json::Value;

    let source = r#"
        fn is_active() -> Bool

        node Test {
            if is_active() {
                text: "Active"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    let content = json["nodes"][0]["content"].as_array().unwrap();
    assert_eq!(content[0]["type"], "text");
    assert_eq!(content[0]["value"], "Active");

    let condition = &content[0]["condition"];
    assert_eq!(condition["type"], "func_call");
    assert_eq!(condition["operand"]["value"], "is_active");
}

#[test]
fn test_serialize_func_call_comparison_in_if() {
    use crate::Serializer;
    use serde_json::Value;

    let source = r#"
        fn get_hp() -> Number

        node Test {
            if get_hp() > 0 {
                text: "Alive"
            }
        }
    "#;

    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    let condition = &json["nodes"][0]["content"][0]["condition"];
    assert_eq!(condition["type"], "binary");
    assert_eq!(condition["operator"], ">");
    assert_eq!(condition["left"]["type"], "func_call");
    assert_eq!(condition["left"]["operand"]["value"], "get_hp");
    assert_eq!(condition["right"]["type"], "identifier");
    assert_eq!(condition["right"]["value"], "0");
}
