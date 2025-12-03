//! # serializer.rs
//!
//! # serializer.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Handles the serialization of the parsed AST into the compiled JSON format (`.mortared`).
//!
//! 处理将解析后的 AST 序列化为编译后的 JSON 格式 (`.mortared`)。
//!
//! This transformation converts the rich AST into a flatter, runtime-optimized JSON structure.
//!
//! 此转换将丰富的 AST 转换为更扁平、针对运行时优化的 JSON 结构。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! This file contains the `Serializer` struct and logic for mapping AST types to JSON schema types.
//!
//! 此文件包含 `Serializer` 结构体以及将 AST 类型映射到 JSON 模式类型的逻辑。

use crate::Language;
use crate::ast::{
    Arg, AssignValue, BranchDef, ChoiceDest, ChoiceItem, ComparisonOp, Condition, ConstDecl,
    EnumDef, Event, EventDef, FuncCall, FunctionDecl, IfCondition, IfElseStmt, IndexOverride,
    InterpolatedString, NodeDef, NodeJump, NodeStmt, Program, StringPart, TimelineDef,
    TimelineStmt, TopLevel, VarDecl, VarValue, WithEventItem, WithEventsStmt,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

fn get_text(key: &str, language: Language) -> &'static str {
    match (key, language) {
        ("generated", Language::English) => "Generated:",
        ("generated", Language::Chinese) => "生成文件:",
        _ => "",
    }
}

