//! Contains property-style fuzz tests for the Mortar compiler.
//! It generates arbitrary identifiers, conditions, declarations, and small valid
//! programs so parser and serializer code paths can be stress-tested against a
//! broad input space instead of a fixed set of handwritten samples.
//!
//! 包含 Mortar 编译器的属性式 fuzz 测试。它会生成任意标识符、条件、声明
//! 和小型合法程序，让解析器和序列化器面对广泛输入空间接受压力测试，而不是只依赖少量
//! 手写样例。

use proptest::prelude::*;

use crate::Serializer;
use crate::parser::ParseHandler;

// --- Strategy helpers ---

fn arb_identifier() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{0,15}"
        .prop_map(|s| s.to_string())
        .prop_filter("not a keyword", |s| {
            !matches!(
                s.as_str(),
                "if" | "else"
                    | "fn"
                    | "function"
                    | "let"
                    | "const"
                    | "node"
                    | "nd"
                    | "true"
                    | "false"
                    | "text"
                    | "choice"
                    | "event"
                    | "events"
                    | "return"
                    | "break"
                    | "match"
                    | "for"
                    | "while"
                    | "loop"
                    | "continue"
                    | "enum"
                    | "branch"
                    | "pub"
                    | "public"
                    | "run"
                    | "with"
                    | "now"
                    | "when"
                    | "wait"
                    | "index"
                    | "action"
                    | "duration"
                    | "timeline"
                    | "tl"
            )
        })
}

fn arb_type() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("Number"), Just("String"), Just("Bool"),]
}

fn arb_number_literal() -> impl Strategy<Value = String> {
    prop_oneof![
        (0i64..=999).prop_map(|n| n.to_string()),
        (0.0f64..=999.0)
            .prop_map(|n| format!("{:.1}", n))
            .prop_filter("not NaN", |s| !s.contains("NaN")),
    ]
}

fn _arb_string_literal() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 _]{0,30}".prop_map(|s| format!("\"{}\"", s))
}

fn arb_comparison_op() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just(">"),
        Just("<"),
        Just(">="),
        Just("<="),
        Just("=="),
        Just("!="),
    ]
}

fn arb_value() -> impl Strategy<Value = String> {
    prop_oneof![
        arb_number_literal(),
        Just("true".to_string()),
        Just("false".to_string()),
    ]
}

fn arb_if_condition() -> impl Strategy<Value = String> {
    let leaf = prop_oneof![
        arb_identifier(),
        Just("true".to_string()),
        Just("false".to_string()),
        // func call: ident()
        arb_identifier().prop_map(|name| format!("{}()", name)),
    ];

    leaf.prop_recursive(3, 32, 4, |inner| {
        prop_oneof![
            // binary comparison: expr op expr
            (inner.clone(), arb_comparison_op(), arb_value())
                .prop_map(|(l, op, r)| format!("{} {} {}", l, op, r)),
            // logical and
            (inner.clone(), inner.clone()).prop_map(|(l, r)| format!("{} && {}", l, r)),
            // logical or
            (inner.clone(), inner.clone()).prop_map(|(l, r)| format!("{} || {}", l, r)),
            // negation
            inner.clone().prop_map(|e| format!("!{}", e)),
            // parenthesized
            inner.prop_map(|e| format!("({})", e)),
        ]
    })
}

fn arb_func_decl() -> impl Strategy<Value = String> {
    (arb_identifier(), arb_type()).prop_map(|(name, ret)| format!("fn {}() -> {}", name, ret))
}

fn arb_var_decl() -> impl Strategy<Value = String> {
    (arb_identifier(), arb_type()).prop_map(|(name, ty)| format!("let {}: {}", name, ty))
}

fn arb_text_stmt() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 !?.]{1,40}".prop_map(|text| format!("    text: \"{}\"", text))
}

fn arb_node_body() -> impl Strategy<Value = String> {
    prop::collection::vec(arb_text_stmt(), 1..=3).prop_map(|stmts| stmts.join("\n"))
}

fn arb_simple_program() -> impl Strategy<Value = String> {
    (
        prop::collection::vec(arb_var_decl(), 0..=3),
        arb_identifier(),
        arb_node_body(),
    )
        .prop_map(|(vars, node_name, body)| {
            let var_section = vars.join("\n");
            if var_section.is_empty() {
                format!("node {} {{\n{}\n}}", node_name, body)
            } else {
                format!("{}\n\nnode {} {{\n{}\n}}", var_section, node_name, body)
            }
        })
}

fn arb_program_with_if() -> impl Strategy<Value = String> {
    (
        prop::collection::vec(arb_var_decl(), 0..=2),
        arb_identifier(),
        arb_if_condition(),
        arb_text_stmt(),
        arb_text_stmt(),
    )
        .prop_map(|(vars, node_name, cond, then_text, else_text)| {
            let var_section = vars.join("\n");
            let node = format!(
                "node {} {{\n    if {} {{\n    {}\n    }} else {{\n    {}\n    }}\n}}",
                node_name, cond, then_text, else_text
            );
            if var_section.is_empty() {
                node
            } else {
                format!("{}\n\n{}", var_section, node)
            }
        })
}

