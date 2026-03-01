//! # serializer_types.rs
//!
//! # serializer_types.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! JSON type definitions used by the serializer for the compiled `.mortared` format.
//!
//! 序列化器使用的 JSON 类型定义，用于编译后的 `.mortared` 格式。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MortaredOutput {
    pub(crate) metadata: Metadata,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) variables: Vec<JsonVariable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) constants: Vec<JsonConstant>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) enums: Vec<JsonEnum>,
    pub(crate) nodes: Vec<JsonNode>,
    pub(crate) functions: Vec<JsonFunction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) events: Vec<JsonEventDef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) timelines: Vec<JsonTimelineDef>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Metadata {
    pub(crate) version: String,
    pub(crate) generated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ContentItem {
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
pub(crate) struct JsonIndexOverride {
    #[serde(rename = "type")]
    pub(crate) override_type: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct JsonNode {
    pub(crate) name: String,
    pub(crate) content: Vec<ContentItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) branches: Option<Vec<JsonBranchDef>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) variables: Vec<JsonVariable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct JsonBranchDef {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enum_type: Option<String>,
    pub(crate) cases: Vec<JsonBranchCase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct JsonStatement {
    #[serde(rename = "type")]
    pub(crate) stmt_type: String, // "assignment"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) var_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct JsonIfCondition {
    #[serde(rename = "type")]
    pub(crate) cond_type: String, // "binary", "unary", "identifier", "literal"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) left: Option<Box<JsonIfCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) right: Option<Box<JsonIfCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) operand: Option<Box<JsonIfCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct JsonStringPart {
    #[serde(rename = "type")]
    pub(crate) part_type: String, // "text", "expression", or "branch"
    pub(crate) content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) function_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enum_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) branches: Option<Vec<JsonBranchCase>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct JsonBranchCase {
    pub(crate) condition: String,
    pub(crate) text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) events: Option<Vec<JsonEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct JsonEvent {
    pub(crate) index: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) index_variable: Option<String>, // Variable name for runtime resolution
    pub(crate) actions: Vec<JsonAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct JsonAction {
    #[serde(rename = "type")]
    pub(crate) action_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct JsonChoice {
    pub(crate) text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) condition: Option<JsonCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) choice: Option<Vec<JsonChoice>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct JsonCondition {
    #[serde(rename = "type")]
    pub(crate) condition_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) args: Vec<String>,
}

pub(crate) fn is_false(v: &bool) -> bool {
    !*v
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonFunction {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) params: Vec<JsonParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "return")]
    pub(crate) return_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonParam {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) param_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct JsonVariable {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) var_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonConstant {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) const_type: String,
    pub(crate) value: serde_json::Value,
    pub(crate) public: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonEnum {
    pub(crate) name: String,
    pub(crate) variants: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonEventDef {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<f64>,
    pub(crate) action: JsonAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonTimelineDef {
    pub(crate) name: String,
    pub(crate) statements: Vec<JsonTimelineStmt>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonTimelineStmt {
    #[serde(rename = "type")]
    pub(crate) stmt_type: String, // "run" or "wait"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) event_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<f64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub(crate) ignore_duration: bool,
}
