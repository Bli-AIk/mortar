#[cfg(test)]
mod tests {
    use crate::token::*;
    use logos::Logos;

    // ---------------------------
    // Mortar Language Token Test
    // ---------------------------

    #[test]
    fn test_keywords() {
        let input = "node nd text events choice fn return break when";
        let expected = vec![
            Token::Node,
            Token::Node,
            Token::Text,
            Token::Events,
            Token::Choice,
            Token::Fn,
            Token::Return,
            Token::Break,
            Token::When,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators_and_punctuation() {
        let input = "-> : , . { } [ ] ( )";
        let expected = vec![
            Token::Arrow,
            Token::Colon,
            Token::Comma,
            Token::Dot,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::LeftParen,
            Token::RightParen,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello world" 'single quotes' "escaped \" quote""#;
        let expected = vec![
            Token::String("hello world"),
            Token::String("single quotes"),
            Token::String("escaped \\\" quote"),
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_numbers() {
        let input = "42 3.14 0 123.456";
        let expected = vec![
            Token::Number("42"),
            Token::Number("3.14"),
            Token::Number("0"),
            Token::Number("123.456"),
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "my_ident another123 play_sound set_animation get_name _private";
        let expected = vec![
            Token::Identifier("my_ident"),
            Token::Identifier("another123"),
            Token::Identifier("play_sound"),
            Token::Identifier("set_animation"),
            Token::Identifier("get_name"),
            Token::Identifier("_private"),
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments_ignored() {
        let input = "node // single line comment\n text /* multi-line comment */ events";
        let expected = vec![Token::Node, Token::Text, Token::Events];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_whitespace_ignored() {
        let input = "  node\t\ttext\r\n  events  ";
        let expected = vec![Token::Node, Token::Text, Token::Events];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_simple_node_definition() {
        let input = r#"node start {
            text: "Hello world"
        }"#;

        let expected = vec![
            Token::Node,
            Token::Identifier("start"),
            Token::LeftBrace,
            Token::Text,
            Token::Colon,
            Token::String("Hello world"),
            Token::RightBrace,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_node_with_arrow() {
        let input = "node start { } -> end_node";
        let expected = vec![
            Token::Node,
            Token::Identifier("start"),
            Token::LeftBrace,
            Token::RightBrace,
            Token::Arrow,
            Token::Identifier("end_node"),
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_events_with_numbers() {
        let input = r#"events: [
            0, play_sound("greeting.wav")
            6.5, set_animation("wave")
        ]"#;

        let expected = vec![
            Token::Events,
            Token::Colon,
            Token::LeftBracket,
            Token::Number("0"),
            Token::Comma,
            Token::Identifier("play_sound"),
            Token::LeftParen,
            Token::String("greeting.wav"),
            Token::RightParen,
            Token::Number("6.5"),
            Token::Comma,
            Token::Identifier("set_animation"),
            Token::LeftParen,
            Token::String("wave"),
            Token::RightParen,
            Token::RightBracket,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_choice_structure() {
        let input = r#"choice: [
            "Option 1" -> node1,
            "Option 2" when condition -> node2
        ]"#;

        let expected = vec![
            Token::Choice,
            Token::Colon,
            Token::LeftBracket,
            Token::String("Option 1"),
            Token::Arrow,
            Token::Identifier("node1"),
            Token::Comma,
            Token::String("Option 2"),
            Token::When,
            Token::Identifier("condition"),
            Token::Arrow,
            Token::Identifier("node2"),
            Token::RightBracket,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_definition() {
        let input = "fn play_sound(file_name: String)";
        let expected = vec![
            Token::Fn,
            Token::Identifier("play_sound"),
            Token::LeftParen,
            Token::Identifier("file_name"),
            Token::Colon,
            Token::Identifier("String"),
            Token::RightParen,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_function_with_return_type() {
        let input = "fn get_name() -> String";
        let expected = vec![
            Token::Fn,
            Token::Identifier("get_name"),
            Token::LeftParen,
            Token::RightParen,
            Token::Arrow,
            Token::Identifier("String"),
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_complex_mortar_snippet() {
        let input = r#"
        node start {
            text: "Hello and welcome to this interactive story."
            events: [
                0, play_sound("greeting.wav")
                6, set_animation("wave").play_sound("wave_sound.wav")
            ]
        } -> choice_point
        "#;

        let expected = vec![
            Token::Node,
            Token::Identifier("start"),
            Token::LeftBrace,
            Token::Text,
            Token::Colon,
            Token::String("Hello and welcome to this interactive story."),
            Token::Events,
            Token::Colon,
            Token::LeftBracket,
            Token::Number("0"),
            Token::Comma,
            Token::Identifier("play_sound"),
            Token::LeftParen,
            Token::String("greeting.wav"),
            Token::RightParen,
            Token::Number("6"),
            Token::Comma,
            Token::Identifier("set_animation"),
            Token::LeftParen,
            Token::String("wave"),
            Token::RightParen,
            Token::Dot,
            Token::Identifier("play_sound"),
            Token::LeftParen,
            Token::String("wave_sound.wav"),
            Token::RightParen,
            Token::RightBracket,
            Token::RightBrace,
            Token::Arrow,
            Token::Identifier("choice_point"),
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_break_and_return_keywords() {
        let input = r#"choice: [
            "Continue" -> next_node,
            "Exit" -> return,
            "Stop" -> break
        ]"#;

        let expected = vec![
            Token::Choice,
            Token::Colon,
            Token::LeftBracket,
            Token::String("Continue"),
            Token::Arrow,
            Token::Identifier("next_node"),
            Token::Comma,
            Token::String("Exit"),
            Token::Arrow,
            Token::Return,
            Token::Comma,
            Token::String("Stop"),
            Token::Arrow,
            Token::Break,
            Token::RightBracket,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_error_handling() {
        let input = "node start { @invalid_symbol }";
        let mut lexer = Token::lexer(input);

        //Token parsed normally
        assert_eq!(lexer.next(), Some(Ok(Token::Node)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("start"))));
        assert_eq!(lexer.next(), Some(Ok(Token::LeftBrace)));

        // Invalid symbols should generate an error
        assert_eq!(lexer.next(), Some(Err(())));
    }

    #[test]
    fn test_token_display() {
        assert_eq!(format!("{}", Token::Node), "node");
        assert_eq!(format!("{}", Token::Arrow), "->");
        assert_eq!(format!("{}", Token::String("test")), "\"test\"");
        assert_eq!(format!("{}", Token::Number("42")), "42");
        assert_eq!(format!("{}", Token::Identifier("my_var")), "my_var");
        assert_eq!(format!("{}", Token::LeftBrace), "{");
        assert_eq!(format!("{}", Token::RightBrace), "}");
    }

    #[test]
    fn test_multiline_comment() {
        let input = r#"
        node start /* this is a 
        multiline comment */ {
            text: "Hello"
        }
        "#;

        let expected = vec![
            Token::Node,
            Token::Identifier("start"),
            Token::LeftBrace,
            Token::Text,
            Token::Colon,
            Token::String("Hello"),
            Token::RightBrace,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_empty_string() {
        let input = r#""""#;
        let expected = vec![Token::String("")];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_nd_abbreviation() {
        let input = "nd quick_node { text: \"fast_node\" }";
        let expected = vec![
            Token::Node,
            Token::Identifier("quick_node"),
            Token::LeftBrace,
            Token::Text,
            Token::Colon,
            Token::String("fast_node"),
            Token::RightBrace,
        ];

        let tokens = Token::lexer(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_lexer_output_function() {
        // Test that lex_with_output function does not crash
        let input = "node test { text: \"Hello\" }";
        let tokens = lex_with_output(input);

        // Check whether the number of tokens returned is correct
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::Node);
        assert_eq!(tokens[1], Token::Identifier("test"));
    }
}
