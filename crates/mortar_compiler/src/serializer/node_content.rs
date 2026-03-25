//! # node_content.rs
//!
//! # node_content.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Serializes node-local Mortar content into the flattened `.mortared` representation.
//! It is where text, lines, runs, choices, `if` branches, local statements, and `with events`
//! blocks are merged into the final node content payload.
//!
//! 负责把节点局部的 Mortar 内容序列化成扁平化的 `.mortared` 表示。文本、line、
//! run、choice、`if` 分支、本地语句以及 `with events` 块，都会在这里被合并进最终节点内容载荷。

use crate::ast::{
    Arg, BranchDef, ChoiceDest, ChoiceItem, Condition, Event, EventDef, IfElseStmt, IndexOverride,
    NodeDef, NodeJump, NodeStmt, WithEventItem, WithEventsStmt,
};
use crate::serializer_types::{
    ContentItem, JsonBranchCase, JsonBranchDef, JsonChoice, JsonCondition, JsonEvent,
    JsonIfCondition, JsonIndexOverride, JsonNode, JsonStatement,
};

use super::Serializer;

impl Serializer {
    fn peek_with_events(
        body_iter: &mut std::iter::Peekable<std::slice::Iter<'_, NodeStmt>>,
        event_map: &std::collections::HashMap<String, &EventDef>,
    ) -> Result<Option<Vec<JsonEvent>>, String> {
        let Some(NodeStmt::WithEvents(with_events)) = body_iter.peek() else {
            return Ok(None);
        };
        let mut events = Vec::new();
        Self::process_with_events(with_events, &mut events, event_map)?;
        body_iter.next();
        Ok(if events.is_empty() {
            None
        } else {
            Some(events)
        })
    }

    fn combine_conditions(
        outer: &Option<JsonIfCondition>,
        inner: Option<JsonIfCondition>,
    ) -> Option<JsonIfCondition> {
        match (outer, inner) {
            (Some(current_cond), Some(inner_cond)) => Some(JsonIfCondition {
                cond_type: "binary".to_string(),
                operator: Some("&&".to_string()),
                left: Some(Box::new(current_cond.clone())),
                right: Some(Box::new(inner_cond)),
                operand: None,
                value: None,
            }),
            (Some(current_cond), None) => Some(current_cond.clone()),
            (None, inner_cond) => inner_cond,
        }
    }

    fn merge_nested_content(
        nested_content: Vec<ContentItem>,
        condition: &Option<JsonIfCondition>,
        content: &mut Vec<ContentItem>,
    ) {
        for item in nested_content {
            if let ContentItem::Text {
                value,
                interpolated_parts,
                condition: nested_cond,
                pre_statements,
                events,
            } = item
            {
                let combined_cond = Self::combine_conditions(condition, nested_cond);
                content.push(ContentItem::Text {
                    value,
                    interpolated_parts,
                    condition: combined_cond,
                    pre_statements,
                    events,
                });
            } else {
                content.push(item);
            }
        }
    }