#[derive(Serialize, Deserialize)]
pub struct MortaredOutput {
    metadata: Metadata,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    variables: Vec<JsonVariable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    constants: Vec<JsonConstant>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    enums: Vec<JsonEnum>,
    nodes: Vec<JsonNode>,
    functions: Vec<JsonFunction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    events: Vec<JsonEventDef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    timelines: Vec<JsonTimelineDef>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    version: String,
    generated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum ContentItem {
    Text {
        value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        interpolated_parts: Option<Vec<JsonStringPart>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        condition: Option<JsonIfCondition>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default)]
        pre_statements: Vec<JsonStatement>,
        #[serde(skip_serializing_if = "Option::is_none")]
        events: Option<Vec<JsonEvent>>,
    },
    RunEvent {
        name: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        index_override: Option<JsonIndexOverride>,
        #[serde(skip_serializing_if = "is_false", default)]
        ignore_duration: bool,
    },
    RunTimeline {
        name: String,
    },
    Choice {
        options: Vec<JsonChoice>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonIndexOverride {
    #[serde(rename = "type")]
    override_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonNode {
    name: String,
    content: Vec<ContentItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<JsonBranchDef>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    variables: Vec<JsonVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonBranchDef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_type: Option<String>,
    cases: Vec<JsonBranchCase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonStatement {
    #[serde(rename = "type")]
    stmt_type: String, // "assignment"
    #[serde(skip_serializing_if = "Option::is_none")]
    var_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonIfCondition {
    #[serde(rename = "type")]
    cond_type: String, // "binary", "unary", "identifier", "literal"
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<Box<JsonIfCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<Box<JsonIfCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operand: Option<Box<JsonIfCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonStringPart {
    #[serde(rename = "type")]
    part_type: String, // "text", "expression", or "branch"
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Vec<JsonBranchCase>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonBranchCase {
    condition: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Vec<JsonEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonEvent {
    index: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_variable: Option<String>, // Variable name for runtime resolution
    actions: Vec<JsonAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct JsonAction {
    #[serde(rename = "type")]
    action_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonChoice {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<JsonCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choice: Option<Vec<JsonChoice>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonCondition {
    #[serde(rename = "type")]
    condition_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
}

fn is_false(v: &bool) -> bool {
    !*v
}

#[derive(Serialize, Deserialize)]
struct JsonFunction {
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<JsonParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "return")]
    return_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonParam {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonVariable {
    name: String,
    #[serde(rename = "type")]
    var_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct JsonConstant {
    name: String,
    #[serde(rename = "type")]
    const_type: String,
    value: serde_json::Value,
    public: bool,
}

#[derive(Serialize, Deserialize)]
struct JsonEnum {
    name: String,
    variants: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonEventDef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<f64>,
    action: JsonAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct JsonTimelineDef {
    name: String,
    statements: Vec<JsonTimelineStmt>,
}

#[derive(Serialize, Deserialize)]
struct JsonTimelineStmt {
    #[serde(rename = "type")]
    stmt_type: String, // "run" or "wait"
    #[serde(skip_serializing_if = "Option::is_none")]
    event_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[serde(skip_serializing_if = "is_false", default)]
    ignore_duration: bool,
}

pub struct Serializer;

impl Serializer {
    pub fn serialize_to_json(program: &Program, pretty: bool) -> Result<String, String> {
        let mortared = Self::convert_program_to_mortared(program)?;
        if pretty {
            serde_json::to_string_pretty(&mortared)
                .map_err(|e| format!("Serialization error: {}", e))
        } else {
            serde_json::to_string(&mortared).map_err(|e| format!("Serialization error: {}", e))
        }
    }

    pub fn save_to_file(program: &Program, input_path: &str, pretty: bool) -> Result<(), String> {
        Self::save_to_file_with_language(program, input_path, pretty, Language::English)
    }

    pub fn save_to_file_with_language(
        program: &Program,
        input_path: &str,
        pretty: bool,
        language: Language,
    ) -> Result<(), String> {
        let input_path = Path::new(input_path);
        let json_content = Self::serialize_to_json(program, pretty)?;

        let output_path = input_path.with_extension("mortared");
        std::fs::write(&output_path, json_content)
            .map_err(|e| format!("Failed to write file {}: {}", output_path.display(), e))?;

        println!(
            "{} {}",
            get_text("generated", language),
            output_path.display()
        );
        Ok(())
    }

    fn convert_program_to_mortared(program: &Program) -> Result<MortaredOutput, String> {
        let metadata = Metadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            generated_at: Utc::now(),
        };

        let mut variables = Vec::new();
        let mut constants = Vec::new();
        let mut enums = Vec::new();
        let mut nodes = Vec::new();
        let mut functions = Vec::new();
        let mut events = Vec::new();
        let mut timelines = Vec::new();

        // Build event definitions map for reference resolution
        let mut event_map = std::collections::HashMap::new();
        for top_level in &program.body {
            if let TopLevel::EventDef(event_def) = top_level {
                event_map.insert(event_def.name.clone(), event_def);
            }
        }

        for top_level in &program.body {
            match top_level {
                TopLevel::NodeDef(node_def) => {
                    nodes.push(Self::convert_node_def(node_def, &event_map)?);
                }
                TopLevel::FunctionDecl(func_decl) => {
                    functions.push(Self::convert_function_decl(func_decl));
                }
                TopLevel::VarDecl(var_decl) => {
                    variables.push(Self::convert_var_decl(var_decl));
                }
                TopLevel::ConstDecl(const_decl) => {
                    constants.push(Self::convert_const_decl(const_decl));
                }
                TopLevel::EnumDef(enum_def) => {
                    enums.push(Self::convert_enum_def(enum_def));
                }
                TopLevel::EventDef(event_def) => {
                    events.push(Self::convert_event_def(event_def));
                }
                TopLevel::TimelineDef(timeline_def) => {
                    timelines.push(Self::convert_timeline_def(timeline_def));
                }
            }
        }

        Ok(MortaredOutput {
            metadata,
            variables,
            constants,
            enums,
            nodes,
            functions,
            events,
            timelines,
        })
    }

    fn convert_node_def(
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
                    let mut events = Vec::new();
                    if let Some(NodeStmt::WithEvents(with_events)) = body_iter.peek() {
                        Self::process_with_events(with_events, &mut events, event_map)?;
                        body_iter.next(); // Consume the WithEvents statement
                    }
                    content.push(ContentItem::Text {
                        value: text.clone(),
                        interpolated_parts: None,
                        condition: None,
                        pre_statements: std::mem::take(&mut pending_statements),
                        events: if events.is_empty() {
                            None
                        } else {
                            Some(events)
                        },
                    });
                }
                NodeStmt::InterpolatedText(interpolated) => {
                    let (rendered_text, parts) = Self::convert_interpolated_string(interpolated)?;
                    let mut events = Vec::new();
                    if let Some(NodeStmt::WithEvents(with_events)) = body_iter.peek() {
                        Self::process_with_events(with_events, &mut events, event_map)?;
                        body_iter.next(); // Consume the WithEvents statement
                    }
                    content.push(ContentItem::Text {
                        value: rendered_text,
                        interpolated_parts: Some(parts),
                        condition: None,
                        pre_statements: std::mem::take(&mut pending_statements),
                        events: if events.is_empty() {
                            None
                        } else {
                            Some(events)
                        },
                    });
                }
                NodeStmt::Run(run_stmt) => {
                    // In a Node, a "run" statement can be for an event or a timeline.
                    // We need to check what the name refers to.
                    // For now, let's assume all `run` in NodeStmts are events as per parser constraints.
                    // If timelines in nodes are supported, this will need timeline definitions passed in.

                    let args: Vec<String> = run_stmt
                        .args
                        .iter()
                        .map(|arg| match arg {
                            Arg::String(s) => format!("\"{}\"", s),
                            Arg::Number(n) => n.to_string(),
                            Arg::Boolean(b) => b.to_string(),
                            Arg::Identifier(id) => id.clone(),
                            Arg::FuncCall(func_call) => {
                                format!("{}(...)", func_call.name)
                            }
                        })
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
                    let mut json_choices = Vec::new();
                    for item in choice_items {
                        json_choices.push(Self::convert_choice_item(item)?);
                    }
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
                    let value_str = match &assignment.value {
                        AssignValue::EnumMember(enum_name, member) => {
                            format!("{}.{}", enum_name, member)
                        }
                        AssignValue::Identifier(id) => id.clone(),
                        AssignValue::Number(n) => n.to_string(),
                        AssignValue::Boolean(b) => b.to_string(),
                        AssignValue::String(s) => s.clone(),
                    };

                    pending_statements.push(JsonStatement {
                        stmt_type: "assignment".to_string(),
                        var_name: Some(assignment.var_name.clone()),
                        value: Some(value_str),
                    });
                }
                // WithEvents is handled by peeking, so we shouldn't encounter it here directly.
                NodeStmt::WithEvents(_) => {
                    // This should be handled by peeking in Text/InterpolatedText.
                    // If we get here, it means a `with events:` block is not preceded by text.
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
                            .map(|e| Self::convert_event(e))
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
            match item {
                WithEventItem::InlineEvent(event) => {
                    events.push(Self::convert_event(event)?);
                }
                WithEventItem::EventRef(name, _span) => {
                    if let Some(event_def) = event_map.get(name) {
                        let event = Event {
                            index: event_def.index.unwrap_or(0.0),
                            action: event_def.action.clone(),
                        };
                        events.push(Self::convert_event(&event)?);
                    } else {
                        return Err(format!("Event '{}' not found", name));
                    }
                }
                WithEventItem::EventRefWithOverride(name, _span, override_val) => {
                    if let Some(event_def) = event_map.get(name) {
                        let (index, index_variable) = match override_val {
                            IndexOverride::Value(v) => (*v, None),
                            IndexOverride::Variable(var_name) => (0.0, Some(var_name.clone())),
                        };

                        let mut actions =
                            vec![Self::convert_func_call_to_action(&event_def.action.call)?];
                        for chain_call in &event_def.action.chains {
                            actions.push(Self::convert_func_call_to_action(chain_call)?);
                        }

                        events.push(JsonEvent {
                            index,
                            index_variable,
                            actions,
                        });
                    } else {
                        return Err(format!("Event '{}' not found", name));
                    }
                }
                WithEventItem::EventList(_) => {
                    // TODO: Handle nested event lists if needed
                }
            }
        }
        Ok(())
    }

    fn process_if_else_to_content(
        if_else: &IfElseStmt,
        content: &mut Vec<ContentItem>,
    ) -> Result<(), String> {
        let condition_json = Self::convert_if_condition(&if_else.condition)?;

        // Process 'then' body
        Self::process_conditional_body_to_content(
            &if_else.then_body,
            Some(condition_json.clone()),
            content,
        )?;

        // Process 'else' body
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
                NodeStmt::Assignment(assignment) => {
                    let value_str = match &assignment.value {
                        AssignValue::EnumMember(enum_name, member) => {
                            format!("{}.{}", enum_name, member)
                        }
                        AssignValue::Identifier(id) => id.clone(),
                        AssignValue::Number(n) => n.to_string(),
                        AssignValue::Boolean(b) => b.to_string(),
                        AssignValue::String(s) => s.clone(),
                    };
                    pending_stmts.push(JsonStatement {
                        stmt_type: "assignment".to_string(),
                        var_name: Some(assignment.var_name.clone()),
                        value: Some(value_str),
                    });
                }
                NodeStmt::IfElse(nested_if) => {
                    let mut nested_content = Vec::new();
                    Self::process_if_else_to_content(nested_if, &mut nested_content)?;

                    for item in nested_content {
                        if let ContentItem::Text {
                            value,
                            interpolated_parts,
                            condition: nested_cond,
                            pre_statements,
                            events,
                        } = item
                        {
                            let combined_cond = if let Some(current_cond) = &condition {
                                if let Some(inner_cond) = nested_cond {
                                    Some(JsonIfCondition {
                                        cond_type: "binary".to_string(),
                                        operator: Some("&&".to_string()),
                                        left: Some(Box::new(current_cond.clone())),
                                        right: Some(Box::new(inner_cond)),
                                        operand: None,
                                        value: None,
                                    })
                                } else {
                                    Some(current_cond.clone())
                                }
                            } else {
                                nested_cond
                            };

                            content.push(ContentItem::Text {
                                value,
                                interpolated_parts,
                                condition: combined_cond,
                                pre_statements,
                                events,
                            });
                        } else {
                            // Non-text items from nested blocks are pushed directly.
                            // Their execution is implicitly conditional on the client side.
                            content.push(item);
                        }
                    }
                }
                _ => {} // Other statements like Run, Choice, etc., are not valid inside if/else text blocks
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

    fn convert_if_condition(cond: &IfCondition) -> Result<JsonIfCondition, String> {
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
        }
    }

    fn convert_event(event: &Event) -> Result<JsonEvent, String> {
        let mut actions = vec![Self::convert_func_call_to_action(&event.action.call)?];

        // Add chained actions
        for chain_call in &event.action.chains {
            actions.push(Self::convert_func_call_to_action(chain_call)?);
        }

        Ok(JsonEvent {
            index: event.index,
            index_variable: None, // Default to None for regular events
            actions,
        })
    }

    fn convert_func_call_to_action(func_call: &FuncCall) -> Result<JsonAction, String> {
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
                        Arg::FuncCall(_) => "nested_call".to_string(), // Simplified
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

    fn convert_function_decl(func_decl: &FunctionDecl) -> JsonFunction {
        let params = func_decl
            .params
            .iter()
            .map(|param| JsonParam {
                name: param.name.clone(),
                param_type: param.type_name.clone(),
            })
            .collect();

        JsonFunction {
            name: func_decl.name.clone(),
            params,
            return_type: func_decl.return_type.clone(),
        }
    }

    fn convert_var_decl(var_decl: &VarDecl) -> JsonVariable {
        JsonVariable {
            name: var_decl.name.clone(),
            var_type: var_decl.type_name.clone(),
            value: var_decl.value.as_ref().map(Self::convert_var_value),
        }
    }

    fn convert_const_decl(const_decl: &ConstDecl) -> JsonConstant {
        JsonConstant {
            name: const_decl.name.clone(),
            const_type: const_decl.type_name.clone(),
            value: Self::convert_var_value(&const_decl.value),
            public: const_decl.is_public,
        }
    }

    fn convert_enum_def(enum_def: &EnumDef) -> JsonEnum {
        JsonEnum {
            name: enum_def.name.clone(),
            variants: enum_def.variants.clone(),
        }
    }

    fn convert_var_value(value: &VarValue) -> serde_json::Value {
        match value {
            VarValue::String(s) => serde_json::Value::String(s.clone()),
            VarValue::Number(n) => serde_json::json!(n),
            VarValue::Boolean(b) => serde_json::Value::Bool(*b),
            VarValue::EnumMember(enum_name, member) => {
                serde_json::Value::String(format!("{}.{}", enum_name, member))
            }
            VarValue::Branch(branch_value) => {
                let cases: Vec<_> = branch_value
                    .cases
                    .iter()
                    .map(|case| {
                        let events = case.events.as_ref().and_then(|events| {
                            let converted: Result<Vec<_>, _> =
                                events.iter().map(|e| Self::convert_event(e)).collect();
                            converted.ok()
                        });

                        serde_json::json!({
                            "condition": case.condition,
                            "text": case.text,
                            "events": events
                        })
                    })
                    .collect();

                serde_json::json!({
                    "enum_type": branch_value.enum_type,
                    "cases": cases
                })
            }
        }
    }

    fn convert_interpolated_string(
        interpolated: &InterpolatedString,
    ) -> Result<(String, Vec<JsonStringPart>), String> {
        let mut rendered_text = String::new();
        let mut parts = Vec::new();

        for part in &interpolated.parts {
            match part {
                StringPart::Text(text) => {
                    rendered_text.push_str(text);
                    parts.push(JsonStringPart {
                        part_type: "text".to_string(),
                        content: text.clone(),
                        function_name: None,
                        args: Vec::new(),
                        enum_type: None,
                        branches: None,
                    });
                }
                StringPart::Expression(func_call) => {
                    // For rendering, we'll use a placeholder
                    let placeholder = format!("{{{}}}", func_call.name);
                    rendered_text.push_str(&placeholder);

                    // Convert arguments to strings
                    let args: Vec<String> = func_call
                        .args
                        .iter()
                        .map(|arg| {
                            match arg {
                                Arg::String(s) => format!("\"{}\"", s),
                                Arg::Number(n) => n.to_string(),
                                Arg::Boolean(b) => b.to_string(),
                                Arg::Identifier(id) => id.clone(),
                                Arg::FuncCall(nested) => format!("{}()", nested.name), // Simplified
                            }
                        })
                        .collect();

                    parts.push(JsonStringPart {
                        part_type: "expression".to_string(),
                        content: placeholder.clone(),
                        function_name: Some(func_call.name.clone()),
                        args,
                        enum_type: None,
                        branches: None,
                    });
                }
                StringPart::Placeholder(name) => {
                    // Placeholder will be resolved by branch definitions
                    let placeholder = format!("{{{}}}", name);
                    rendered_text.push_str(&placeholder);

                    parts.push(JsonStringPart {
                        part_type: "placeholder".to_string(),
                        content: placeholder,
                        function_name: None,
                        args: Vec::new(),
                        enum_type: None,
                        branches: None,
                    });
                }
            }
        }

        Ok((rendered_text, parts))
    }

    fn convert_event_def(event_def: &EventDef) -> JsonEventDef {
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
                    .map(|arg| match arg {
                        Arg::String(s) => format!("\"{}\"", s),
                        Arg::Number(n) => n.to_string(),
                        Arg::Boolean(b) => b.to_string(),
                        Arg::Identifier(id) => id.clone(),
                        Arg::FuncCall(fc) => format!("{}()", fc.name),
                    })
                    .collect(),
            },
            duration: event_def.duration,
        }
    }

    fn convert_timeline_def(timeline_def: &TimelineDef) -> JsonTimelineDef {
        let statements = timeline_def
            .body
            .iter()
            .map(|stmt| match stmt {
                TimelineStmt::Run(run_stmt) => JsonTimelineStmt {
                    stmt_type: "run".to_string(),
                    event_name: Some(run_stmt.event_name.clone()),
                    args: run_stmt
                        .args
                        .iter()
                        .map(|arg| match arg {
                            Arg::String(s) => format!("\"{}\"", s),
                            Arg::Number(n) => n.to_string(),
                            Arg::Boolean(b) => b.to_string(),
                            Arg::Identifier(id) => id.clone(),
                            Arg::FuncCall(fc) => format!("{}()", fc.name),
                        })
                        .collect(),
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
