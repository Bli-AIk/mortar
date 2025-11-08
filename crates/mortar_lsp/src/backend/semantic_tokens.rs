use mortar_compiler::{Token, tokenize};
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

        for token_info in compiler_tokens {
            let token_type = self.get_semantic_token_type(&token_info.token);

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

    /// Get semantic token type from compiler lexical token
    fn get_semantic_token_type(&self, token: &Token) -> u32 {
        const KEYWORD: u32 = 0;
        const STRING: u32 = 1;
        const NUMBER: u32 = 2;
        const COMMENT: u32 = 3;
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
            | Token::When => KEYWORD,

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

            Token::Identifier(_) => VARIABLE,

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
}
