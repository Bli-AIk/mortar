//! # converters.rs
//!
//! # converters.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This file contains the focused conversion helpers used by the Mortar serializer. It translates
//! conditions, events, timelines, and action-like AST fragments into the JSON-side structures that
//! the `.mortared` format expects.
//!
//! 这个文件包含 Mortar 序列化器使用的集中式转换辅助函数。它负责把条件、事件、时间线以及动作
//! 类 AST 片段转换成 `.mortared` 格式要求的 JSON 侧结构。

use crate::ast::{
    Arg, BranchCase, ComparisonOp, EventDef, FuncCall, IfCondition, TimelineDef, TimelineStmt,
};
use crate::serializer_types::*;

use super::Serializer;

impl Serializer {
    pub(super) fn convert_if_condition(cond: &IfCondition) -> Result<JsonIfCondition, String> {
        match cond {
            IfCondition::Binary(binary) => {
                let op_str = match binary.operator {
                    ComparisonOp::Greater => ">",
                    ComparisonOp::Less => "<",
                    ComparisonOp::GreaterEqual => ">=",
                    ComparisonOp::LessEqual => "<=",
                    ComparisonOp::Equal => "==",
                    ComparisonOp::NotEqual => "!=",
                    ComparisonOp::And => "&&",
                    ComparisonOp::Or => "||",
                };

                Ok(JsonIfCondition {
                    cond_type: "binary".to_string(),
                    operator: Some(op_str.to_string()),
                    left: Some(Box::new(Self::convert_if_condition(&binary.left)?)),
                    right: Some(Box::new(Self::convert_if_condition(&binary.right)?)),
                    operand: None,
                    value: None,
                })
            }
            IfCondition::Unary(unary) => Ok(JsonIfCondition {
                cond_type: "unary".to_string(),
                operator: Some("!".to_string()),
                left: None,
                right: None,
                operand: Some(Box::new(Self::convert_if_condition(&unary.operand)?)),
                value: None,
            }),
            IfCondition::Identifier(name) => Ok(JsonIfCondition {
                cond_type: "identifier".to_string(),
                operator: None,
                left: None,
                right: None,
                operand: None,
                value: Some(name.clone()),
            }),
            IfCondition::EnumMember(enum_name, member) => Ok(JsonIfCondition {
                cond_type: "enum_member".to_string(),
                operator: None,
                left: None,
                right: None,
                operand: None,
                value: Some(format!("{}.{}", enum_name, member)),
            }),
            IfCondition::Literal(val) => Ok(JsonIfCondition {
                cond_type: "literal".to_string(),
                operator: None,
                left: None,
                right: None,
                operand: None,
                value: Some(val.to_string()),
            }),
            IfCondition::FuncCall(func_call) => {
                let args_part = if func_call.args.is_empty() {
                    None
                } else {
                    Some(Box::new(JsonIfCondition {
                        cond_type: "literal".to_string(),
                        operator: None,
                        left: None,
                        right: None,
                        operand: None,
                        value: Some(
                            func_call
                                .args
                                .iter()
                                .map(Self::convert_arg_to_string)
                                .collect::<Vec<_>>()
                                .join(" "),
                        ),
                    }))
                };

                Ok(JsonIfCondition {
                    cond_type: "func_call".to_string(),
                    operator: None,
                    left: None,
                    right: args_part,
                    operand: Some(Box::new(JsonIfCondition {
                        cond_type: "identifier".to_string(),
                        operator: None,
                        left: None,
                        right: None,
                        operand: None,
                        value: Some(func_call.name.clone()),
                    })),
                    value: None,
                })
            }
        }
    }

    pub(super) fn convert_branch_case(case: &BranchCase) -> serde_json::Value {
        let events = case.events.as_ref().and_then(|e| {
            e.iter()
                .map(Self::convert_event)
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });
        serde_json::json!({
            "condition": case.condition,
            "text": case.text,
            "events": events
        })
    }

    pub(super) fn convert_func_call_to_action(func_call: &FuncCall) -> Result<JsonAction, String> {
        let mut args = Vec::new();

        for arg in &func_call.args {
            match arg {
                Arg::String(s) => args.push(s.clone()),
                Arg::Number(n) => args.push(n.to_string()),
                Arg::Boolean(b) => args.push(b.to_string()),
                Arg::Identifier(id) => args.push(id.clone()),
                Arg::FuncCall(_) => {
                    return Err(
                        "Nested function calls in arguments not supported in JSON output"
                            .to_string(),
                    );
                }
            }
        }

        Ok(JsonAction {
            action_type: func_call.name.clone(),
            args,
        })
    }

    fn format_arg(arg: &Arg) -> String {
        match arg {
            Arg::String(s) => format!("\"{}\"", s),
            Arg::Number(n) => n.to_string(),
            Arg::Boolean(b) => b.to_string(),
            Arg::Identifier(id) => id.clone(),
            Arg::FuncCall(fc) => format!("{}()", fc.name),
        }
    }

    pub(super) fn convert_event_def(event_def: &EventDef) -> JsonEventDef {
        JsonEventDef {
            name: event_def.name.clone(),
            index: event_def.index,
            action: JsonAction {
                action_type: event_def.action.call.name.clone(),
                args: event_def
                    .action
                    .call
                    .args
                    .iter()
                    .map(Self::format_arg)
                    .collect(),
            },
            duration: event_def.duration,
        }
    }

    pub(super) fn convert_timeline_def(timeline_def: &TimelineDef) -> JsonTimelineDef {
        let statements = timeline_def
            .body
            .iter()
            .map(|stmt| match stmt {
                TimelineStmt::Run(run_stmt) => JsonTimelineStmt {
                    stmt_type: "run".to_string(),
                    event_name: Some(run_stmt.event_name.clone()),
                    args: run_stmt.args.iter().map(Self::format_arg).collect(),
                    duration: None,
                    ignore_duration: run_stmt.ignore_duration,
                },
                TimelineStmt::Wait(duration) => JsonTimelineStmt {
                    stmt_type: "wait".to_string(),
                    event_name: None,
                    args: Vec::new(),
                    duration: Some(*duration),
                    ignore_duration: false,
                },
            })
            .collect();

        JsonTimelineDef {
            name: timeline_def.name.clone(),
            statements,
        }
    }
}
