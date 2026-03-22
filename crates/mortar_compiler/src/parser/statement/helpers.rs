use super::{ParseError, Parser, StatementParser};
use crate::ast::{ChoiceDest, NodeStmt, WithEventItem};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub(super) fn parse_identifier_node_stmt(&mut self) -> Result<NodeStmt, ParseError> {
        if self
            .tokens
            .get(self.current + 1)
            .is_some_and(|t| t.token == Token::Equals)
        {
            return Ok(NodeStmt::Assignment(self.parse_assignment()?));
        }
        let is_branch = self
            .tokens
            .get(self.current + 1)
            .is_some_and(|t| t.token == Token::Colon)
            && self
                .tokens
                .get(self.current + 2)
                .is_some_and(|t| matches!(t.token, Token::Branch));
        if is_branch {
            return Ok(NodeStmt::Branch(self.parse_branch_def()?));
        }
        Err(ParseError::Custom(
            "Unexpected identifier in node body. Expected 'text', 'choice', 'run', 'with', assignment, or branch definition".to_string(),
        ))
    }

    pub(super) fn parse_nested_choice_dest(&mut self) -> Result<ChoiceDest, ParseError> {
        self.advance();
        let mut items = Vec::new();
        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            self.skip_comments_and_separators();
            if self.check(&Token::RightBracket) || self.is_at_end() {
                continue;
            }
            items.push(self.parse_choice_item()?);
            self.skip_optional_separators();
        }
        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(ChoiceDest::NestedChoices(items))
    }

    pub(super) fn parse_node_stmts_in_braces(
        &mut self,
        open_msg: &str,
        close_msg: &str,
    ) -> Result<Vec<NodeStmt>, ParseError> {
        self.consume(&Token::LeftBrace, open_msg)?;
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();
            if self.check(&Token::RightBrace) || self.is_at_end() {
                break;
            }
            body.push(self.parse_node_stmt()?);
        }
        self.consume(&Token::RightBrace, close_msg)?;
        Ok(body)
    }

    pub(super) fn parse_with_events_list(&mut self) -> Result<Vec<WithEventItem>, ParseError> {
        self.advance();
        self.consume(&Token::Colon, "Expected ':' after 'events'")?;
        self.consume(&Token::LeftBracket, "Expected '['")?;

        let mut events = Vec::new();
        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            self.skip_comments_and_separators();
            if self.check(&Token::RightBracket) || self.is_at_end() {
                break;
            }
            events.push(self.parse_with_events_item()?);
            self.skip_optional_separators();
        }
        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(events)
    }

    fn parse_with_events_item(&mut self) -> Result<WithEventItem, ParseError> {
        if let Some(Token::Number(_)) = self.peek().map(|t| &t.token) {
            let event = self.parse_event()?;
            Ok(WithEventItem::InlineEvent(event))
        } else if let Some(Token::Identifier(name)) = self.peek().map(|t| &t.token) {
            let name = name.to_string();
            let span = self.peek().map(|t| (t.start, t.end));
            self.advance();
            Ok(WithEventItem::EventRef(name, span))
        } else {
            Err(ParseError::Custom(
                "Expected event index or event name in 'with events' list".to_string(),
            ))
        }
    }
}
