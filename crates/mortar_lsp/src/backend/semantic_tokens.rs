use mortar_compiler::{Token, TokenInfo, tokenize};
use tower_lsp_server::lsp_types::*;

use crate::backend::Backend;

impl Backend {
    /// Analyze semantic tokens for syntax highlighting
    pub fn analyze_semantic_tokens(&self, content: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut last_line = 0u32;
        let mut last_column = 0u32;

        // 对整个文档进行tokenize，而不是逐行处理
        let compiler_tokens = tokenize(content);

        for (i, token_info) in compiler_tokens.iter().enumerate() {
            let token_type =
                self.get_semantic_token_type_with_context(&token_info.token, &compiler_tokens, i);

            // 计算token的行列位置
            let (token_line, token_column) =
                self.get_line_column_position(content, token_info.start);

            // 计算token的UTF-16长度（而不是UTF-8字节长度）
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

        // 使用byte indices来遍历，确保与tokenizer的offset匹配
        for (i, ch) in content.char_indices() {
            if i >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                utf16_column = 0;
            } else {
                // UTF-16编码单位数量：大部分字符是1个单位，但某些Unicode字符需要2个单位
                utf16_column += ch.len_utf16() as u32;
            }
        }

        (line, utf16_column)
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
            | Token::Dot
            | Token::LeftBrace
            | Token::RightBrace
            | Token::LeftBracket
            | Token::RightBracket
            | Token::LeftParen
            | Token::RightParen => PUNCTUATION,

            Token::Identifier(_) => {
                // 检查是否是 node/nd 或 fn 后面的标识符
                if current_index > 0 {
                    if let Some(prev_token_info) = all_tokens.get(current_index - 1) {
                        match prev_token_info.token {
                            Token::Node | Token::Fn => return FUNCTION,
                            _ => {}
                        }
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
        // 测试包含中文字符的注释tokenization
        let content = r#"    text: "你好呀，欢迎阅读这个互动故事。"
    
    // 这个事件列表写在紧挨着上一个 text，所以它们是关联的。"#;

        let tokens = tokenize(content);

        // 寻找注释token
        let comment_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| matches!(t.token, Token::SingleLineComment(_)))
            .collect();

        assert_eq!(comment_tokens.len(), 1, "应该有且仅有一个注释token");

        let comment_token = comment_tokens[0];
        // 验证注释的完整性
        assert_eq!(
            comment_token.text,
            "// 这个事件列表写在紧挨着上一个 text，所以它们是关联的。"
        );
    }

    #[test]
    fn test_utf16_position_calculation() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(|client| Backend::new(client));
        let backend = Backend::new(service.inner().client.clone());

        // 测试含有中文字符的位置计算
        let content = "你好text"; // "你" 和 "好" 都是中文字符

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
        let (service, _) = LspService::new(|client| Backend::new(client));
        let backend = Backend::new(service.inner().client.clone());

        // 测试包含中文注释的语义token长度计算
        let content = "// 然后，由于没有任何后续节点，这个对话还是结束了。";

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        assert_eq!(semantic_tokens.len(), 1, "应该有一个语义token");

        let comment_token = &semantic_tokens[0];
        assert_eq!(comment_token.token_type, 3, "应该是注释类型");

        // 验证UTF-16长度计算正确
        let expected_utf16_length = content.encode_utf16().count() as u32;
        assert_eq!(
            comment_token.length, expected_utf16_length,
            "注释token的UTF-16长度计算应该正确"
        );
    }

    #[test]
    fn test_type_keywords_highlighting() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(|client| Backend::new(client));
        let backend = Backend::new(service.inner().client.clone());

        // 测试类型关键字和布尔字面量的高亮
        let content = r#"fn process_data(name: String, count: Number, active: Boolean) -> Boolean
fn get_status() -> String

node test {
    text: "Result is true"
    text: "Status is false"
}"#;

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        // 找出所有关键字类型的标记（token_type = 0）
        let keyword_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 0) // KEYWORD = 0
            .collect();

        // 应该包含：fn, String, Number, Boolean, Boolean, fn, String, node, true, false
        // 注意：true 和 false 作为字符串内容不会被识别为关键字，只有作为独立token才会
        assert!(
            keyword_tokens.len() >= 8,
            "应该有至少8个关键字标记，实际有{}",
            keyword_tokens.len()
        );

        // 找出所有函数类型的标记（token_type = 4）
        let function_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 4) // FUNCTION = 4
            .collect();

        // 应该有3个函数标记：process_data, get_status, test
        assert_eq!(function_tokens.len(), 3, "应该有3个函数名标记");
    }

    #[test]
    fn test_original_function_name_highlighting() {
        use tower_lsp_server::LspService;
        let (service, _) = LspService::new(|client| Backend::new(client));
        let backend = Backend::new(service.inner().client.clone());

        // 测试节点定义和函数定义中的名称高亮
        let content = r#"node start_game {
    text: "Hello"
}

nd another_node {
    text: "World"
}

fn play_sound(file: String)
fn get_name() -> String"#;

        let semantic_tokens = backend.analyze_semantic_tokens(content);

        // 找出所有函数类型的标记（token_type = 4）
        let function_tokens: Vec<_> = semantic_tokens
            .iter()
            .filter(|token| token.token_type == 4) // FUNCTION = 4
            .collect();

        // 应该有4个函数标记：start_game, another_node, play_sound, get_name
        assert_eq!(function_tokens.len(), 4, "应该有4个函数名标记");
    }
}
