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
    EventDef(EventDef),
    TimelineDef(TimelineDef),
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
    Choice(Vec<ChoiceItem>),
    Branch(BranchDef),
    IfElse(IfElseStmt),
    Run(RunStmt),
    WithEvents(WithEventsStmt),
    VarDecl(VarDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseStmt {
    pub condition: IfCondition,
    pub then_body: Vec<NodeStmt>,
    pub else_body: Option<Vec<NodeStmt>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfCondition {
    Binary(Box<BinaryCondition>),
    Unary(Box<UnaryCondition>),
    Identifier(String),
    Literal(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryCondition {
    pub left: IfCondition,
    pub operator: ComparisonOp,
    pub right: IfCondition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryCondition {
    pub operator: UnaryOp,
    pub operand: IfCondition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    Equal,        // ==
    NotEqual,     // !=
    And,          // &&
    Or,           // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, // !
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedString {
    pub parts: Vec<StringPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Expression(FuncCall),
    Placeholder(String), // e.g., {place}, {object}
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchDef {
    pub name: String, // e.g., "place", "object"
    pub name_span: Option<(usize, usize)>,
    pub enum_type: Option<String>, // Some("ExampleEnum") or None for bool branches
    pub cases: Vec<BranchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchCase {
    pub condition: String, // e.g., "is_forest", "tree"
    pub text: String,
    pub events: Option<Vec<Event>>,
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
pub struct EventDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub index: Option<f64>,
    pub action: EventAction,
    pub duration: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub body: Vec<TimelineStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimelineStmt {
    Run(RunStmt),
    Wait(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunStmt {
    pub event_name: String,
    pub event_name_span: Option<(usize, usize)>,
    pub args: Vec<Arg>,
    pub index_override: Option<IndexOverride>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndexOverride {
    Value(f64),
    Variable(String),
    Reference(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithEventsStmt {
    pub events: Vec<WithEventItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WithEventItem {
    EventRef(String, Option<(usize, usize)>),
    InlineEvent(Event),
    EventList(Vec<WithEventItem>),
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
    Branch(BranchValue),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchValue {
    pub enum_type: Option<String>, // Some("EnumType") for enum-based, None for bool-based
    pub cases: Vec<BranchCase>,
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

    fn consume_identifier(&mut self, error_msg: &str) -> Result<String, String> {
        if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                Ok(name.to_string())
            } else {
                Err(format!(
                    "{}: expected identifier, found {:?}",
                    error_msg, token_info.token
                ))
            }
        } else {
            Err(format!("{}: unexpected end of input", error_msg))
        }
    }

    fn consume_string(&mut self, error_msg: &str) -> Result<String, String> {
        if let Some(token_info) = self.advance() {
            if let Token::String(s) = &token_info.token {
                Ok(s.to_string())
            } else {
                Err(format!(
                    "{}: expected string, found {:?}",
                    error_msg, token_info.token
                ))
            }
        } else {
            Err(format!("{}: unexpected end of input", error_msg))
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
            Some(Token::Const) | Some(Token::Pub) => {
                Ok(TopLevel::ConstDecl(self.parse_const_decl()?))
            }
            Some(Token::Enum) => Ok(TopLevel::EnumDef(self.parse_enum_def()?)),
            Some(Token::Event) => Ok(TopLevel::EventDef(self.parse_event_def()?)),
            Some(Token::Timeline) => Ok(TopLevel::TimelineDef(self.parse_timeline_def()?)),
            _ => Err(format!(
                "Expected 'node', 'fn', 'let', 'const', 'pub', 'enum', 'event', or 'timeline', found {:?}",
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
            Some(Token::If) => Ok(NodeStmt::IfElse(self.parse_if_else()?)),
            Some(Token::Text) => Ok(self.parse_text_stmt()?),
            Some(Token::Events) => Err("Standalone 'events:' is deprecated. Use 'with events:' after a text statement instead.".to_string()),
            Some(Token::Choice) => Ok(NodeStmt::Choice(self.parse_choice_stmt()?)),
            Some(Token::Run) => Ok(NodeStmt::Run(self.parse_run_stmt()?)),
            Some(Token::With) => Ok(NodeStmt::WithEvents(self.parse_with_events_stmt()?)),
            Some(Token::Let) => Ok(NodeStmt::VarDecl(self.parse_var_decl()?)),
            Some(Token::Identifier(_)) => {
                // Could be a branch definition (name: branch [...])
                // Peek ahead to see if there's a colon followed by 'branch'
                if self.current + 1 < self.tokens.len() {
                    if matches!(self.tokens[self.current + 1].token, Token::Colon) {
                        if self.current + 2 < self.tokens.len() {
                            if matches!(self.tokens[self.current + 2].token, Token::Branch) {
                                return Ok(NodeStmt::Branch(self.parse_branch_def()?));
                            }
                        }
                    }
                }
                Err(format!(
                    "Unexpected identifier in node body. Expected 'text', 'choice', 'run', 'with', 'let', or branch definition"
                ))
            }
            _ => Err(format!(
                "Expected 'text', 'choice', 'run', 'with', 'let', or branch definition, found {:?}",
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
                let mut in_string = false;
                let mut escape_next = false;

                for expr_ch in chars.by_ref() {
                    if escape_next {
                        expr_text.push(expr_ch);
                        escape_next = false;
                        continue;
                    }

                    if expr_ch == '\\' {
                        expr_text.push(expr_ch);
                        escape_next = true;
                        continue;
                    }

                    if expr_ch == '"' {
                        in_string = !in_string;
                        expr_text.push(expr_ch);
                        continue;
                    }

                    if !in_string {
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
                    } else {
                        expr_text.push(expr_ch);
                    }
                }

                if brace_count != 0 {
                    eprintln!(
                        "DEBUG: brace_count={}, in_string={}, expr_text={:?}",
                        brace_count, in_string, expr_text
                    );
                    return Err("Unmatched '{' in interpolated string".to_string());
                }

                // Check if this is a simple placeholder (identifier) or function call
                let expr_trimmed = expr_text.trim();
                if expr_trimmed.contains('(') {
                    // It's a function call
                    let func_call = self.parse_expression_from_string(&expr_text)?;
                    parts.push(StringPart::Expression(func_call));
                } else {
                    // It's a simple placeholder
                    parts.push(StringPart::Placeholder(expr_trimmed.to_string()));
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

    fn parse_branch_def(&mut self) -> Result<BranchDef, String> {
        // Parse: name: branch [condition, text, ...]
        // or: name: branch<EnumType> [variant, text, ...]

        let name_token = self.consume_identifier("Expected branch name")?;
        let name = name_token.clone();
        let name_span = Some((0, name.len())); // Approximate

        self.consume(&Token::Colon, "Expected ':' after branch name")?;
        self.consume(&Token::Branch, "Expected 'branch' keyword")?;

        // Check for optional enum type: <EnumType>
        let enum_type = if self.check(&Token::Less) {
            self.advance(); // consume <
            let type_token = self.consume_identifier("Expected enum type name")?;
            self.consume(&Token::Greater, "Expected '>' after enum type")?;
            Some(type_token)
        } else {
            None
        };

        // Parse cases in brackets: [condition, text, ...]
        self.consume(&Token::LeftBracket, "Expected '[' to start branch cases")?;

        let mut cases = Vec::new();

        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            let case = self.parse_branch_case()?;
            cases.push(case);

            // Cases can be separated by newlines or commas (optional)
            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']' to end branch cases")?;

        Ok(BranchDef {
            name,
            name_span,
            enum_type,
            cases,
        })
    }

    fn parse_branch_case(&mut self) -> Result<BranchCase, String> {
        // Parse: condition, text
        // or: condition, text, events: [...]

        let condition = self.consume_identifier("Expected condition or variant")?;
        self.consume(&Token::Comma, "Expected ',' after condition")?;

        let text = self.consume_string("Expected text for branch case")?;

        // Check for optional events
        let events = if self.check(&Token::Comma) {
            self.advance(); // consume comma

            if self.check(&Token::Events) {
                self.advance(); // consume 'events'
                self.consume(&Token::Colon, "Expected ':' after 'events'")?;
                Some(self.parse_event_list()?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(BranchCase {
            condition,
            text,
            events,
        })
    }

    fn parse_event_list(&mut self) -> Result<Vec<Event>, String> {
        self.consume(&Token::LeftBracket, "Expected '[' to start events")?;

        let mut events = Vec::new();

        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            let event = self.parse_event()?;
            events.push(event);

            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']' to end events")?;

        Ok(events)
    }

    fn parse_if_else(&mut self) -> Result<IfElseStmt, String> {
        self.consume(&Token::If, "Expected 'if'")?;

        // Parse condition
        let condition = self.parse_if_condition()?;

        // Parse then body
        self.consume(&Token::LeftBrace, "Expected '{' after if condition")?;

        let mut then_body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();
            if self.check(&Token::RightBrace) {
                break;
            }
            then_body.push(self.parse_node_stmt()?);
        }

        self.consume(&Token::RightBrace, "Expected '}' to end if body")?;

        // Parse optional else body
        let else_body = if self.check(&Token::Else) {
            self.advance(); // consume 'else'
            self.consume(&Token::LeftBrace, "Expected '{' after else")?;

            let mut body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                self.skip_comments_and_separators();
                if self.check(&Token::RightBrace) {
                    break;
                }
                body.push(self.parse_node_stmt()?);
            }

            self.consume(&Token::RightBrace, "Expected '}' to end else body")?;
            Some(body)
        } else {
            None
        };

        Ok(IfElseStmt {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_if_condition(&mut self) -> Result<IfCondition, String> {
        self.parse_or_expression()
    }

    fn parse_or_expression(&mut self) -> Result<IfCondition, String> {
        let mut left = self.parse_and_expression()?;

        while self.check(&Token::Or) {
            self.advance();
            let right = self.parse_and_expression()?;
            left = IfCondition::Binary(Box::new(BinaryCondition {
                left,
                operator: ComparisonOp::Or,
                right,
            }));
        }

        Ok(left)
    }

    fn parse_and_expression(&mut self) -> Result<IfCondition, String> {
        let mut left = self.parse_comparison_expression()?;

        while self.check(&Token::And) {
            self.advance();
            let right = self.parse_comparison_expression()?;
            left = IfCondition::Binary(Box::new(BinaryCondition {
                left,
                operator: ComparisonOp::And,
                right,
            }));
        }

        Ok(left)
    }

    fn parse_comparison_expression(&mut self) -> Result<IfCondition, String> {
        let mut left = self.parse_unary_expression()?;

        while let Some(op) = self.peek_comparison_op() {
            self.advance(); // consume operator
            let right = self.parse_unary_expression()?;
            left = IfCondition::Binary(Box::new(BinaryCondition {
                left,
                operator: op,
                right,
            }));
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<IfCondition, String> {
        if self.check(&Token::Not) {
            self.advance();
            let operand = self.parse_unary_expression()?;
            return Ok(IfCondition::Unary(Box::new(UnaryCondition {
                operator: UnaryOp::Not,
                operand,
            })));
        }

        self.parse_primary_if_condition()
    }

    fn parse_primary_if_condition(&mut self) -> Result<IfCondition, String> {
        // Handle parenthesized expressions
        if self.check(&Token::LeftParen) {
            self.advance();
            let cond = self.parse_if_condition()?;
            self.consume(&Token::RightParen, "Expected ')' after condition")?;
            return Ok(cond);
        }

        // Handle boolean literals
        if self.check(&Token::True) {
            self.advance();
            return Ok(IfCondition::Literal(true));
        }

        if self.check(&Token::False) {
            self.advance();
            return Ok(IfCondition::Literal(false));
        }

        // Handle identifiers (variables) and numbers
        if let Some(token_info) = self.peek() {
            match &token_info.token {
                Token::Identifier(name) => {
                    let name = name.to_string();
                    self.advance();
                    return Ok(IfCondition::Identifier(name));
                }
                Token::Number(num) => {
                    let num = num.to_string();
                    self.advance();
                    return Ok(IfCondition::Identifier(num));
                }
                _ => {}
            }
        }

        Err("Expected condition expression".to_string())
    }

    fn peek_comparison_op(&self) -> Option<ComparisonOp> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Greater) => Some(ComparisonOp::Greater),
            Some(Token::Less) => Some(ComparisonOp::Less),
            Some(Token::GreaterEqual) => Some(ComparisonOp::GreaterEqual),
            Some(Token::LessEqual) => Some(ComparisonOp::LessEqual),
            Some(Token::EqualEqual) => Some(ComparisonOp::Equal),
            Some(Token::NotEqual) => Some(ComparisonOp::NotEqual),
            _ => None,
        }
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

        // Check if this is a branch type variable
        if self.check(&Token::Branch) {
            self.advance(); // consume 'branch'

            // Check for optional enum type: branch<EnumType>
            let enum_type = if self.check(&Token::Less) {
                self.advance(); // consume <
                let type_token = self.consume_identifier("Expected enum type or variable name")?;
                self.consume(&Token::Greater, "Expected '>' after type")?;
                Some(type_token)
            } else {
                None
            };

            // Parse branch cases in brackets: [condition, text, ...]
            self.consume(&Token::LeftBracket, "Expected '[' to start branch cases")?;

            let mut cases = Vec::new();

            while !self.check(&Token::RightBracket) && !self.is_at_end() {
                let case = self.parse_branch_case()?;
                cases.push(case);

                // Cases can be separated by newlines or commas (optional)
                if self.check(&Token::Comma) {
                    self.advance();
                }
            }

            self.consume(&Token::RightBracket, "Expected ']' to end branch cases")?;

            Ok(VarDecl {
                name,
                name_span,
                type_name: "branch".to_string(),
                value: Some(VarValue::Branch(BranchValue { enum_type, cases })),
            })
        } else {
            // Regular variable declaration
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

    fn parse_event_def(&mut self) -> Result<EventDef, String> {
        self.consume(&Token::Event, "Expected 'event'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected identifier after 'event'".to_string());
            }
        } else {
            return Err("Expected identifier after 'event'".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut index = None;
        let mut action = None;
        let mut duration = None;

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if self.check(&Token::RightBrace) {
                break;
            }

            match self.peek().map(|t| &t.token) {
                Some(Token::Index) => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'index'")?;
                    if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                        index = Some(n.parse::<f64>().map_err(|_| "Invalid number")?);
                        self.advance();
                    } else {
                        return Err("Expected number after 'index:'".to_string());
                    }
                }
                Some(Token::Action) => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'action'")?;
                    action = Some(self.parse_event_action()?);
                }
                Some(Token::Duration) => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'duration'")?;
                    if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                        duration = Some(n.parse::<f64>().map_err(|_| "Invalid number")?);
                        self.advance();
                    } else {
                        return Err("Expected number after 'duration:'".to_string());
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected 'index', 'action', or 'duration' in event definition, found {:?}",
                        self.peek().map(|t| &t.token)
                    ));
                }
            }

            self.skip_optional_separators();
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        let action = action.ok_or("Event definition must have an 'action' field")?;

        Ok(EventDef {
            name,
            name_span,
            index,
            action,
            duration,
        })
    }

    fn parse_timeline_def(&mut self) -> Result<TimelineDef, String> {
        self.consume(&Token::Timeline, "Expected 'timeline'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected identifier after 'timeline'".to_string());
            }
        } else {
            return Err("Expected identifier after 'timeline'".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBrace) && !self.is_at_end() {
                body.push(self.parse_timeline_stmt()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        Ok(TimelineDef {
            name,
            name_span,
            body,
        })
    }

    fn parse_timeline_stmt(&mut self) -> Result<TimelineStmt, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Run) => Ok(TimelineStmt::Run(self.parse_run_stmt()?)),
            Some(Token::Wait) => {
                self.advance();
                if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                    let duration = n.parse::<f64>().map_err(|_| "Invalid number")?;
                    self.advance();
                    Ok(TimelineStmt::Wait(duration))
                } else {
                    Err("Expected number after 'wait'".to_string())
                }
            }
            _ => Err(format!(
                "Expected 'run' or 'wait' in timeline, found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_run_stmt(&mut self) -> Result<RunStmt, String> {
        self.consume(&Token::Run, "Expected 'run'")?;

        let (event_name, event_name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected identifier after 'run'".to_string());
            }
        } else {
            return Err("Expected identifier after 'run'".to_string());
        };

        let mut args = Vec::new();

        // Check if there are arguments
        if self.check(&Token::LeftParen) {
            self.advance();

            while !self.check(&Token::RightParen) && !self.is_at_end() {
                args.push(self.parse_arg()?);
                self.skip_optional_separators();

                if self.check(&Token::RightParen) {
                    break;
                }
            }

            self.consume(&Token::RightParen, "Expected ')'")?;
        }

        // Check for 'with' clause
        let index_override = if self.check(&Token::With) {
            self.advance();

            if self.check(&Token::Ref) {
                self.advance();
                let var_name = self.consume_identifier("Expected variable name after 'ref'")?;
                Some(IndexOverride::Reference(var_name))
            } else if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                let value = n.parse::<f64>().map_err(|_| "Invalid number")?;
                self.advance();
                Some(IndexOverride::Value(value))
            } else if let Some(Token::Identifier(name)) = self.peek().map(|t| &t.token) {
                let name = name.to_string();
                self.advance();
                Some(IndexOverride::Variable(name))
            } else {
                return Err("Expected number, identifier, or 'ref' after 'with'".to_string());
            }
        } else {
            None
        };

        Ok(RunStmt {
            event_name,
            event_name_span,
            args,
            index_override,
        })
    }

    fn parse_with_events_stmt(&mut self) -> Result<WithEventsStmt, String> {
        self.consume(&Token::With, "Expected 'with'")?;

        let mut events = Vec::new();

        // Check if it's:
        // - "with event { index, action }" (single inline event) - check FIRST
        // - "with events: [...]" (multiple events)
        // - "with EventName" (single event reference)
        if self.check(&Token::Event) {
            // "with event { index, action }" - single inline event
            self.advance();
            self.consume(&Token::LeftBrace, "Expected '{' after 'event'")?;
            let event = self.parse_event()?;
            events.push(WithEventItem::InlineEvent(event));
            self.consume(&Token::RightBrace, "Expected '}' after event")?;
        } else if self.check(&Token::Events) {
            self.advance();
            self.consume(&Token::Colon, "Expected ':' after 'events'")?;
            self.consume(&Token::LeftBracket, "Expected '['")?;

            while !self.check(&Token::RightBracket) && !self.is_at_end() {
                self.skip_comments_and_separators();

                if self.check(&Token::RightBracket) {
                    break;
                }

                // Check if this is an inline event (starts with a number) or an event reference (identifier)
                if let Some(Token::Number(_)) = self.peek().map(|t| &t.token) {
                    // Inline event: index, action
                    let event = self.parse_event()?;
                    events.push(WithEventItem::InlineEvent(event));
                } else if let Some(Token::Identifier(name)) = self.peek().map(|t| &t.token) {
                    let name = name.to_string();
                    let span = self.peek().map(|t| (t.start, t.end));
                    self.advance();
                    events.push(WithEventItem::EventRef(name, span));
                } else {
                    return Err(
                        "Expected event index or event name in 'with events' list".to_string()
                    );
                }

                self.skip_optional_separators();
            }

            self.consume(&Token::RightBracket, "Expected ']'")?;
        } else if let Some(Token::Identifier(name)) = self.peek().map(|t| &t.token) {
            let name = name.to_string();
            let span = self.peek().map(|t| (t.start, t.end));
            self.advance();
            events.push(WithEventItem::EventRef(name, span));
        } else {
            return Err("Expected 'events', 'event', or event name after 'with'".to_string());
        }

        Ok(WithEventsStmt { events })
    }

    fn parse_func_call(&mut self) -> Result<FuncCall, String> {
        let (name, name_span) = if let Some(token_info) = self.advance() {
            match &token_info.token {
                Token::Identifier(name) => {
                    (name.to_string(), Some((token_info.start, token_info.end)))
                }
                // Allow certain keywords as function names for backwards compatibility
                Token::Wait => ("wait".to_string(), Some((token_info.start, token_info.end))),
                Token::Run => ("run".to_string(), Some((token_info.start, token_info.end))),
                Token::Action => (
                    "action".to_string(),
                    Some((token_info.start, token_info.end)),
                ),
                Token::Index => (
                    "index".to_string(),
                    Some((token_info.start, token_info.end)),
                ),
                Token::Duration => (
                    "duration".to_string(),
                    Some((token_info.start, token_info.end)),
                ),
                _ => {
                    return Err("Expected function name".to_string());
                }
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
