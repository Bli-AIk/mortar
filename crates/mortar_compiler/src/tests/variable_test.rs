use crate::parser::{ParseHandler, TopLevel, VarValue};

#[test]
fn test_parse_variable_declaration_without_value() {
    let source = "let player_name: String";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.body.len(), 1);

    match &program.body[0] {
        TopLevel::VarDecl(var) => {
            assert_eq!(var.name, "player_name");
            assert_eq!(var.type_name, "String");
            assert!(var.value.is_none());
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_variable_declaration_with_string_value() {
    let source = r#"let game_title: String = "My Game""#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.body.len(), 1);

    match &program.body[0] {
        TopLevel::VarDecl(var) => {
            assert_eq!(var.name, "game_title");
            assert_eq!(var.type_name, "String");
            assert!(var.value.is_some());
            match &var.value {
                Some(VarValue::String(s)) => assert_eq!(s, "My Game"),
                _ => panic!("Expected String value"),
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_variable_declaration_with_number_value() {
    let source = "let score: Number = 100";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::VarDecl(var) => {
            assert_eq!(var.name, "score");
            assert_eq!(var.type_name, "Number");
            match &var.value {
                Some(VarValue::Number(n)) => assert_eq!(*n, 100.0),
                _ => panic!("Expected Number value"),
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_variable_declaration_with_float_value() {
    let source = "let health: Number = 50.5";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::VarDecl(var) => {
            assert_eq!(var.name, "health");
            match &var.value {
                Some(VarValue::Number(n)) => assert_eq!(*n, 50.5),
                _ => panic!("Expected Number value"),
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_variable_declaration_with_bool_true() {
    let source = "let is_active: Bool = true";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::VarDecl(var) => {
            assert_eq!(var.name, "is_active");
            assert_eq!(var.type_name, "Boolean");
            match &var.value {
                Some(VarValue::Boolean(b)) => assert_eq!(*b, true),
                _ => panic!("Expected Boolean value"),
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_variable_declaration_with_bool_false() {
    let source = "let debug_mode: Boolean = false";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::VarDecl(var) => {
            assert_eq!(var.name, "debug_mode");
            assert_eq!(var.type_name, "Boolean");
            match &var.value {
                Some(VarValue::Boolean(b)) => assert_eq!(*b, false),
                _ => panic!("Expected Boolean value"),
            }
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_public_constant() {
    let source = r#"pub const welcome: String = "Hello""#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::ConstDecl(const_decl) => {
            assert_eq!(const_decl.name, "welcome");
            assert_eq!(const_decl.type_name, "String");
            assert!(const_decl.is_public);
            match &const_decl.value {
                VarValue::String(s) => assert_eq!(s, "Hello"),
                _ => panic!("Expected String value"),
            }
        }
        _ => panic!("Expected ConstDecl"),
    }
}

#[test]
fn test_parse_private_constant() {
    let source = "const max_level: Number = 99";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::ConstDecl(const_decl) => {
            assert_eq!(const_decl.name, "max_level");
            assert_eq!(const_decl.type_name, "Number");
            assert!(!const_decl.is_public);
            match &const_decl.value {
                VarValue::Number(n) => assert_eq!(*n, 99.0),
                _ => panic!("Expected Number value"),
            }
        }
        _ => panic!("Expected ConstDecl"),
    }
}

#[test]
fn test_parse_public_keyword_constant() {
    let source = r#"public const title: String = "Game""#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::ConstDecl(const_decl) => {
            assert_eq!(const_decl.name, "title");
            assert!(const_decl.is_public);
        }
        _ => panic!("Expected ConstDecl"),
    }
}

#[test]
fn test_parse_enum_definition() {
    let source = r#"enum GameState {
        menu
        playing
        paused
    }"#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::EnumDef(enum_def) => {
            assert_eq!(enum_def.name, "GameState");
            assert_eq!(enum_def.variants.len(), 3);
            assert_eq!(enum_def.variants[0], "menu");
            assert_eq!(enum_def.variants[1], "playing");
            assert_eq!(enum_def.variants[2], "paused");
        }
        _ => panic!("Expected EnumDef"),
    }
}

#[test]
fn test_parse_enum_with_single_variant() {
    let source = r#"enum Status { active }"#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::EnumDef(enum_def) => {
            assert_eq!(enum_def.name, "Status");
            assert_eq!(enum_def.variants.len(), 1);
            assert_eq!(enum_def.variants[0], "active");
        }
        _ => panic!("Expected EnumDef"),
    }
}

#[test]
fn test_parse_multiple_variables() {
    let source = r#"
        let name: String
        let score: Number = 100
        let active: Bool = true
    "#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.body.len(), 3);

    match &program.body[0] {
        TopLevel::VarDecl(var) => assert_eq!(var.name, "name"),
        _ => panic!("Expected VarDecl"),
    }

    match &program.body[1] {
        TopLevel::VarDecl(var) => assert_eq!(var.name, "score"),
        _ => panic!("Expected VarDecl"),
    }

    match &program.body[2] {
        TopLevel::VarDecl(var) => assert_eq!(var.name, "active"),
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_mixed_declarations() {
    let source = r#"
        let player_name: String
        pub const game_title: String = "My Game"
        enum State { active, paused }
    "#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.body.len(), 3);

    assert!(matches!(&program.body[0], TopLevel::VarDecl(_)));
    assert!(matches!(&program.body[1], TopLevel::ConstDecl(_)));
    assert!(matches!(&program.body[2], TopLevel::EnumDef(_)));
}

#[test]
fn test_parse_variables_with_nodes() {
    let source = r#"
        let score: Number = 0
        
        node Start {
            text: "Hello"
        }
    "#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.body.len(), 2);

    assert!(matches!(&program.body[0], TopLevel::VarDecl(_)));
    assert!(matches!(&program.body[1], TopLevel::NodeDef(_)));
}

#[test]
fn test_boolean_argument_in_function_call() {
    let source = r#"
        node Test {
            text: "Test"
            events: [
                0, set_flag(true)
            ]
        }
        
        fn set_flag(value: Bool)
    "#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());

    let program = result.unwrap();
    
    // Check that boolean argument is parsed
    match &program.body[0] {
        TopLevel::NodeDef(node) => {
            // This should have parsed without error
            assert_eq!(node.name, "Test");
        }
        _ => panic!("Expected NodeDef"),
    }
}

#[test]
fn test_variable_error_missing_colon() {
    let source = "let invalid String";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_err());
}

#[test]
fn test_constant_error_missing_value() {
    let source = "const invalid: String";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_err());
}

#[test]
fn test_enum_empty_body() {
    let source = "enum Empty { }";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
    
    let program = result.unwrap();
    match &program.body[0] {
        TopLevel::EnumDef(enum_def) => {
            assert_eq!(enum_def.name, "Empty");
            assert_eq!(enum_def.variants.len(), 0);
        }
        _ => panic!("Expected EnumDef"),
    }
}