// --- Property tests ---

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    /// Parser must never panic on arbitrary strings.
    #[test]
    fn parser_never_panics_on_arbitrary_input(input in "\\PC{0,200}") {
        let _ = ParseHandler::parse_source_code(&input, false);
    }

    /// Parser must never panic on strings that look like mortar fragments.
    #[test]
    fn parser_never_panics_on_mortar_fragments(
        input in prop_oneof![
            "node [a-z]{1,10} \\{ [a-zA-Z0-9:\" \\n]{0,50} \\}",
            "fn [a-z]{1,10}\\(\\) -> (Number|String|Bool)",
            "let [a-z]{1,10}: (Number|String|Bool)",
            "if [a-z]{1,10} [><=!]{1,2} [0-9]{1,3} \\{ text: \"[a-z ]{1,20}\" \\}",
        ]
    ) {
        let _ = ParseHandler::parse_source_code(&input, false);
    }

    /// Valid simple programs must parse successfully.
    #[test]
    fn valid_simple_programs_parse(source in arb_simple_program()) {
        let result = ParseHandler::parse_source_code(&source, false);
        prop_assert!(result.is_ok(), "Failed to parse:\n{}\nError: {:?}", source, result.err());
    }

    /// Valid programs must survive serialize → JSON roundtrip.
    #[test]
    fn serialize_roundtrip_no_panic(source in arb_simple_program()) {
        let result = ParseHandler::parse_source_code(&source, false);
        if let Ok(program) = result {
            let json_result = Serializer::serialize_to_json(&program, false);
            prop_assert!(json_result.is_ok(), "Serialization failed for:\n{}", source);

            let json_str = json_result.unwrap();
            let parse_result: Result<serde_json::Value, _> = serde_json::from_str(&json_str);
            prop_assert!(parse_result.is_ok(), "JSON output is not valid JSON");
        }
    }

    /// Programs with if-conditions should parse when variables are declared.
    #[test]
    fn programs_with_if_parse(source in arb_program_with_if()) {
        // This may or may not parse depending on whether variables referenced
        // in conditions are declared. We only assert no panics.
        let _ = ParseHandler::parse_source_code(&source, false);
    }

    /// Function declarations must parse.
    #[test]
    fn func_decl_parses(decl in arb_func_decl()) {
        let source = format!("{}\nnode T {{ text: \"x\" }}", decl);
        let result = ParseHandler::parse_source_code(&source, false);
        prop_assert!(result.is_ok(), "Failed to parse func decl:\n{}\nError: {:?}", source, result.err());
    }

    /// Serialized JSON must be valid JSON and contain expected structure.
    #[test]
    fn serialized_json_structure_valid(source in arb_simple_program()) {
        if let Ok(program) = ParseHandler::parse_source_code(&source, false)
            && let Ok(json_str) = Serializer::serialize_to_json(&program, false)
        {
            let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            prop_assert!(json.get("nodes").is_some(), "Missing 'nodes' in JSON output");
        }
    }

    /// If-condition expressions with func calls must parse and serialize.
    #[test]
    fn func_call_in_if_roundtrip(
        func_name in arb_identifier(),
        var_name in arb_identifier(),
    ) {
        let source = format!(
            "fn {}() -> Bool\nlet {}: Number\n\nnode T {{\n    if {}() {{\n        text: \"yes\"\n    }}\n}}",
            func_name, var_name, func_name,
        );
        let result = ParseHandler::parse_source_code(&source, false);
        prop_assert!(result.is_ok(), "Failed to parse func_call in if:\n{}\nError: {:?}", source, result.err());

        let program = result.unwrap();
        let json_result = Serializer::serialize_to_json(&program, false);
        prop_assert!(json_result.is_ok(), "Failed to serialize:\n{}", source);
    }

    /// Func call with comparison operator in if-condition.
    #[test]
    fn func_call_comparison_in_if(
        func_name in arb_identifier(),
        op in arb_comparison_op(),
        value in arb_number_literal(),
    ) {
        let source = format!(
            "fn {}() -> Number\n\nnode T {{\n    if {}() {} {} {{\n        text: \"ok\"\n    }}\n}}",
            func_name, func_name, op, value,
        );
        let result = ParseHandler::parse_source_code(&source, false);
        prop_assert!(result.is_ok(), "Failed:\n{}\nError: {:?}", source, result.err());
    }

    /// Negated func call in if-condition.
    #[test]
    fn negated_func_call_in_if(func_name in arb_identifier()) {
        let source = format!(
            "fn {}() -> Bool\n\nnode T {{\n    if !{}() {{\n        text: \"no\"\n    }}\n}}",
            func_name, func_name,
        );
        let result = ParseHandler::parse_source_code(&source, false);
        prop_assert!(result.is_ok(), "Failed:\n{}\nError: {:?}", source, result.err());
    }
}

