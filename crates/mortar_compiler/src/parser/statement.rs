use super::Parser;
use crate::ast::{
    Assignment, BranchCase, BranchDef, ChoiceDest, ChoiceItem, Condition, Event, EventAction,
    IfElseStmt, IndexOverride, NodeStmt, RunStmt, WithEventItem, WithEventsStmt,
};
use crate::parser::expression::ExpressionParser;
use crate::token::Token;

pub trait StatementParser {
    fn parse_node_stmt(&mut self) -> Result<NodeStmt, String>;
    fn parse_text_stmt(&mut self) -> Result<NodeStmt, String>;
    fn parse_choice_stmt(&mut self) -> Result<Vec<ChoiceItem>, String>;
    fn parse_choice_item(&mut self) -> Result<ChoiceItem, String>;
    fn parse_choice_cond(&mut self) -> Result<Condition, String>;
    fn parse_choice_dest(&mut self) -> Result<ChoiceDest, String>;
    fn parse_condition(&mut self) -> Result<Condition, String>;

    fn parse_branch_def(&mut self) -> Result<BranchDef, String>;
    fn parse_branch_case(&mut self) -> Result<BranchCase, String>;

    fn parse_event_list(&mut self) -> Result<Vec<Event>, String>;
    fn parse_event(&mut self) -> Result<Event, String>;
    fn parse_event_action(&mut self) -> Result<EventAction, String>;

    fn parse_if_else(&mut self) -> Result<IfElseStmt, String>;

    fn parse_run_stmt(&mut self) -> Result<RunStmt, String>;
    fn parse_with_events_stmt(&mut self) -> Result<WithEventsStmt, String>;

    fn parse_assignment(&mut self) -> Result<Assignment, String>;
}

impl<'a> StatementParser for Parser<'a> {
    fn parse_node_stmt(&mut self) -> Result<NodeStmt, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::If) => Ok(NodeStmt::IfElse(self.parse_if_else()?)),
            Some(Token::Text) => Ok(self.parse_text_stmt()?),
            Some(Token::Events) => Err("Standalone 'events:' is deprecated. Use 'with events:' after a text statement instead.".to_string()),
            Some(Token::Choice) => Ok(NodeStmt::Choice(self.parse_choice_stmt()?)),
            Some(Token::Run) => Ok(NodeStmt::Run(self.parse_run_stmt()?)),
            Some(Token::With) => Ok(NodeStmt::WithEvents(self.parse_with_events_stmt()?)),
            Some(Token::Let) => Err("Variable declarations with 'let' are not allowed inside nodes. Please define variables at the top level (outside of nodes).".to_string()),
            Some(Token::Identifier(_)) => {
                // Could be:
                // 1. Assignment (name = value)
                // 2. Branch definition (name: branch [...])
                if self.current + 1 < self.tokens.len() {
                    match &self.tokens[self.current + 1].token {
                        Token::Equals => {
                            return Ok(NodeStmt::Assignment(self.parse_assignment()?));
                        }
                        Token::Colon => {
                            if self.current + 2 < self.tokens.len()
                                && matches!(self.tokens[self.current + 2].token, Token::Branch) {
                                    return Ok(NodeStmt::Branch(self.parse_branch_def()?));
                                }

                        }
                        _ => {}
                    }
                }
                Err("Unexpected identifier in node body. Expected 'text', 'choice', 'run', 'with', assignment, or branch definition".to_string())
            }
            _ => Err(format!(
                "Expected 'text', 'choice', 'run', 'with', assignment, or branch definition, found {:?}",
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

            if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                let value = n.parse::<f64>().map_err(|_| "Invalid number")?;
                self.advance();
                Some(IndexOverride::Value(value))
            } else if let Some(Token::Identifier(name)) = self.peek().map(|t| &t.token) {
                let name = name.to_string();
                self.advance();
                Some(IndexOverride::Variable(name))
            } else {
                return Err("Expected number or identifier after 'with'".to_string());
            }
        } else {
            None
        };

        Ok(RunStmt {
            event_name,
            event_name_span,
            args,
            index_override,
            ignore_duration: false,
        })
    }

    fn parse_with_events_stmt(&mut self) -> Result<WithEventsStmt, String> {
        self.consume(&Token::With, "Expected 'with'")?;

        let mut events = Vec::new();

        // Check if it's:
        // - "with run EventName with var" (run statement with index override converted to text event)
        // - "with event { index, action }" (single inline event) - check FIRST
        // - "with events: [...]" (multiple events)
        // - "with EventName" (single event reference)
        if self.check(&Token::Run) {
            // "with run EventName with var" - parse as run statement but treat as text event
            // Parse the run statement
            let run_stmt = self.parse_run_stmt()?;

            // Store as event reference with or without index override
            let name = run_stmt.event_name.clone();
            let span = run_stmt.event_name_span;

            if let Some(override_val) = run_stmt.index_override {
                events.push(WithEventItem::EventRefWithOverride(
                    name,
                    span,
                    override_val,
                ));
            } else {
                events.push(WithEventItem::EventRef(name, span));
            }
        } else if self.check(&Token::Event) {
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

    fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let (var_name, var_name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected variable name".to_string());
            }
        } else {
            return Err("Expected variable name".to_string());
        };

        self.consume(&Token::Equals, "Expected '=' after variable name")?;

        let value = self.parse_assign_value()?;

        Ok(Assignment {
            var_name,
            var_name_span,
            value,
        })
    }
}