    pub(super) fn convert_node_def(
        node_def: &NodeDef,
        event_map: &std::collections::HashMap<String, &EventDef>,
    ) -> Result<JsonNode, String> {
        let mut content = Vec::new();
        let mut branches_vec = Vec::new();
        let mut local_variables = Vec::new();
        let mut pending_statements: Vec<JsonStatement> = Vec::new();

        let mut body_iter = node_def.body.iter().peekable();

        while let Some(stmt) = body_iter.next() {
            match stmt {
                NodeStmt::Text(text) => {
                    let events = Self::peek_with_events(&mut body_iter, event_map)?;
                    content.push(ContentItem::Text {
                        value: text.clone(),
                        interpolated_parts: None,
                        condition: None,
                        pre_statements: std::mem::take(&mut pending_statements),
                        events,
                    });
                }
                NodeStmt::InterpolatedText(interpolated) => {
                    let (rendered_text, parts) = Self::convert_interpolated_string(interpolated)?;
                    let events = Self::peek_with_events(&mut body_iter, event_map)?;
                    content.push(ContentItem::Text {
                        value: rendered_text,
                        interpolated_parts: Some(parts),
                        condition: None,
                        pre_statements: std::mem::take(&mut pending_statements),
                        events,
                    });
                }
                NodeStmt::Line(text) => {
                    let events = Self::peek_with_events(&mut body_iter, event_map)?;
                    content.push(ContentItem::Line {
                        value: text.clone(),
                        interpolated_parts: None,
                        condition: None,
                        pre_statements: std::mem::take(&mut pending_statements),
                        events,
                    });
                }
                NodeStmt::InterpolatedLine(interpolated) => {
                    let (rendered_text, parts) = Self::convert_interpolated_string(interpolated)?;
                    let events = Self::peek_with_events(&mut body_iter, event_map)?;
                    content.push(ContentItem::Line {
                        value: rendered_text,
                        interpolated_parts: Some(parts),
                        condition: None,
                        pre_statements: std::mem::take(&mut pending_statements),
                        events,
                    });
                }
                NodeStmt::Run(run_stmt) => {
                    let args: Vec<String> = run_stmt
                        .args
                        .iter()
                        .map(Self::convert_arg_to_string)
                        .collect();

                    let index_override =
                        run_stmt
                            .index_override
                            .as_ref()
                            .map(|override_val| match override_val {
                                IndexOverride::Value(v) => JsonIndexOverride {
                                    override_type: "value".to_string(),
                                    value: v.to_string(),
                                },
                                IndexOverride::Variable(var) => JsonIndexOverride {
                                    override_type: "variable".to_string(),
                                    value: var.clone(),
                                },
                            });

                    content.push(ContentItem::RunEvent {
                        name: run_stmt.event_name.clone(),
                        args,
                        index_override,
                        ignore_duration: run_stmt.ignore_duration,
                    });
                }
                NodeStmt::Choice(choice_items) => {
                    let json_choices = choice_items
                        .iter()
                        .map(Self::convert_choice_item)
                        .collect::<Result<Vec<_>, String>>()?;
                    content.push(ContentItem::Choice {
                        options: json_choices,
                    });
                }
                NodeStmt::IfElse(if_else) => {
                    Self::process_if_else_to_content(if_else, &mut content)?;
                }
                NodeStmt::Branch(branch_def) => {
                    branches_vec.push(Self::convert_branch_def(branch_def)?);
                }
                NodeStmt::VarDecl(var_decl) => {
                    local_variables.push(Self::convert_var_decl(var_decl));
                }
                NodeStmt::Assignment(assignment) => {
                    let value_str = Self::convert_assign_value_to_string(&assignment.value);
                    pending_statements.push(JsonStatement {
                        stmt_type: "assignment".to_string(),
                        var_name: Some(assignment.var_name.clone()),
                        value: Some(value_str),
                    });
                }
                NodeStmt::WithEvents(_) => {
                    return Err("`with events:` block must follow a `text:` statement.".to_string());
                }
            }
        }

        let next = match &node_def.jump {
            Some(NodeJump::Identifier(name, _)) => Some(name.clone()),
            _ => None,
        };

        Ok(JsonNode {
            name: node_def.name.clone(),
            content,
            branches: if branches_vec.is_empty() {
                None
            } else {
                Some(branches_vec)
            },
            variables: local_variables,
            next,
        })
    }

