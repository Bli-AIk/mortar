use crate::diagnostics::{Diagnostic, DiagnosticCollector, DiagnosticKind, Severity};
use crate::token::{Token, TokenInfo};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    NodeDef(NodeDef),
    FunctionDecl(FunctionDecl),
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    EnumDef(EnumDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub body: Vec<NodeStmt>,
    pub jump: Option<NodeJump>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStmt {
    Text(String),
    InterpolatedText(InterpolatedString),
    Events(Vec<Event>),
    Choice(Vec<ChoiceItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedString {
    pub parts: Vec<StringPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Expression(FuncCall),
    Branch(BranchInterpolation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchInterpolation {
    pub enum_type: String,
    pub enum_value_expr: Box<FuncCall>,
    pub branches: Vec<BranchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchCase {
    pub variant: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeJump {
    Identifier(String, Option<(usize, usize)>),
    Return,
    Break,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub index: f64,
    pub action: EventAction,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventAction {
    pub call: FuncCall,
    pub chains: Vec<FuncCall>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceItem {
    pub text: String,
    pub condition: Option<Condition>,
    pub target: ChoiceDest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Identifier(String),
    FuncCall(FuncCall),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceDest {
    Identifier(String, Option<(usize, usize)>),
    Return,
    Break,
    NestedChoices(Vec<ChoiceItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCall {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    FuncCall(Box<FuncCall>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub type_name: String,
    pub value: Option<VarValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecl {
    pub is_public: bool,
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub type_name: String,
    pub value: VarValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

pub struct ParseHandler;

impl ParseHandler {
    pub fn parse_source_code(content: &str, verbose_lexer: bool) -> Result<Program, String> {
        let tokens = if verbose_lexer {
            crate::token::lex_with_output(content)
                .into_iter()
                .map(|token| TokenInfo {
                    token,
                    start: 0, // We'll use better position tracking later
                    end: 0,
                    text: "",
                })
                .collect()
        } else {
            crate::token::tokenize(content)
        };

        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    pub fn parse_source_code_with_diagnostics(
        content: &str,
        file_name: String,
        verbose_lexer: bool,
    ) -> (Result<Program, String>, DiagnosticCollector) {
        Self::parse_source_code_with_diagnostics_and_language(
            content,
            file_name,
            verbose_lexer,
            crate::Language::English,
        )
    }

    pub fn parse_source_code_with_diagnostics_and_language(
        content: &str,
        file_name: String,
        verbose_lexer: bool,
        language: crate::Language,
    ) -> (Result<Program, String>, DiagnosticCollector) {
        let tokens = if verbose_lexer {
            crate::token::lex_with_output(content)
                .into_iter()
                .map(|token| TokenInfo {
                    token,
                    start: 0, // We'll use better position tracking later
                    end: 0,
                    text: "",
                })
                .collect()
        } else {
            crate::token::tokenize(content)
        };

        let mut parser = Parser::new(tokens);
        let mut diagnostics = DiagnosticCollector::new_with_language(file_name, language);

        let result = parser.parse_program();

        // If parsing failed, add parse error to diagnostics
        if let Err(ref parse_error) = result {
            let current_span = parser.get_current_span();
            diagnostics.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::SyntaxError {
                    message: parse_error.clone(),
                },
                severity: Severity::Error,
                span: current_span,
                message: parse_error.clone(),
            });
        }

        // If parsing succeeded, run semantic analysis
        if let Ok(ref program) = result {
            diagnostics.analyze_program(program);
        }

        (result, diagnostics)
    }
}

struct Parser<'a> {
    tokens: Vec<TokenInfo<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<TokenInfo<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&TokenInfo<'_>> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&TokenInfo<'_>> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn get_current_span(&self) -> Option<(usize, usize)> {
        if let Some(token_info) = self.peek() {
            Some((token_info.start, token_info.end))
        } else if self.current > 0 {
            // If we're at the end, use the last token's position
            self.tokens
                .get(self.current - 1)
                .map(|token_info| (token_info.start, token_info.end))
        } else {
            None
        }
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(current_token) = self.peek() {
            std::mem::discriminant(&current_token.token) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    fn consume(&mut self, expected: &Token, error_msg: &str) -> Result<&TokenInfo<'_>, String> {
        if self.check(expected) {
            Ok(self.advance().unwrap())
        } else {
            Err(format!(
                "{}: expected {:?}, found {:?}",
                error_msg,
                expected,
                self.peek().map(|t| &t.token)
            ))
        }
    }

    /// Skip optional separators (commas and semicolons)
    fn skip_optional_separators(&mut self) {
        while let Some(token_info) = self.peek() {
            if matches!(token_info.token, Token::Comma | Token::Semicolon) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip comments and optional separators
    fn skip_comments_and_separators(&mut self) {
        loop {
            let mut skipped_something = false;

            // Skip comments
            while let Some(token_info) = self.peek() {
                if matches!(
                    token_info.token,
                    Token::SingleLineComment(_) | Token::MultiLineComment(_)
                ) {
                    self.advance();
                    skipped_something = true;
                } else {
                    break;
                }
            }

            // Skip separators
            while let Some(token_info) = self.peek() {
                if matches!(token_info.token, Token::Comma | Token::Semicolon) {
                    self.advance();
                    skipped_something = true;
                } else {
                    break;
                }
            }

            if !skipped_something {
                break;
            }
        }
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut body = Vec::new();

        while !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.is_at_end() {
                body.push(self.parse_top_level()?);
            }
        }

        Ok(Program { body })
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        self.skip_comments_and_separators();

        match self.peek().map(|t| &t.token) {
            Some(Token::Node) => Ok(TopLevel::NodeDef(self.parse_node_def()?)),
            Some(Token::Fn) => Ok(TopLevel::FunctionDecl(self.parse_function_decl()?)),
            Some(Token::Let) => Ok(TopLevel::VarDecl(self.parse_var_decl()?)),
            Some(Token::Const) | Some(Token::Pub) => Ok(TopLevel::ConstDecl(self.parse_const_decl()?)),
            Some(Token::Enum) => Ok(TopLevel::EnumDef(self.parse_enum_def()?)),
            _ => Err(format!(
                "Expected 'node', 'fn', 'let', 'const', 'pub', or 'enum', found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_node_def(&mut self) -> Result<NodeDef, String> {
        self.consume(&Token::Node, "Expected 'node'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected identifier after 'node'".to_string());
            }
        } else {
            return Err("Expected identifier after 'node'".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBrace) && !self.is_at_end() {
                body.push(self.parse_node_stmt()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        let jump = if self.check(&Token::Arrow) {
            Some(self.parse_node_jump()?)
        } else {
            None
        };

        Ok(NodeDef {
            name,
            name_span,
            body,
            jump,
        })
    }

    fn parse_node_stmt(&mut self) -> Result<NodeStmt, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Text) => Ok(self.parse_text_stmt()?),
            Some(Token::Events) => Ok(NodeStmt::Events(self.parse_events_stmt()?)),
            Some(Token::Choice) => Ok(NodeStmt::Choice(self.parse_choice_stmt()?)),
            _ => Err(format!(
                "Expected 'text', 'events', or 'choice', found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_text_stmt(&mut self) -> Result<NodeStmt, String> {
        self.consume(&Token::Text, "Expected 'text'")?;
        self.consume(&Token::Colon, "Expected ':'")?;

        if let Some(token_info) = self.advance() {
            match &token_info.token {
                Token::String(text) => Ok(NodeStmt::Text(text.to_string())),
                Token::InterpolatedString(text) => {
                    let text_copy = text.to_string(); // Make a copy to avoid borrow issues
                    let interpolated = self.parse_interpolated_string(&text_copy)?;
                    Ok(NodeStmt::InterpolatedText(interpolated))
                }
                _ => Err("Expected string or interpolated string after 'text:'".to_string()),
            }
        } else {
            Err("Expected string or interpolated string after 'text:'".to_string())
        }
    }

    fn parse_interpolated_string(&mut self, text: &str) -> Result<InterpolatedString, String> {
        let mut parts = Vec::new();
        let mut chars = text.chars().peekable();
        let mut current_text = String::new();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Save any accumulated text
                if !current_text.is_empty() {
                    parts.push(StringPart::Text(current_text.clone()));
                    current_text.clear();
                }

                // Parse expression until '}'
                let mut expr_text = String::new();
                let mut brace_count = 1;

                for expr_ch in chars.by_ref() {
                    if expr_ch == '{' {
                        brace_count += 1;
                        expr_text.push(expr_ch);
                    } else if expr_ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                        expr_text.push(expr_ch);
                    } else {
                        expr_text.push(expr_ch);
                    }
                }

                if brace_count != 0 {
                    return Err("Unmatched '{' in interpolated string".to_string());
                }

                // Check if this is a branch interpolation
                if expr_text.trim().starts_with("branch") {
                    let branch = self.parse_branch_from_string(&expr_text)?;
                    parts.push(StringPart::Branch(branch));
                } else {
                    // Parse the expression as a function call
                    let func_call = self.parse_expression_from_string(&expr_text)?;
                    parts.push(StringPart::Expression(func_call));
                }
            } else {
                current_text.push(ch);
            }
        }

        // Save any remaining text
        if !current_text.is_empty() {
            parts.push(StringPart::Text(current_text));
        }

        Ok(InterpolatedString { parts })
    }

    fn parse_expression_from_string(&mut self, expr_text: &str) -> Result<FuncCall, String> {
        // Simple parsing of "function_name()" or "function_name(args)"
        let expr_text = expr_text.trim();

        if let Some(paren_pos) = expr_text.find('(') {
            let func_name = expr_text[..paren_pos].trim();
            let args_part = &expr_text[paren_pos + 1..];

            if !args_part.ends_with(')') {
                return Err("Expected ')' at end of function call".to_string());
            }

            let args_part = &args_part[..args_part.len() - 1].trim();
            let args = if args_part.is_empty() {
                Vec::new()
            } else {
                // For now, only support simple arguments (this could be expanded)
                self.parse_simple_args(args_part)?
            };

            Ok(FuncCall {
                name: func_name.to_string(),
                name_span: None, // We don't have precise span info from string parsing
                args,
            })
        } else {
            Err("Expression in interpolated string must be a function call".to_string())
        }
    }

    fn parse_simple_args(&mut self, args_text: &str) -> Result<Vec<Arg>, String> {
        let mut args = Vec::new();

        for arg in args_text.split(',') {
            let arg = arg.trim();
            if arg.starts_with('"') && arg.ends_with('"') {
                args.push(Arg::String(arg[1..arg.len() - 1].to_string()));
            } else if arg.chars().all(|c| c.is_ascii_digit() || c == '.') {
                if let Ok(num) = arg.parse::<f64>() {
                    args.push(Arg::Number(num));
                } else {
                    return Err(format!("Invalid number: {}", arg));
                }
            } else {
                args.push(Arg::Identifier(arg.to_string()));
            }
        }

        Ok(args)
    }

    fn parse_branch_from_string(&mut self, branch_text: &str) -> Result<BranchInterpolation, String> {
        // Expected format: branch<EnumType>(get_value()) { variant1: "text1", variant2: "text2" }
        let branch_text = branch_text.trim();
        
        if !branch_text.starts_with("branch") {
            return Err("Expected 'branch' keyword".to_string());
        }
        
        let rest = &branch_text[6..].trim(); // Skip "branch"
        
        // Parse enum type in angle brackets: <EnumType>
        if !rest.starts_with('<') {
            return Err("Expected '<' after 'branch'".to_string());
        }
        
        let gt_pos = rest.find('>').ok_or("Expected '>' for enum type")?;
        let enum_type = rest[1..gt_pos].trim().to_string();
        let rest = &rest[gt_pos + 1..].trim();
        
        // Parse function call in parentheses: (get_value())
        if !rest.starts_with('(') {
            return Err("Expected '(' after enum type".to_string());
        }
        
        let mut paren_count = 0;
        let mut func_end = 0;
        for (i, ch) in rest.chars().enumerate() {
            if ch == '(' {
                paren_count += 1;
            } else if ch == ')' {
                paren_count -= 1;
                if paren_count == 0 {
                    func_end = i;
                    break;
                }
            }
        }
        
        if func_end == 0 {
            return Err("Expected ')' after function call".to_string());
        }
        
        let func_text = &rest[1..func_end]; // Content inside parentheses
        let enum_value_expr = Box::new(self.parse_expression_from_string(func_text)?);
        let rest = &rest[func_end + 1..].trim();
        
        // Parse branches in curly braces: { variant1: "text1", variant2: "text2" }
        if !rest.starts_with('{') {
            return Err("Expected '{' before branch cases".to_string());
        }
        
        if !rest.ends_with('}') {
            return Err("Expected '}' after branch cases".to_string());
        }
        
        let cases_text = &rest[1..rest.len() - 1].trim();
        let mut branches = Vec::new();
        
        // Split by comma, but be careful with quoted strings
        for case in self.split_branch_cases(cases_text)? {
            let case = case.trim();
            if case.is_empty() {
                continue;
            }
            
            // Parse "variant: \"text\"" or variant: "text"
            let colon_pos = case.find(':').ok_or("Expected ':' in branch case")?;
            let variant = case[..colon_pos].trim().to_string();
            let text_part = case[colon_pos + 1..].trim();
            
            // Handle escaped quotes or regular quotes
            let text = if text_part.starts_with("\\\"") && text_part.ends_with("\\\"") {
                // Escaped quotes: \"text\"
                text_part[2..text_part.len() - 2].to_string()
            } else if text_part.starts_with('"') && text_part.ends_with('"') {
                // Regular quotes: "text"
                text_part[1..text_part.len() - 1].to_string()
            } else {
                return Err(format!("Branch case text must be quoted: {}", text_part));
            };
            
            branches.push(BranchCase { variant, text });
        }
        
        if branches.is_empty() {
            return Err("Branch must have at least one case".to_string());
        }
        
        Ok(BranchInterpolation {
            enum_type,
            enum_value_expr,
            branches,
        })
    }
    
    fn split_branch_cases(&self, cases_text: &str) -> Result<Vec<String>, String> {
        let mut cases = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut escape_next = false;
        
        for ch in cases_text.chars() {
            if escape_next {
                current.push(ch);
                escape_next = false;
                continue;
            }
            
            match ch {
                '\\' => {
                    escape_next = true;
                    current.push(ch);
                }
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    if !current.trim().is_empty() {
                        cases.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.trim().is_empty() {
            cases.push(current.trim().to_string());
        }
        
        Ok(cases)
    }

    fn parse_events_stmt(&mut self) -> Result<Vec<Event>, String> {
        self.consume(&Token::Events, "Expected 'events'")?;
        self.consume(&Token::Colon, "Expected ':'")?;
        self.consume(&Token::LeftBracket, "Expected '['")?;

        let mut events = Vec::new();

        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBracket) && !self.is_at_end() {
                events.push(self.parse_event()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(events)
    }

    fn parse_event(&mut self) -> Result<Event, String> {
        let index = if let Some(token_info) = self.advance() {
            if let Token::Number(n) = &token_info.token {
                n.parse::<f64>().map_err(|_| "Invalid number")?
            } else {
                return Err("Expected number for event index".to_string());
            }
        } else {
            return Err("Expected number for event index".to_string());
        };

        // Skip optional comma or semicolon after event index
        self.skip_optional_separators();

        let action = self.parse_event_action()?;

        Ok(Event { index, action })
    }

    fn parse_event_action(&mut self) -> Result<EventAction, String> {
        let call = self.parse_func_call()?;
        let mut chains = Vec::new();

        while self.check(&Token::Dot) {
            self.advance(); // consume '.'
            chains.push(self.parse_func_call()?);
        }

        Ok(EventAction { call, chains })
    }

    fn parse_choice_stmt(&mut self) -> Result<Vec<ChoiceItem>, String> {
        self.consume(&Token::Choice, "Expected 'choice'")?;
        self.consume(&Token::Colon, "Expected ':'")?;
        self.consume(&Token::LeftBracket, "Expected '['")?;

        let mut items = Vec::new();

        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBracket) && !self.is_at_end() {
                items.push(self.parse_choice_item()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(items)
    }

    fn parse_choice_item(&mut self) -> Result<ChoiceItem, String> {
        // Parse choice text
        let text = if self.check(&Token::LeftParen) {
            self.advance(); // consume '('
            let text = if let Some(token_info) = self.advance() {
                if let Token::String(s) = &token_info.token {
                    s.to_string()
                } else {
                    return Err("Expected string in parentheses".to_string());
                }
            } else {
                return Err("Expected string in parentheses".to_string());
            };
            self.consume(&Token::RightParen, "Expected ')'")?;
            text
        } else if let Some(token_info) = self.advance() {
            if let Token::String(s) = &token_info.token {
                s.to_string()
            } else {
                return Err("Expected choice text".to_string());
            }
        } else {
            return Err("Expected choice text".to_string());
        };

        // Parse optional condition
        let condition = if self.check(&Token::When)
            || (self.check(&Token::Dot)
                && self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::When))
        {
            Some(self.parse_choice_cond()?)
        } else {
            None
        };

        // Parse target
        self.consume(&Token::Arrow, "Expected '->'")?;
        let target = self.parse_choice_dest()?;

        Ok(ChoiceItem {
            text,
            condition,
            target,
        })
    }

    fn parse_choice_cond(&mut self) -> Result<Condition, String> {
        if self.check(&Token::Dot) {
            self.advance(); // consume '.'
            self.consume(&Token::When, "Expected 'when'")?;
            self.consume(&Token::LeftParen, "Expected '('")?;
            let condition = self.parse_condition()?;
            self.consume(&Token::RightParen, "Expected ')'")?;
            Ok(condition)
        } else {
            self.consume(&Token::When, "Expected 'when'")?;
            self.parse_condition()
        }
    }

    fn parse_condition(&mut self) -> Result<Condition, String> {
        if let Some(token_info) = self.peek() {
            if let Token::Identifier(name) = &token_info.token {
                // Look ahead to see if it's a function call
                if self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::LeftParen) {
                    Ok(Condition::FuncCall(self.parse_func_call()?))
                } else {
                    let name = name.to_string();
                    self.advance();
                    Ok(Condition::Identifier(name))
                }
            } else {
                Err("Expected identifier or function call in condition".to_string())
            }
        } else {
            Err("Expected identifier or function call in condition".to_string())
        }
    }

    fn parse_choice_dest(&mut self) -> Result<ChoiceDest, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Identifier(_name)) => {
                let token_info = self.advance().unwrap();
                let name = if let Token::Identifier(name) = &token_info.token {
                    name.to_string()
                } else {
                    unreachable!()
                };
                let span = Some((token_info.start, token_info.end));
                Ok(ChoiceDest::Identifier(name, span))
            }
            Some(Token::Return) => {
                self.advance();
                Ok(ChoiceDest::Return)
            }
            Some(Token::Break) => {
                self.advance();
                Ok(ChoiceDest::Break)
            }
            Some(Token::LeftBracket) => {
                self.advance(); // consume '['

                let mut items = Vec::new();

                while !self.check(&Token::RightBracket) && !self.is_at_end() {
                    self.skip_comments_and_separators();

                    if !self.check(&Token::RightBracket) && !self.is_at_end() {
                        items.push(self.parse_choice_item()?);
                        self.skip_optional_separators();
                    }
                }

                self.consume(&Token::RightBracket, "Expected ']'")?;
                Ok(ChoiceDest::NestedChoices(items))
            }
            _ => Err(format!(
                "Expected choice destination, found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_node_jump(&mut self) -> Result<NodeJump, String> {
        self.consume(&Token::Arrow, "Expected '->'")?;

        match self.peek().map(|t| &t.token) {
            Some(Token::Identifier(_name)) => {
                let token_info = self.advance().unwrap();
                let name = if let Token::Identifier(name) = &token_info.token {
                    name.to_string()
                } else {
                    unreachable!()
                };
                let span = Some((token_info.start, token_info.end));
                Ok(NodeJump::Identifier(name, span))
            }
            Some(Token::Return) => {
                self.advance();
                Ok(NodeJump::Return)
            }
            Some(Token::Break) => {
                self.advance();
                Ok(NodeJump::Break)
            }
            _ => Err(format!(
                "Expected identifier, 'return', or 'break', found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, String> {
        self.consume(&Token::Fn, "Expected 'fn'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected function name".to_string());
            }
        } else {
            return Err("Expected function name".to_string());
        };

        self.consume(&Token::LeftParen, "Expected '('")?;

        let mut params = Vec::new();

        while !self.check(&Token::RightParen) && !self.is_at_end() {
            params.push(self.parse_param()?);
            self.skip_optional_separators();

            if self.check(&Token::RightParen) {
                break;
            }
        }

        self.consume(&Token::RightParen, "Expected ')'")?;

        let return_type = if self.check(&Token::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type()?)
        } else {
            None
        };

        Ok(FunctionDecl {
            name,
            name_span,
            params,
            return_type,
        })
    }

    fn parse_param(&mut self) -> Result<Param, String> {
        let name = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                name.to_string()
            } else {
                return Err("Expected parameter name".to_string());
            }
        } else {
            return Err("Expected parameter name".to_string());
        };

        self.consume(&Token::Colon, "Expected ':'")?;

        let type_name = self.parse_type()?;

        Ok(Param { name, type_name })
    }

    fn parse_type(&mut self) -> Result<String, String> {
        match self.advance().map(|t| &t.token) {
            Some(Token::Identifier(type_name)) => Ok(type_name.to_string()),
            Some(Token::StringType) => Ok("String".to_string()),
            Some(Token::NumberType) => Ok("Number".to_string()),
            Some(Token::BooleanType) => Ok("Boolean".to_string()),
            _ => Err("Expected type".to_string()),
        }
    }

    fn parse_var_decl(&mut self) -> Result<VarDecl, String> {
        self.consume(&Token::Let, "Expected 'let'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected variable name".to_string());
            }
        } else {
            return Err("Expected variable name".to_string());
        };

        self.consume(&Token::Colon, "Expected ':' after variable name")?;

        let type_name = self.parse_type()?;

        let value = if self.check(&Token::Equals) {
            self.advance(); // consume '='
            Some(self.parse_var_value()?)
        } else {
            None
        };

        Ok(VarDecl {
            name,
            name_span,
            type_name,
            value,
        })
    }

    fn parse_const_decl(&mut self) -> Result<ConstDecl, String> {
        let is_public = if self.check(&Token::Pub) {
            self.advance();
            true
        } else {
            false
        };

        self.consume(&Token::Const, "Expected 'const'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected constant name".to_string());
            }
        } else {
            return Err("Expected constant name".to_string());
        };

        self.consume(&Token::Colon, "Expected ':' after constant name")?;

        let type_name = self.parse_type()?;

        self.consume(&Token::Equals, "Expected '=' after constant type")?;

        let value = self.parse_var_value()?;

        Ok(ConstDecl {
            is_public,
            name,
            name_span,
            type_name,
            value,
        })
    }

    fn parse_enum_def(&mut self) -> Result<EnumDef, String> {
        self.consume(&Token::Enum, "Expected 'enum'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected enum name".to_string());
            }
        } else {
            return Err("Expected enum name".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut variants = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if self.check(&Token::RightBrace) {
                break;
            }

            if let Some(token_info) = self.advance() {
                if let Token::Identifier(variant) = &token_info.token {
                    variants.push(variant.to_string());
                } else {
                    return Err("Expected enum variant name".to_string());
                }
            } else {
                return Err("Expected enum variant name".to_string());
            }

            self.skip_optional_separators();
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        Ok(EnumDef {
            name,
            name_span,
            variants,
        })
    }

    fn parse_var_value(&mut self) -> Result<VarValue, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let value = s.to_string();
                self.advance();
                Ok(VarValue::String(value))
            }
            Some(Token::Number(n)) => {
                let value = n.parse::<f64>().map_err(|_| "Invalid number")?;
                self.advance();
                Ok(VarValue::Number(value))
            }
            Some(Token::True) => {
                self.advance();
                Ok(VarValue::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(VarValue::Boolean(false))
            }
            _ => Err(format!(
                "Expected value (string, number, or boolean), found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_func_call(&mut self) -> Result<FuncCall, String> {
        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected function name".to_string());
            }
        } else {
            return Err("Expected function name".to_string());
        };

        self.consume(&Token::LeftParen, "Expected '('")?;

        let mut args = Vec::new();

        while !self.check(&Token::RightParen) && !self.is_at_end() {
            args.push(self.parse_arg()?);
            self.skip_optional_separators();

            if self.check(&Token::RightParen) {
                break;
            }
        }

        self.consume(&Token::RightParen, "Expected ')'")?;

        Ok(FuncCall {
            name,
            name_span,
            args,
        })
    }

    fn parse_arg(&mut self) -> Result<Arg, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let s = s.to_string();
                self.advance();
                Ok(Arg::String(s))
            }
            Some(Token::Number(n)) => {
                let n = n.parse::<f64>().map_err(|_| "Invalid number")?;
                self.advance();
                Ok(Arg::Number(n))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Arg::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(Arg::Boolean(false))
            }
            Some(Token::Identifier(name)) => {
                // Look ahead to see if it's a function call
                if self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::LeftParen) {
                    Ok(Arg::FuncCall(Box::new(self.parse_func_call()?)))
                } else {
                    let name = name.to_string();
                    self.advance();
                    Ok(Arg::Identifier(name))
                }
            }
            _ => Err(format!(
                "Expected argument, found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }
}
