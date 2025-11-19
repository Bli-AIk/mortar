use crate::parser::{ParseHandler, TopLevel, NodeStmt, StringPart};

#[test]
fn test_parse_branch_interpolation_simple() {
    let source = r#"
        enum Gender { male, female }
        
        node Test {
            text: $"Hello {branch<Gender>(get_gender()) { male: \"Mr.\", female: \"Ms.\" }}"
        }
        
        fn get_gender() -> Gender
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    
    // Check enum definition
    assert!(matches!(&program.body[0], TopLevel::EnumDef(_)));
    
    // Check node with branch interpolation
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            assert_eq!(node.name, "Test");
            
            match &node.body[0] {
                NodeStmt::InterpolatedText(interp) => {
                    // Should have 3 parts: text, branch, text
                    assert_eq!(interp.parts.len(), 2); // "Hello " and branch
                    
                    match &interp.parts[1] {
                        StringPart::Branch(branch) => {
                            assert_eq!(branch.enum_type, "Gender");
                            assert_eq!(branch.enum_value_expr.name, "get_gender");
                            assert_eq!(branch.branches.len(), 2);
                            assert_eq!(branch.branches[0].variant, "male");
                            assert_eq!(branch.branches[0].text, "Mr.");
                            assert_eq!(branch.branches[1].variant, "female");
                            assert_eq!(branch.branches[1].text, "Ms.");
                        }
                        _ => panic!("Expected Branch part"),
                    }
                }
                _ => panic!("Expected InterpolatedText"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_parse_branch_with_multiple_cases() {
    let source = r#"
        enum Count { zero, one, two, many }
        
        node Test {
            text: $"Items: {branch<Count>(get_count()) { zero: \"none\", one: \"1\", two: \"2\", many: \"lots\" }}"
        }
        
        fn get_count() -> Count
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            match &node.body[0] {
                NodeStmt::InterpolatedText(interp) => {
                    match &interp.parts[1] {
                        StringPart::Branch(branch) => {
                            assert_eq!(branch.enum_type, "Count");
                            assert_eq!(branch.branches.len(), 4);
                            assert_eq!(branch.branches[0].variant, "zero");
                            assert_eq!(branch.branches[0].text, "none");
                            assert_eq!(branch.branches[3].variant, "many");
                            assert_eq!(branch.branches[3].text, "lots");
                        }
                        _ => panic!("Expected Branch"),
                    }
                }
                _ => panic!("Expected InterpolatedText"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_branch_with_empty_text() {
    let source = r#"
        enum Type { a, b, c }
        
        node Test {
            text: $"Text {branch<Type>(get_type()) { a: \"A\", b: \"\", c: \"C\" }} end"
        }
        
        fn get_type() -> Type
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            match &node.body[0] {
                NodeStmt::InterpolatedText(interp) => {
                    match &interp.parts[1] {
                        StringPart::Branch(branch) => {
                            assert_eq!(branch.branches[1].text, ""); // Empty text for 'b'
                        }
                        _ => panic!("Expected Branch"),
                    }
                }
                _ => panic!("Expected InterpolatedText"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_branch_mixed_with_regular_interpolation() {
    let source = r#"
        enum Gender { male, female }
        
        node Test {
            text: $"Hello {get_title()} {branch<Gender>(get_gender()) { male: \"Mr.\", female: \"Ms.\" }} {get_name()}"
        }
        
        fn get_gender() -> Gender
        fn get_title() -> String
        fn get_name() -> String
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    
    match &program.body[1] {
        TopLevel::NodeDef(node) => {
            match &node.body[0] {
                NodeStmt::InterpolatedText(interp) => {
                    // Should have: text, expression, text, branch, text, expression
                    assert!(interp.parts.len() >= 5);
                    assert!(matches!(&interp.parts[1], StringPart::Expression(_)));
                    assert!(matches!(&interp.parts[3], StringPart::Branch(_)));
                    assert!(matches!(&interp.parts[5], StringPart::Expression(_)));
                }
                _ => panic!("Expected InterpolatedText"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_serialize_branch_interpolation() {
    use crate::Serializer;
    use serde_json::Value;
    
    let source = r#"
        enum Gender { male, female }
        
        node Test {
            text: $"Hello {branch<Gender>(get_gender()) { male: \"Sir\", female: \"Madam\" }}"
        }
        
        fn get_gender() -> Gender
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    let json_str = Serializer::serialize_to_json(&program, false).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    
    // Check that branch interpolation is in JSON
    let text = &json["nodes"][0]["texts"][0];
    assert!(text["interpolated_parts"].is_array());
    
    let parts = text["interpolated_parts"].as_array().unwrap();
    assert!(parts.len() >= 2);
    
    // Find the branch part
    let branch_part = parts.iter().find(|p| p["type"] == "branch");
    assert!(branch_part.is_some());
    
    let branch = branch_part.unwrap();
    assert_eq!(branch["enum_type"], "Gender");
    assert_eq!(branch["function_name"], "get_gender");
    assert!(branch["branches"].is_array());
    
    let branches = branch["branches"].as_array().unwrap();
    assert_eq!(branches.len(), 2);
    assert_eq!(branches[0]["variant"], "male");
    assert_eq!(branches[0]["text"], "Sir");
    assert_eq!(branches[1]["variant"], "female");
    assert_eq!(branches[1]["text"], "Madam");
}

#[test]
fn test_branch_error_no_enum_type() {
    let source = r#"
        node Test {
            text: $"Hello {branch(get_gender()) { male: \"Mr.\" }}"
        }
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    // Should fail because branch needs <EnumType>
    assert!(result.is_err());
}

#[test]
fn test_branch_error_no_cases() {
    let source = r#"
        enum Gender { male, female }
        
        node Test {
            text: $"Hello {branch<Gender>(get_gender()) { }}"
        }
        
        fn get_gender() -> Gender
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    // Should fail because branch needs at least one case
    assert!(result.is_err());
}

#[test]
fn test_branch_with_complex_enum_names() {
    let source = r#"
        enum PlayerStatus { active, inactive, banned }
        
        node Test {
            text: $"Status: {branch<PlayerStatus>(check_status()) { active: \"Active\", inactive: \"Inactive\", banned: \"Banned\" }}"
        }
        
        fn check_status() -> PlayerStatus
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_branches_in_one_text() {
    let source = r#"
        enum Gender { male, female }
        enum Count { one, many }
        
        node Test {
            text: $"{branch<Gender>(get_gender()) { male: \"He\", female: \"She\" }} has {branch<Count>(get_count()) { one: \"1 item\", many: \"many items\" }}"
        }
        
        fn get_gender() -> Gender
        fn get_count() -> Count
    "#;
    
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    
    match &program.body[2] {
        TopLevel::NodeDef(node) => {
            match &node.body[0] {
                NodeStmt::InterpolatedText(interp) => {
                    // Count branch parts
                    let branch_count = interp.parts.iter().filter(|p| matches!(p, StringPart::Branch(_))).count();
                    assert_eq!(branch_count, 2);
                }
                _ => panic!("Expected InterpolatedText"),
            }
        }
        _ => panic!("Expected NodeDef"),
    }
}
