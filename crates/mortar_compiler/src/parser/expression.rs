use super::Parser;
use crate::ast::{
    Arg,
    AssignValue,
    BinaryCondition,
    ComparisonOp,
    FuncCall,
    IfCondition,
    InterpolatedString,
    StringPart,
    UnaryCondition,
    UnaryOp,
    VarValue,
};
use crate::token::Token;

pub trait ExpressionParser {
    fn parse_if_condition(&mut self) -> Result<IfCondition, String>;
    fn parse_or_expression(&mut self) -> Result<IfCondition, String>;
    fn parse_and_expression(&mut self) -> Result<IfCondition, String>;
    fn parse_comparison_expression(&mut self) -> Result<IfCondition, String>;
    fn parse_unary_expression(&mut self) -> Result<IfCondition, String>;
    fn parse_primary_if_condition(&mut self) -> Result<IfCondition, String>;
    fn peek_comparison_op(&self) -> Option<ComparisonOp>;

    fn parse_func_call(&mut self) -> Result<FuncCall, String>;
    fn parse_arg(&mut self) -> Result<Arg, String>;

    fn parse_assign_value(&mut self) -> Result<AssignValue, String>;
    fn parse_var_value(&mut self) -> Result<VarValue, String>;
    fn parse_type(&mut self) -> Result<String, String>;

    fn parse_interpolated_string(&mut self, text: &str) -> Result<InterpolatedString, String>;
    fn parse_expression_from_string(&mut self, expr_text: &str) -> Result<FuncCall, String>;
    fn parse_simple_args(&mut self, args_text: &str) -> Result<Vec<Arg>, String>;
}

impl<'a> ExpressionParser for Parser<'a> {
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

        // Handle identifiers (variables, enum members) and numbers
        if let Some(token_info) = self.peek() {
            match &token_info.token {
                Token::Identifier(name) => {
                    let name = name.to_string();
                    self.advance();
                    // Check for enum member access (EnumName.member)
                    if self.check(&Token::Dot) {
                        self.advance(); // consume '.'
                        let member =
                            self.consume_identifier("Expected enum member name after '.'")?;
                        return Ok(IfCondition::EnumMember(name, member));
                    }
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

    fn parse_assign_value(&mut self) -> Result<AssignValue, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let value = s.to_string();
                self.advance();
                Ok(AssignValue::String(value))
            }
            Some(Token::Number(n)) => {
                let value = n.parse::<f64>().map_err(|_| "Invalid number")?;
                self.advance();
                Ok(AssignValue::Number(value))
            }
            Some(Token::True) => {
                self.advance();
                Ok(AssignValue::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(AssignValue::Boolean(false))
            }
            Some(Token::Identifier(name)) => {
                let first_name = name.to_string();
                self.advance();
                // Check for enum member access (EnumName.member)
                if self.check(&Token::Dot) {
                    self.advance(); // consume '.'
                    let member = self.consume_identifier("Expected enum member name after '.'")?;
                    Ok(AssignValue::EnumMember(first_name, member))
                } else {
                    Ok(AssignValue::Identifier(first_name))
                }
            }
            _ => Err(format!(
                "Expected value (string, number, boolean, identifier, or enum member), found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
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
            Some(Token::Identifier(name)) => {
                let enum_name = name.to_string();
                self.advance();
                // Check for enum member access (EnumName.member)
                if self.check(&Token::Dot) {
                    self.advance(); // consume '.'
                    let member = self.consume_identifier("Expected enum member name after '.'")?;
                    Ok(VarValue::EnumMember(enum_name, member))
                } else {
                    Err(format!(
                        "Unexpected identifier '{}' in variable value",
                        enum_name
                    ))
                }
            }
            _ => Err(format!(
                "Expected value (string, number, boolean, or enum member), found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
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

                    if expr_ch == '\\'
                     {
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
                        brace_count,
                        in_string,
                        expr_text
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
}