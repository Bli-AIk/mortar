use mortar_compiler::{Token, TokenInfo, tokenize};
use tower_lsp_server::lsp_types::*;

use crate::backend::Backend;

impl Backend {
    /// Analyze semantic tokens for syntax highlighting
    pub fn analyze_semantic_tokens(&self, content: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut last_line = 0u32;
        let mut last_column = 0u32;

        // Tokenize the entire document instead of processing line by line
        let compiler_tokens = tokenize(content);

        for (i, token_info) in compiler_tokens.iter().enumerate() {
            let token_type =
                self.get_semantic_token_type_with_context(&token_info.token, &compiler_tokens, i);

            // Calculate token line and column position
            let (token_line, token_column) =
                self.get_line_column_position(content, token_info.start);

            // Calculate token's UTF-16 length (instead of UTF-8 byte length)
            let token_text = &content[token_info.start..token_info.end];
            let length = token_text.encode_utf16().count() as u32;

            let delta_line = token_line - last_line;
            let delta_start = if delta_line == 0 {
                token_column - last_column
            } else {
                token_column
            };

            tokens.push(SemanticToken {
                delta_line,
                delta_start,
                length,
                token_type,
                token_modifiers_bitset: 0,
            });

            last_line = token_line;
            last_column = token_column;
        }

        tokens
    }

    /// Calculate line and column position from byte offset
    /// LSP uses UTF-16 code units for column positions
    fn get_line_column_position(&self, content: &str, offset: usize) -> (u32, u32) {
        let mut line = 0u32;
        let mut utf16_column = 0u32;

        // Use byte indices for iteration to ensure offset matching with tokenizer
        for (i, ch) in content.char_indices() {
            if i >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                utf16_column = 0;
            } else {
                // UTF-16 code unit count: most characters take 1 unit, but some Unicode characters need 2 units
                utf16_column += ch.len_utf16() as u32;
            }
        }

        (line, utf16_column)
    }

    /// Check if the current identifier is in a choice context (e.g., choice list)
    fn is_in_choice_context(&self, all_tokens: &[TokenInfo], current_index: usize) -> bool {
        // Look backward to find 'choice' keyword and corresponding structure
        let mut bracket_depth = 0;

        for i in (0..current_index).rev() {
            match all_tokens[i].token {
                Token::RightBracket => bracket_depth += 1,
                Token::LeftBracket => {
                    bracket_depth -= 1;
                    if bracket_depth < 0 {
                        // Found matching left bracket, continue looking for choice keyword
                        for j in (0..i).rev() {
                            match all_tokens[j].token {
                                Token::Choice => return true,
                                Token::LeftBrace | Token::RightBrace => break, // Crossed node boundary
                                _ => continue,
                            }
                        }
                        break;
                    }
                }
                _ => {}
            }
        }

        false
    }

    /// Get semantic token type from compiler lexical token with context awareness
    fn get_semantic_token_type_with_context(
        &self,
        token: &Token,
        all_tokens: &[TokenInfo],
        current_index: usize,
    ) -> u32 {
        const KEYWORD: u32 = 0;
        const STRING: u32 = 1;
        const NUMBER: u32 = 2;
        const COMMENT: u32 = 3;
        const FUNCTION: u32 = 4;
        const VARIABLE: u32 = 5;
        const METHOD: u32 = 6; // Used for function calls
        const OPERATOR: u32 = 7;
        const PUNCTUATION: u32 = 8;

        match token {
            Token::SingleLineComment(_) | Token::MultiLineComment(_) => COMMENT,

            Token::Node
            | Token::Text
            | Token::Events
            | Token::Choice
            | Token::Fn
            | Token::Return
            | Token::Break
            | Token::When
            | Token::StringType
            | Token::NumberType
            | Token::BooleanType
            | Token::True
            | Token::False => KEYWORD,

            Token::String(_) => STRING,

            Token::Number(_) => NUMBER,

            Token::Arrow => OPERATOR,

            Token::Colon
            | Token::Comma
            | Token::Semicolon
            | Token::Dot
            | Token::LeftBrace
            | Token::RightBrace
            | Token::LeftBracket
            | Token::RightBracket
            | Token::LeftParen
            | Token::RightParen => PUNCTUATION,

            Token::Identifier(_) => {
                // Check if it's an identifier after node/nd or fn (function/node definition)
                if current_index > 0
                    && let Some(prev_token_info) = all_tokens.get(current_index - 1)
                {
                    match prev_token_info.token {
                        Token::Node | Token::Fn => return FUNCTION,
                        _ => {}
                    }
                }

                // Check if it's a function call (identifier followed by left parenthesis)
                if current_index + 1 < all_tokens.len()
                    && let Some(next_token_info) = all_tokens.get(current_index + 1)
                    && matches!(next_token_info.token, Token::LeftParen)
                {
                    return METHOD;
                }

                // Check if it's a node call (identifier in choice or jump context)
                // In this case, identifier usually appears after arrow (->) or comma
                if current_index > 0
                    && let Some(prev_token_info) = all_tokens.get(current_index - 1)
                {
                    match prev_token_info.token {
                        Token::Arrow => return METHOD, // Node jump
                        Token::Comma => {
                            // Node reference in choice list
                            // Check if previous tokens indicate this is a choice context
                            if self.is_in_choice_context(all_tokens, current_index) {
                                return METHOD;
                            }
                        }
                        _ => {}
                    }
                }

                VARIABLE
            }

            Token::Error => KEYWORD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_tokenization() {
        // Test comment tokenization containing Chinese characters
        let content = r#"    text: "你好呀，欢迎阅读这个互动故事。"
    
    // 这个事件列表写在紧挨着上一个 text，所以它们是关联的。"#;

        let tokens = tokenize(content);

        // Find comment tokens
        let comment_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| matches!(t.token, Token::SingleLineComment(_)))
            .collect();

        assert_eq!(
            comment_tokens.len(),
            1,
            "Should have exactly one comment token"
        );

        let comment_token = comment_tokens[0];
        // Verify comment completeness
        assert_eq!(
            comment_token.text,
            "// 这个事件列表写在紧挨着上一个 text，所以它们是关联的。"
        );
    }

    #[test]
    fn test_utf16_position_calculation() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(Backend::new);
        let backend = Backend::new(service.inner().client.clone());

        // Test position calculation with Chinese characters
        let content = "你好text"; // "你" and "好" are Chinese characters

        // "你" at position 0 (0 bytes)
        let (line, col) = backend.get_line_column_position(content, 0);
        assert_eq!((line, col), (0, 0));

        // "好" at position 3 (3 bytes, "你" is 3 bytes in UTF-8)
        let (line, col) = backend.get_line_column_position(content, 3);
        assert_eq!((line, col), (0, 1)); // UTF-16: "你" takes 1 code unit

        // "text" at position 6 (6 bytes, "你好" is 6 bytes in UTF-8)
        let (line, col) = backend.get_line_column_position(content, 6);
        assert_eq!((line, col), (0, 2)); // UTF-16: "你好" takes 2 code units
    }

    #[test]
    fn test_semantic_tokens_utf16_length_calculation() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(Backend::new);
        let backend = Backend::new(service.inner().client.clone());

        // Test semantic token length calculation with Chinese comments
        let content = "// 然后，由于没有任何后续节点，这个对话还是结束了。";

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        assert_eq!(semantic_tokens.len(), 1, "Should have one semantic token");

        let comment_token = &semantic_tokens[0];
        assert_eq!(comment_token.token_type, 3, "Should be comment type");

        // Verify UTF-16 length calculation is correct
        let expected_utf16_length = content.encode_utf16().count() as u32;
        assert_eq!(
            comment_token.length, expected_utf16_length,
            "Comment token UTF-16 length calculation should be correct"
        );
    }

    #[test]
    fn test_type_keywords_highlighting() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(Backend::new);
        let backend = Backend::new(service.inner().client.clone());

        // Test type keyword and boolean literal highlighting
        let content = r#"fn process_data(name: String, count: Number, active: Boolean) -> Boolean