    fn convert_branch_def(branch_def: &BranchDef) -> Result<JsonBranchDef, String> {
        let cases = branch_def
            .cases
            .iter()
            .map(|case| {
                let events = if let Some(event_list) = &case.events {
                    Some(
                        event_list
                            .iter()
                            .map(Self::convert_event)
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                } else {
                    None
                };

                Ok(JsonBranchCase {
                    condition: case.condition.clone(),
                    text: case.text.clone(),
                    events,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(JsonBranchDef {
            name: branch_def.name.clone(),
            enum_type: branch_def.enum_type.clone(),
            cases,
        })
    }

    fn process_with_events(
        with_events: &WithEventsStmt,
        events: &mut Vec<JsonEvent>,
        event_map: &std::collections::HashMap<String, &EventDef>,
    ) -> Result<(), String> {
        for item in &with_events.events {
            Self::process_with_event_item(item, events, event_map)?;
        }
        Ok(())
    }

    fn process_with_event_item(
        item: &WithEventItem,
        events: &mut Vec<JsonEvent>,
        event_map: &std::collections::HashMap<String, &EventDef>,
    ) -> Result<(), String> {
        match item {
            WithEventItem::InlineEvent(event) => {
                events.push(Self::convert_event(event)?);
            }
            WithEventItem::EventRef(name, _span) => {
                let event_def = event_map
                    .get(name)
                    .ok_or_else(|| format!("Event '{}' not found", name))?;
                let event = Event {
                    index: event_def.index.unwrap_or(0.0),
                    action: event_def.action.clone(),
                };
                events.push(Self::convert_event(&event)?);
            }
            WithEventItem::EventRefWithOverride(name, _span, override_val) => {
                let event_def = event_map
                    .get(name)
                    .ok_or_else(|| format!("Event '{}' not found", name))?;
                let (index, index_variable) = match override_val {
                    IndexOverride::Value(v) => (*v, None),
                    IndexOverride::Variable(var_name) => (0.0, Some(var_name.clone())),
                };

                let mut actions = vec![Self::convert_func_call_to_action(&event_def.action.call)?];
                for chain_call in &event_def.action.chains {
                    actions.push(Self::convert_func_call_to_action(chain_call)?);
                }

                events.push(JsonEvent {
                    index,
                    index_variable,
                    actions,
                });
            }
            WithEventItem::EventList(_) => {}
        }
        Ok(())
    }

    fn process_if_else_to_content(
        if_else: &IfElseStmt,
        content: &mut Vec<ContentItem>,
    ) -> Result<(), String> {
        let condition_json = Self::convert_if_condition(&if_else.condition)?;

        Self::process_conditional_body_to_content(
            &if_else.then_body,
            Some(condition_json.clone()),
            content,
        )?;

        if let Some(else_body) = &if_else.else_body {
            let negated_condition = JsonIfCondition {
                cond_type: "unary".to_string(),
                operator: Some("!".to_string()),
                left: None,
                right: None,
                operand: Some(Box::new(condition_json)),
                value: None,
            };
            Self::process_conditional_body_to_content(else_body, Some(negated_condition), content)?;
        }

        Ok(())
    }

    fn process_conditional_body_to_content(
        body: &[NodeStmt],
        condition: Option<JsonIfCondition>,
        content: &mut Vec<ContentItem>,
    ) -> Result<(), String> {
        let mut pending_stmts = Vec::new();
        let mut has_text_in_block = false;

        for stmt in body {
            match stmt {
                NodeStmt::Text(text) => {
                    has_text_in_block = true;
                    content.push(ContentItem::Text {
                        value: text.clone(),
                        interpolated_parts: None,
                        events: None,
                        condition: condition.clone(),
                        pre_statements: std::mem::take(&mut pending_stmts),
                    });
                }
                NodeStmt::InterpolatedText(interp) => {
                    has_text_in_block = true;
                    let (rendered, parts) = Self::convert_interpolated_string(interp)?;
                    content.push(ContentItem::Text {
                        value: rendered,
                        interpolated_parts: Some(parts),
                        events: None,
                        condition: condition.clone(),
                        pre_statements: std::mem::take(&mut pending_stmts),
                    });
                }
                NodeStmt::Line(text) => {
                    has_text_in_block = true;
                    content.push(ContentItem::Line {
                        value: text.clone(),
                        interpolated_parts: None,
                        events: None,
                        condition: condition.clone(),
                        pre_statements: std::mem::take(&mut pending_stmts),
                    });
                }
                NodeStmt::InterpolatedLine(interp) => {
                    has_text_in_block = true;
                    let (rendered, parts) = Self::convert_interpolated_string(interp)?;
                    content.push(ContentItem::Line {
                        value: rendered,
                        interpolated_parts: Some(parts),
                        events: None,
                        condition: condition.clone(),
                        pre_statements: std::mem::take(&mut pending_stmts),
                    });
                }
                NodeStmt::Assignment(assignment) => {
                    let value_str = Self::convert_assign_value_to_string(&assignment.value);
                    pending_stmts.push(JsonStatement {
                        stmt_type: "assignment".to_string(),
                        var_name: Some(assignment.var_name.clone()),
                        value: Some(value_str),
                    });
                }
                NodeStmt::IfElse(nested_if) => {
                    let mut nested_content = Vec::new();
                    Self::process_if_else_to_content(nested_if, &mut nested_content)?;
                    Self::merge_nested_content(nested_content, &condition, content);
                }
                _ => {}
            }
        }

        if !pending_stmts.is_empty() && !has_text_in_block {
            content.push(ContentItem::Text {
                value: String::new(),
                interpolated_parts: None,
                events: None,
                condition: condition.clone(),
                pre_statements: pending_stmts,
            });
        }
        Ok(())
    }

    pub(super) fn convert_event(event: &Event) -> Result<JsonEvent, String> {
        let mut actions = vec![Self::convert_func_call_to_action(&event.action.call)?];

        for chain_call in &event.action.chains {
            actions.push(Self::convert_func_call_to_action(chain_call)?);
        }

        Ok(JsonEvent {
            index: event.index,
            index_variable: None,
            actions,
        })
    }

    fn convert_choice_item(choice_item: &ChoiceItem) -> Result<JsonChoice, String> {
        let condition = match &choice_item.condition {
            Some(Condition::Identifier(id)) => Some(JsonCondition {
                condition_type: id.clone(),
                args: Vec::new(),
            }),
            Some(Condition::FuncCall(func_call)) => Some(JsonCondition {
                condition_type: func_call.name.clone(),
                args: func_call
                    .args
                    .iter()
                    .map(|arg| match arg {
                        Arg::String(s) => s.clone(),
                        Arg::Number(n) => n.to_string(),
                        Arg::Boolean(b) => b.to_string(),
                        Arg::Identifier(id) => id.clone(),
                        Arg::FuncCall(_) => "nested_call".to_string(),
                    })
                    .collect(),
            }),
            None => None,
        };

        let (next, action, nested_choice) = match &choice_item.target {
            ChoiceDest::Identifier(name, _) => (Some(name.clone()), None, None),
            ChoiceDest::Return => (None, Some("return".to_string()), None),
            ChoiceDest::Break => (None, Some("break".to_string()), None),
            ChoiceDest::NestedChoices(nested_items) => {
                let mut nested_choices = Vec::new();
                for item in nested_items {
                    nested_choices.push(Self::convert_choice_item(item)?);
                }
                (None, None, Some(nested_choices))
            }
        };

        Ok(JsonChoice {
            text: choice_item.text.clone(),
            condition,
            next,
            action,
            choice: nested_choice,
        })
    }
}