// --- Deterministic edge-case tests ---

#[test]
fn empty_source_does_not_panic() {
    let _ = ParseHandler::parse_source_code("", false);
}

#[test]
fn whitespace_only_does_not_panic() {
    let _ = ParseHandler::parse_source_code("   \n\t\n  ", false);
}

#[test]
fn comment_only_does_not_panic() {
    let _ = ParseHandler::parse_source_code("// just a comment\n// another", false);
}

#[test]
fn deeply_nested_if_does_not_panic() {
    let mut source = String::from("let a: Bool\nnode T {\n");
    for _ in 0..20 {
        source.push_str("if a {\n");
    }
    source.push_str("text: \"deep\"\n");
    for _ in 0..20 {
        source.push_str("}\n");
    }
    source.push('}');
    let _ = ParseHandler::parse_source_code(&source, false);
}

#[test]
fn unclosed_brace_does_not_panic() {
    let _ = ParseHandler::parse_source_code("node T { text: \"x\"", false);
}

#[test]
fn unclosed_string_does_not_panic() {
    let _ = ParseHandler::parse_source_code("node T { text: \"unclosed }", false);
}

#[test]
fn null_bytes_in_source_does_not_panic() {
    let _ = ParseHandler::parse_source_code("node T { text: \"a\0b\" }", false);
}

#[test]
fn unicode_identifiers_do_not_panic() {
    let _ = ParseHandler::parse_source_code("node 你好 { text: \"世界\" }", false);
}

#[test]
fn extremely_long_identifier_does_not_panic() {
    let long_name: String = std::iter::repeat_n('a', 10000).collect();
    let source = format!("node {} {{ text: \"x\" }}", long_name);
    let _ = ParseHandler::parse_source_code(&source, false);
}

#[test]
fn func_call_with_many_args_does_not_panic() {
    let args: Vec<String> = (0..50).map(|i| format!("\"arg{}\"", i)).collect();
    let source = format!(
        "fn big_func({})\nnode T {{ text: \"x\" }}",
        (0..50)
            .map(|i| format!("a{}: String", i))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = ParseHandler::parse_source_code(&source, false);

    let source2 = format!(
        "fn big_func({}) -> Bool\nnode T {{ if big_func({}) {{ text: \"x\" }} }}",
        (0..50)
            .map(|i| format!("a{}: String", i))
            .collect::<Vec<_>>()
            .join(", "),
        args.join(", ")
    );
    let _ = ParseHandler::parse_source_code(&source2, false);
}

#[test]
fn consecutive_operators_do_not_panic() {
    let _ = ParseHandler::parse_source_code(
        "let a: Number\nnode T { if a >> 0 { text: \"x\" } }",
        false,
    );
    let _ = ParseHandler::parse_source_code(
        "let a: Number\nnode T { if a >< 0 { text: \"x\" } }",
        false,
    );
    let _ =
        ParseHandler::parse_source_code("let a: Bool\nnode T { if !!a { text: \"x\" } }", false);
}

#[test]
fn empty_func_call_parens_in_if_does_not_panic() {
    let source = "fn f() -> Bool\nnode T { if f() { text: \"x\" } }";
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok());
}

#[test]
fn chained_comparison_does_not_panic() {
    // a > b > c is not valid but should not panic
    let _ = ParseHandler::parse_source_code(
        "let a: Number\nlet b: Number\nlet c: Number\nnode T { if a > b > c { text: \"x\" } }",
        false,
    );
}

#[test]
fn mixed_func_calls_and_vars_in_condition() {
    let source = r#"
        fn get_hp() -> Number
        let threshold: Number

        node T {
            if get_hp() > threshold {
                text: "healthy"
            }
        }
    "#;
    let result = ParseHandler::parse_source_code(source, false);
    // This may or may not parse depending on how the parser handles mixed operands.
    // The key assertion is no panic.
    let _ = result;
}

#[test]
fn func_call_both_sides_of_comparison() {
    let source = r#"
        fn get_hp() -> Number
        fn get_max_hp() -> Number

        node T {
            if get_hp() >= get_max_hp() {
                text: "full"
            }
        }
    "#;
    let result = ParseHandler::parse_source_code(source, false);
    assert!(result.is_ok(), "Failed: {:?}", result.err());

    let program = result.unwrap();
    let json = Serializer::serialize_to_json(&program, false).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let cond = &parsed["nodes"][0]["content"][0]["condition"];
    assert_eq!(cond["type"], "binary");
    assert_eq!(cond["left"]["type"], "func_call");
    assert_eq!(cond["right"]["type"], "func_call");
}