fn get_status() -> String

node test {
    text: "Result is true"
    text: "Status is false"
}"#;

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        // Find all keyword type tokens (token_type = 0)
        let keyword_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 0) // KEYWORD = 0
            .collect();

        // Should contain: fn, String, Number, Boolean, Boolean, fn, String, node, true, false
        // Note: true and false as string content won't be recognized as keywords, only as independent tokens
        assert!(
            keyword_tokens.len() >= 8,
            "Should have at least 8 keyword tokens, actual: {}",
            keyword_tokens.len()
        );

        // Find all function type tokens (token_type = 4)
        let function_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 4) // FUNCTION = 4
            .collect();

        // Should have 3 function tokens: process_data, get_status, test
        assert_eq!(
            function_tokens.len(),
            3,
            "Should have 3 function name tokens"
        );
    }

    #[test]
    fn test_original_function_name_highlighting() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(Backend::new);
        let backend = Backend::new(service.inner().client.clone());

        // Test node definition and function definition name highlighting
        let content = r#"node start_game {
    text: "Hello"
}

nd another_node {
    text: "World"
}

fn play_sound(file: String)
fn get_name() -> String"#;

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        // Find all function type tokens (token_type = 4)
        let function_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 4) // FUNCTION = 4
            .collect();

        // Should have 4 function tokens: start_game, another_node, play_sound, get_name
        assert_eq!(
            function_tokens.len(),
            4,
            "Should have 4 function name tokens"
        );
    }

    #[test]
    fn test_function_call_highlighting() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(Backend::new);
        let backend = Backend::new(service.inner().client.clone());

        // Test function call highlighting
        let content = "node start_game {\n    text: \"Hello world\"\n    events: [\n        0, play_sound(\"greeting.wav\")\n    ]\n}\n\nfn play_sound(file: String)";

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        // Find all method call type tokens (token_type = 6)
        let function_call_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 6) // METHOD = 6
            .collect();

        // Should have play_sound function call
        assert!(
            !function_call_tokens.is_empty(),
            "Should have at least 1 function call token, actual: {}",
            function_call_tokens.len()
        );
    }

    #[test]
    fn test_node_jump_highlighting() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(Backend::new);
        let backend = Backend::new(service.inner().client.clone());

        // Test node jump highlighting
        let content = "node start {\n    text: \"Beginning\"\n} -> middle_node\n\nnode middle {\n    text: \"Middle\"\n}";

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        // Find all method call type tokens (including node calls)
        let function_call_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 6) // METHOD = 6
            .collect();

        // Should have middle_node node call
        assert!(
            !function_call_tokens.is_empty(),
            "Should have at least 1 node call token, actual: {}",
            function_call_tokens.len()
        );
    }
}
