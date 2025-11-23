use crate::parser::{ComparisonOp, IfCondition, NodeStmt, ParseHandler, TopLevel};

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
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            match &node.body[0] {
                NodeStmt::IfElse(if_else) => {
                    match &if_else.condition {
                        IfCondition::Unary(unary) => {
                            // Check operand
                            match &unary.operand {
                                IfCondition::Identifier(name) => {
                                    assert_eq!(name, "flag");
                                }
                                _ => panic!("Expected Identifier operand"),
                            }
                        }
                        _ => panic!("Expected Unary condition"),
                    }
                }
                _ => panic!("Expected IfElse"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
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
